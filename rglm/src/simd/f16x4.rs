use super::SimdF32x4;
use half::f16;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct SimdF16x4(pub(crate) [f16; 4]);

impl SimdF16x4 {
    #[inline(always)]
    pub(crate) const fn from_array(a: [f16; 4]) -> Self {
        Self(a)
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f16; 4] {
        self.0
    }

    #[inline(always)]
    pub(crate) fn to_f32x4(self) -> SimdF32x4 {
        #[cfg(all(target_arch = "aarch64", not(feature = "scalar-math")))]
        {
            #[expect(
                unsafe_code,
                reason = "Baseline ARM64 NEON fcvtl converts 4 IEEE-754 f16 lanes in d-register to float32x4_t"
            )]
            unsafe {
                let raw_u64 = u64::from_ne_bytes([
                    self.0[0].to_ne_bytes()[0],
                    self.0[0].to_ne_bytes()[1],
                    self.0[1].to_ne_bytes()[0],
                    self.0[1].to_ne_bytes()[1],
                    self.0[2].to_ne_bytes()[0],
                    self.0[2].to_ne_bytes()[1],
                    self.0[3].to_ne_bytes()[0],
                    self.0[3].to_ne_bytes()[1],
                ]);
                let out: core::arch::aarch64::float32x4_t;
                core::arch::asm!(
                    "fmov {tmp:d}, {inp}",
                    "fcvtl {out:v}.4s, {tmp:v}.4h",
                    inp = in(reg) raw_u64,
                    tmp = out(vreg) _,
                    out = lateout(vreg) out,
                    options(pure, nomem, nostack),
                );
                #[cfg(not(feature = "core-simd"))]
                {
                    SimdF32x4(out)
                }
                #[cfg(feature = "core-simd")]
                {
                    let arr: [f32; 4] = core::mem::transmute(out);
                    SimdF32x4::from_array(arr)
                }
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "f16c",
            target_feature = "sse2",
            not(feature = "scalar-math")
        ))]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64::*;
            #[expect(
                unsafe_code,
                reason = "F16C _mm_cvtph_ps converts the low 4 f16 lanes to __m128"
            )]
            unsafe {
                let packed = _mm_set_epi16(
                    0,
                    0,
                    0,
                    0,
                    self.0[3].to_bits() as i16,
                    self.0[2].to_bits() as i16,
                    self.0[1].to_bits() as i16,
                    self.0[0].to_bits() as i16,
                );
                let cvt = _mm_cvtph_ps(packed);
                #[cfg(not(feature = "core-simd"))]
                {
                    SimdF32x4(cvt)
                }
                #[cfg(feature = "core-simd")]
                {
                    let arr: [f32; 4] = core::mem::transmute(cvt);
                    SimdF32x4::from_array(arr)
                }
            }
        }
        #[cfg(not(any(
            all(target_arch = "aarch64", not(feature = "scalar-math")),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "f16c",
                target_feature = "sse2",
                not(feature = "scalar-math")
            ),
        )))]
        {
            SimdF32x4::from_array([
                self.0[0].to_f32(),
                self.0[1].to_f32(),
                self.0[2].to_f32(),
                self.0[3].to_f32(),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn from_f32x4(v: SimdF32x4) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "Baseline ARM64 NEON fcvtn converts float32x4_t to 4 IEEE-754 f16 lanes"
            )]
            unsafe {
                let raw_u64: u64;
                core::arch::asm!(
                    "fcvtn {tmp:v}.4h, {inp:v}.4s",
                    "fmov {out}, {tmp:d}",
                    inp = in(vreg) v.0,
                    tmp = out(vreg) _,
                    out = lateout(reg) raw_u64,
                    options(pure, nomem, nostack),
                );
                let b = raw_u64.to_ne_bytes();
                Self([
                    f16::from_ne_bytes([b[0], b[1]]),
                    f16::from_ne_bytes([b[2], b[3]]),
                    f16::from_ne_bytes([b[4], b[5]]),
                    f16::from_ne_bytes([b[6], b[7]]),
                ])
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "f16c",
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64::*;
            #[expect(
                unsafe_code,
                reason = "F16C _mm_cvtps_ph converts __m128 to 4 IEEE-754 round-to-nearest-even f16 lanes"
            )]
            unsafe {
                let ph = _mm_cvtps_ph::<_MM_FROUND_TO_NEAREST_INT>(v.0);
                let mut lanes = [0u16; 8];
                _mm_storeu_si128(lanes.as_mut_ptr().cast::<__m128i>(), ph);
                Self([
                    f16::from_bits(lanes[0]),
                    f16::from_bits(lanes[1]),
                    f16::from_bits(lanes[2]),
                    f16::from_bits(lanes[3]),
                ])
            }
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "f16c",
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = v.to_array();
            Self([
                f16::from_f32(a[0]),
                f16::from_f32(a[1]),
                f16::from_f32(a[2]),
                f16::from_f32(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn add(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "ARMv8.2+ FP16 fadd.4h adds 4 IEEE-754 f16 lanes in register"
            )]
            unsafe {
                let a = self.pack_u64();
                let b = rhs.pack_u64();
                let out: u64;
                core::arch::asm!(
                    "fmov {va:d}, {a}",
                    "fmov {vb:d}, {b}",
                    "fadd {va:v}.4h, {va:v}.4h, {vb:v}.4h",
                    "fmov {out}, {va:d}",
                    a = in(reg) a,
                    b = in(reg) b,
                    va = out(vreg) _,
                    vb = out(vreg) _,
                    out = lateout(reg) out,
                    options(pure, nomem, nostack),
                );
                Self::unpack_u64(out)
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        )))]
        {
            Self::from_f32x4(self.to_f32x4().add(rhs.to_f32x4()))
        }
    }

    #[inline(always)]
    pub(crate) fn sub(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "ARMv8.2+ FP16 fsub.4h subtracts 4 IEEE-754 f16 lanes in register"
            )]
            unsafe {
                let a = self.pack_u64();
                let b = rhs.pack_u64();
                let out: u64;
                core::arch::asm!(
                    "fmov {va:d}, {a}",
                    "fmov {vb:d}, {b}",
                    "fsub {va:v}.4h, {va:v}.4h, {vb:v}.4h",
                    "fmov {out}, {va:d}",
                    a = in(reg) a,
                    b = in(reg) b,
                    va = out(vreg) _,
                    vb = out(vreg) _,
                    out = lateout(reg) out,
                    options(pure, nomem, nostack),
                );
                Self::unpack_u64(out)
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        )))]
        {
            Self::from_f32x4(self.to_f32x4().sub(rhs.to_f32x4()))
        }
    }

    #[inline(always)]
    pub(crate) fn mul(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "ARMv8.2+ FP16 fmul.4h multiplies 4 IEEE-754 f16 lanes in register"
            )]
            unsafe {
                let a = self.pack_u64();
                let b = rhs.pack_u64();
                let out: u64;
                core::arch::asm!(
                    "fmov {va:d}, {a}",
                    "fmov {vb:d}, {b}",
                    "fmul {va:v}.4h, {va:v}.4h, {vb:v}.4h",
                    "fmov {out}, {va:d}",
                    a = in(reg) a,
                    b = in(reg) b,
                    va = out(vreg) _,
                    vb = out(vreg) _,
                    out = lateout(reg) out,
                    options(pure, nomem, nostack),
                );
                Self::unpack_u64(out)
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        )))]
        {
            Self::from_f32x4(self.to_f32x4().mul(rhs.to_f32x4()))
        }
    }

    #[inline(always)]
    pub(crate) fn div(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "ARMv8.2+ FP16 fdiv.4h divides 4 IEEE-754 f16 lanes in register"
            )]
            unsafe {
                let a = self.pack_u64();
                let b = rhs.pack_u64();
                let out: u64;
                core::arch::asm!(
                    "fmov {va:d}, {a}",
                    "fmov {vb:d}, {b}",
                    "fdiv {va:v}.4h, {va:v}.4h, {vb:v}.4h",
                    "fmov {out}, {va:d}",
                    a = in(reg) a,
                    b = in(reg) b,
                    va = out(vreg) _,
                    vb = out(vreg) _,
                    out = lateout(reg) out,
                    options(pure, nomem, nostack),
                );
                Self::unpack_u64(out)
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        )))]
        {
            Self::from_f32x4(self.to_f32x4().div(rhs.to_f32x4()))
        }
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "ARMv8.2+ FP16 fmla.4h performs fused multiply-add on 4 IEEE-754 f16 lanes"
            )]
            unsafe {
                let pa = self.pack_u64();
                let pb = b.pack_u64();
                let pc = c.pack_u64();
                let out: u64;
                core::arch::asm!(
                    "fmov {va:d}, {a}",
                    "fmov {vb:d}, {b}",
                    "fmov {vc:d}, {c}",
                    "fmla {vc:v}.4h, {va:v}.4h, {vb:v}.4h",
                    "fmov {out}, {vc:d}",
                    a = in(reg) pa,
                    b = in(reg) pb,
                    c = in(reg) pc,
                    va = out(vreg) _,
                    vb = out(vreg) _,
                    vc = out(vreg) _,
                    out = lateout(reg) out,
                    options(pure, nomem, nostack),
                );
                Self::unpack_u64(out)
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            target_feature = "fp16",
            not(feature = "scalar-math")
        )))]
        {
            use crate::math::f16_math;
            Self([
                f16_math::mul_add(self.0[0], b.0[0], c.0[0]),
                f16_math::mul_add(self.0[1], b.0[1], c.0[1]),
                f16_math::mul_add(self.0[2], b.0[2], c.0[2]),
                f16_math::mul_add(self.0[3], b.0[3], c.0[3]),
            ])
        }
    }

    #[cfg(all(
        target_arch = "aarch64",
        target_feature = "fp16",
        not(feature = "scalar-math")
    ))]
    #[inline(always)]
    fn pack_u64(self) -> u64 {
        let b0 = self.0[0].to_ne_bytes();
        let b1 = self.0[1].to_ne_bytes();
        let b2 = self.0[2].to_ne_bytes();
        let b3 = self.0[3].to_ne_bytes();
        u64::from_ne_bytes([b0[0], b0[1], b1[0], b1[1], b2[0], b2[1], b3[0], b3[1]])
    }

    #[cfg(all(
        target_arch = "aarch64",
        target_feature = "fp16",
        not(feature = "scalar-math")
    ))]
    #[inline(always)]
    fn unpack_u64(v: u64) -> Self {
        let b = v.to_ne_bytes();
        Self([
            f16::from_ne_bytes([b[0], b[1]]),
            f16::from_ne_bytes([b[2], b[3]]),
            f16::from_ne_bytes([b[4], b[5]]),
            f16::from_ne_bytes([b[6], b[7]]),
        ])
    }
}
