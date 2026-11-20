use super::SimdMask32x4;

#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use core::arch::aarch64::*;

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use core::arch::x86::*;

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use core::arch::x86_64::*;

#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use core::arch::wasm32::*;

#[cfg(all(
    target_arch = "wasm64",
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use core::arch::wasm64::*;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
use core::simd::{f32x4, num::SimdFloat};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub(crate) struct Align16F32x4(pub(crate) [f32; 4]);

#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawF32x4 = float32x4_t;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawF32x4 = __m128;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawF32x4 = v128;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
type RawF32x4 = f32x4;

#[cfg(any(
    feature = "scalar-math",
    not(any(
        feature = "core-simd",
        target_arch = "aarch64",
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2"
        ),
        all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128"
        ),
    ))
))]
type RawF32x4 = Align16F32x4;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct SimdF32x4(pub(crate) RawF32x4);

const _: () =
    assert!(core::mem::size_of::<SimdF32x4>() == 16 && core::mem::align_of::<SimdF32x4>() == 16);

#[cfg(feature = "bytemuck")]
#[expect(
    unsafe_code,
    reason = "SimdF32x4 is 16 bytes with no padding and all bit patterns are valid"
)]
unsafe impl bytemuck::Zeroable for SimdF32x4 {}

#[cfg(feature = "bytemuck")]
#[expect(
    unsafe_code,
    reason = "SimdF32x4 is 16 bytes with no padding and all bit patterns are valid"
)]
unsafe impl bytemuck::Pod for SimdF32x4 {}

#[repr(C)]
union F32x4Union {
    a: [f32; 4],
    u: [u32; 4],
    v: SimdF32x4,
}

impl core::fmt::Debug for SimdF32x4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SimdF32x4").field(&self.to_array()).finish()
    }
}

impl SimdF32x4 {
    #[inline(always)]
    pub(crate) const fn from_array(a: [f32; 4]) -> Self {
        #[expect(
            unsafe_code,
            reason = "SimdF32x4 and [f32; 4] have identical 16-byte size and representation"
        )]
        unsafe {
            F32x4Union { a }.v
        }
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f32; 4] {
        #[expect(
            unsafe_code,
            reason = "SimdF32x4 and [f32; 4] have identical 16-byte size and all bit patterns are valid f32"
        )]
        unsafe {
            F32x4Union { v: self }.a
        }
    }

    #[inline(always)]
    pub(crate) const fn from_bits(u: [u32; 4]) -> Self {
        #[expect(
            unsafe_code,
            reason = "SimdF32x4 and [u32; 4] have identical 16-byte size"
        )]
        unsafe {
            F32x4Union { u }.v
        }
    }

    #[inline(always)]
    pub(crate) const fn to_bits(self) -> [u32; 4] {
        #[expect(
            unsafe_code,
            reason = "SimdF32x4 and [u32; 4] have identical 16-byte size"
        )]
        unsafe {
            F32x4Union { v: self }.u
        }
    }

    #[inline(always)]
    pub(crate) const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::from_array([x, y, z, w])
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f32) -> Self {
        Self::from_array([v, v, v, v])
    }

    #[inline(always)]
    pub(crate) fn swizzle<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Self {
        let a = self.to_array();
        Self::from_array([a[X], a[Y], a[Z], a[W]])
    }

    #[inline(always)]
    pub(crate) fn add(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vaddq_f32 is always available on aarch64")]
            unsafe {
                Self(vaddq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_add_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_add_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_add(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 + rhs.0)
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            let b = rhs.0.0;
            Self(Align16F32x4([
                a[0] + b[0],
                a[1] + b[1],
                a[2] + b[2],
                a[3] + b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn sub(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vsubq_f32 is always available on aarch64")]
            unsafe {
                Self(vsubq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_sub_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_sub_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_sub(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 - rhs.0)
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            let b = rhs.0.0;
            Self(Align16F32x4([
                a[0] - b[0],
                a[1] - b[1],
                a[2] - b[2],
                a[3] - b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn mul(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vmulq_f32 is always available on aarch64")]
            unsafe {
                Self(vmulq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_mul_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_mul_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_mul(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 * rhs.0)
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            let b = rhs.0.0;
            Self(Align16F32x4([
                a[0] * b[0],
                a[1] * b[1],
                a[2] * b[2],
                a[3] * b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn div(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vdivq_f32 is always available on aarch64")]
            unsafe {
                Self(vdivq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_div_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_div_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_div(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 / rhs.0)
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            let b = rhs.0.0;
            Self(Align16F32x4([
                a[0] / b[0],
                a[1] / b[1],
                a[2] / b[2],
                a[3] / b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn rem(self, rhs: Self) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        Self::from_array([a[0] % b[0], a[1] % b[1], a[2] % b[2], a[3] % b[3]])
    }

    #[inline(always)]
    pub(crate) fn neg(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vnegq_f32 is always available on aarch64")]
            unsafe {
                Self(vnegq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_xor_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_xor_ps(self.0, Self::from_bits([0x8000_0000; 4]).0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_neg(self.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(-self.0)
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            Self(Align16F32x4([-a[0], -a[1], -a[2], -a[3]]))
        }
    }

    #[inline(always)]
    pub(crate) fn abs(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vabsq_f32 is always available on aarch64")]
            unsafe {
                Self(vabsq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_and_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_and_ps(self.0, Self::from_bits([0x7fff_ffff; 4]).0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_abs(self.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0.abs())
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let u = self.to_bits();
            Self::from_bits([
                u[0] & 0x7fff_ffff,
                u[1] & 0x7fff_ffff,
                u[2] & 0x7fff_ffff,
                u[3] & 0x7fff_ffff,
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vfmaq_f32 is always available on aarch64")]
            unsafe {
                Self(vfmaq_f32(c.0, self.0, b.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            target_feature = "fma",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "FMA _mm_fmadd_ps is guarded by target_feature fma"
            )]
            unsafe {
                Self(_mm_fmadd_ps(self.0, b.0, c.0))
            }
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                target_feature = "fma",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            self.mul(b).add(c)
        }
    }

    #[inline(always)]
    pub(crate) fn fma_exact(self, b: Self, c: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vfmaq_f32 is true IEEE-754 FMA on aarch64")]
            unsafe {
                Self(vfmaq_f32(c.0, self.0, b.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            target_feature = "fma",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "FMA _mm_fmadd_ps is true IEEE-754 FMA")]
            unsafe {
                Self(_mm_fmadd_ps(self.0, b.0, c.0))
            }
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                target_feature = "fma",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let x = self.to_array();
            let y = b.to_array();
            let z = c.to_array();
            Self::from_array([
                crate::math::f32_math::mul_add(x[0], y[0], z[0]),
                crate::math::f32_math::mul_add(x[1], y[1], z[1]),
                crate::math::f32_math::mul_add(x[2], y[2], z[2]),
                crate::math::f32_math::mul_add(x[3], y[3], z[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn min(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "NEON vminnmq_f32 is always available on aarch64"
            )]
            unsafe {
                Self(vminnmq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_min_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_min_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_pmin(rhs.0, self.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0.simd_min(rhs.0))
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            let b = rhs.0.0;
            Self(Align16F32x4([
                a[0].min(b[0]),
                a[1].min(b[1]),
                a[2].min(b[2]),
                a[3].min(b[3]),
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn max(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "NEON vmaxnmq_f32 is always available on aarch64"
            )]
            unsafe {
                Self(vmaxnmq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_max_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_max_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_pmax(rhs.0, self.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0.simd_max(rhs.0))
        }
        #[cfg(any(
            feature = "scalar-math",
            not(any(
                feature = "core-simd",
                target_arch = "aarch64",
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "sse2"
                ),
                all(
                    any(target_arch = "wasm32", target_arch = "wasm64"),
                    target_feature = "simd128"
                ),
            ))
        ))]
        {
            let a = self.0.0;
            let b = rhs.0.0;
            Self(Align16F32x4([
                a[0].max(b[0]),
                a[1].max(b[1]),
                a[2].max(b[2]),
                a[3].max(b[3]),
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vrndmq_f32 is always available on aarch64")]
            unsafe {
                Self(vrndmq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE4.1 _mm_floor_ps is guarded by target_feature sse4.1"
            )]
            unsafe {
                Self(_mm_floor_ps(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_floor(self.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f32_math::floor(a[0]),
                crate::math::f32_math::floor(a[1]),
                crate::math::f32_math::floor(a[2]),
                crate::math::f32_math::floor(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn ceil(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vrndpq_f32 is always available on aarch64")]
            unsafe {
                Self(vrndpq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE4.1 _mm_ceil_ps is guarded by target_feature sse4.1"
            )]
            unsafe {
                Self(_mm_ceil_ps(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_ceil(self.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f32_math::ceil(a[0]),
                crate::math::f32_math::ceil(a[1]),
                crate::math::f32_math::ceil(a[2]),
                crate::math::f32_math::ceil(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn trunc(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vrndq_f32 is always available on aarch64")]
            unsafe {
                Self(vrndq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_trunc(self.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f32_math::trunc(a[0]),
                crate::math::f32_math::trunc(a[1]),
                crate::math::f32_math::trunc(a[2]),
                crate::math::f32_math::trunc(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn round(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vrndaq_f32 is always available on aarch64")]
            unsafe {
                Self(vrndaq_f32(self.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f32_math::round(a[0]),
                crate::math::f32_math::round(a[1]),
                crate::math::f32_math::round(a[2]),
                crate::math::f32_math::round(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn round_even(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vrndnq_f32 is always available on aarch64")]
            unsafe {
                Self(vrndnq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_nearest(self.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f32_math::round_even(a[0]),
                crate::math::f32_math::round_even(a[1]),
                crate::math::f32_math::round_even(a[2]),
                crate::math::f32_math::round_even(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn fract(self) -> Self {
        self.sub(self.floor())
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        let zero = Self::splat(0.0);
        let pos = Self::select(self.cmp_gt(zero), Self::splat(1.0), zero);
        Self::select(self.cmp_lt(zero), Self::splat(-1.0), pos)
    }

    #[inline(always)]
    pub(crate) fn sqrt(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vsqrtq_f32 is always available on aarch64")]
            unsafe {
                Self(vsqrtq_f32(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_sqrt_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_sqrt_ps(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f32x4_sqrt(self.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f32_math::sqrt(a[0]),
                crate::math::f32_math::sqrt(a[1]),
                crate::math::f32_math::sqrt(a[2]),
                crate::math::f32_math::sqrt(a[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(self) -> Self {
        Self::splat(1.0).div(self.sqrt())
    }

    #[inline(always)]
    pub(crate) fn cmp_eq(self, rhs: Self) -> SimdMask32x4 {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vceqq_f32 is always available on aarch64")]
            unsafe {
                SimdMask32x4(vceqq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_cmpeq_ps is guarded by target_feature sse2"
            )]
            unsafe {
                SimdMask32x4(_mm_cmpeq_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            SimdMask32x4(f32x4_eq(self.0, rhs.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            let b = rhs.to_array();
            SimdMask32x4::new(a[0] == b[0], a[1] == b[1], a[2] == b[2], a[3] == b[3])
        }
    }

    #[inline(always)]
    pub(crate) fn cmp_ne(self, rhs: Self) -> SimdMask32x4 {
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_cmpneq_ps is guarded by target_feature sse2"
            )]
            unsafe {
                SimdMask32x4(_mm_cmpneq_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            SimdMask32x4(f32x4_ne(self.0, rhs.0))
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            self.cmp_eq(rhs).not()
        }
    }

    #[inline(always)]
    pub(crate) fn cmp_lt(self, rhs: Self) -> SimdMask32x4 {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vcltq_f32 is always available on aarch64")]
            unsafe {
                SimdMask32x4(vcltq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_cmplt_ps is guarded by target_feature sse2"
            )]
            unsafe {
                SimdMask32x4(_mm_cmplt_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            SimdMask32x4(f32x4_lt(self.0, rhs.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            let b = rhs.to_array();
            SimdMask32x4::new(a[0] < b[0], a[1] < b[1], a[2] < b[2], a[3] < b[3])
        }
    }

    #[inline(always)]
    pub(crate) fn cmp_le(self, rhs: Self) -> SimdMask32x4 {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vcleq_f32 is always available on aarch64")]
            unsafe {
                SimdMask32x4(vcleq_f32(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_cmple_ps is guarded by target_feature sse2"
            )]
            unsafe {
                SimdMask32x4(_mm_cmple_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            SimdMask32x4(f32x4_le(self.0, rhs.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            let b = rhs.to_array();
            SimdMask32x4::new(a[0] <= b[0], a[1] <= b[1], a[2] <= b[2], a[3] <= b[3])
        }
    }

    #[inline(always)]
    pub(crate) fn cmp_gt(self, rhs: Self) -> SimdMask32x4 {
        rhs.cmp_lt(self)
    }

    #[inline(always)]
    pub(crate) fn cmp_ge(self, rhs: Self) -> SimdMask32x4 {
        rhs.cmp_le(self)
    }

    #[inline(always)]
    pub(crate) fn is_nan(self) -> SimdMask32x4 {
        self.cmp_ne(self)
    }

    #[inline(always)]
    pub(crate) fn is_infinite(self) -> SimdMask32x4 {
        self.abs().cmp_eq(Self::splat(f32::INFINITY))
    }

    #[inline(always)]
    pub(crate) fn select(mask: SimdMask32x4, if_true: Self, if_false: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vbslq_f32 is always available on aarch64")]
            unsafe {
                Self(vbslq_f32(mask.0, if_true.0, if_false.0))
            }
        }
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_or_ps/_mm_and_ps/_mm_andnot_ps are guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_or_ps(
                    _mm_and_ps(mask.0, if_true.0),
                    _mm_andnot_ps(mask.0, if_false.0),
                ))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(v128_bitselect(if_true.0, if_false.0, mask.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse2",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(
                any(target_arch = "wasm32", target_arch = "wasm64"),
                target_feature = "simd128",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let m = mask.to_u32_array();
            let t = if_true.to_bits();
            let f = if_false.to_bits();
            Self::from_bits([
                (t[0] & m[0]) | (f[0] & !m[0]),
                (t[1] & m[1]) | (f[1] & !m[1]),
                (t[2] & m[2]) | (f[2] & !m[2]),
                (t[3] & m[3]) | (f[3] & !m[3]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn shuffle<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
        rhs: Self,
    ) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        let ab = [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]];
        Self::from_array([ab[X], ab[Y], ab[Z], ab[W]])
    }

    #[inline(always)]
    pub(crate) fn dot3(self, rhs: Self) -> f32 {
        let p = self.mul(rhs).to_array();
        (p[0] + p[1]) + p[2]
    }

    #[inline(always)]
    pub(crate) fn dot3_splat(self, rhs: Self) -> Self {
        let p = self.mul(rhs);
        p.swizzle::<0, 0, 0, 0>()
            .add(p.swizzle::<1, 1, 1, 1>())
            .add(p.swizzle::<2, 2, 2, 2>())
    }

    #[inline(always)]
    pub(crate) fn dot4(self, rhs: Self) -> f32 {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "NEON vmulq_f32 and vaddvq_f32 are available on aarch64"
            )]
            unsafe {
                vaddvq_f32(vmulq_f32(self.0, rhs.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let p = self.mul(rhs).to_array();
            (p[0] + p[2]) + (p[1] + p[3])
        }
    }

    #[inline(always)]
    pub(crate) fn dot4_splat(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "NEON vmulq_f32/vaddvq_f32/vdupq_n_f32 are available on aarch64"
            )]
            unsafe {
                Self(vdupq_n_f32(vaddvq_f32(vmulq_f32(self.0, rhs.0))))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let p = self.mul(rhs);
            let s = p.add(p.swizzle::<2, 3, 0, 1>());
            s.add(s.swizzle::<1, 0, 3, 2>())
        }
    }

    #[inline(always)]
    pub(crate) fn cross3(self, rhs: Self) -> Self {
        let a_yzx = self.swizzle::<1, 2, 0, 3>();
        let b_yzx = rhs.swizzle::<1, 2, 0, 3>();
        self.mul(b_yzx).sub(a_yzx.mul(rhs)).swizzle::<1, 2, 0, 3>()
    }

    #[inline(always)]
    pub(crate) fn quat_mul(self, rhs: Self) -> Self {
        let lw = self.swizzle::<3, 3, 3, 3>();
        let lx = self.swizzle::<0, 0, 0, 0>();
        let ly = self.swizzle::<1, 1, 1, 1>();
        let lz = self.swizzle::<2, 2, 2, 2>();
        let r_wzyx = rhs
            .swizzle::<3, 2, 1, 0>()
            .mul(Self::new(1.0, -1.0, 1.0, -1.0));
        let r_zwxy = rhs
            .swizzle::<2, 3, 0, 1>()
            .mul(Self::new(1.0, 1.0, -1.0, -1.0));
        let r_yxwz = rhs
            .swizzle::<1, 0, 3, 2>()
            .mul(Self::new(-1.0, 1.0, 1.0, -1.0));
        lz.mul_add(r_yxwz, ly.mul_add(r_zwxy, lx.mul_add(r_wzyx, lw.mul(rhs))))
    }

    #[inline]
    pub(crate) fn mat4_determinant(c0: Self, c1: Self, c2: Self, c3: Self) -> f32 {
        let c2_2211 = c2.swizzle::<2, 2, 1, 1>();
        let c2_3332 = c2.swizzle::<3, 3, 3, 2>();
        let c2_1000 = c2.swizzle::<1, 0, 0, 0>();
        let c3_2211 = c3.swizzle::<2, 2, 1, 1>();
        let c3_3332 = c3.swizzle::<3, 3, 3, 2>();
        let c3_1000 = c3.swizzle::<1, 0, 0, 0>();

        let p0 = c2_2211.mul(c3_3332).sub(c3_2211.mul(c2_3332));
        let p1 = c2_1000.mul(c3_3332).sub(c3_1000.mul(c2_3332));
        let p2 = c2_1000.mul(c3_2211).sub(c3_1000.mul(c2_2211));

        let cof0 = c1
            .swizzle::<1, 0, 0, 0>()
            .mul(p0)
            .sub(c1.swizzle::<2, 2, 1, 1>().mul(p1))
            .add(c1.swizzle::<3, 3, 3, 2>().mul(p2))
            .mul(Self::new(1.0, -1.0, 1.0, -1.0));

        c0.dot4(cof0)
    }

    #[inline]
    pub(crate) fn mat4_inverse(c0: Self, c1: Self, c2: Self, c3: Self) -> [Self; 4] {
        let u0 = c2.shuffle::<0, 0, 4, 4>(c1);
        let u1 = c2.shuffle::<1, 1, 5, 5>(c1);
        let u2 = c2.shuffle::<2, 2, 6, 6>(c1);
        let u3 = c2.shuffle::<3, 3, 7, 7>(c1);

        let v0 = c3.shuffle::<0, 0, 0, 4>(c2);
        let v1 = c3.shuffle::<1, 1, 1, 5>(c2);
        let v2 = c3.shuffle::<2, 2, 2, 6>(c2);
        let v3 = c3.shuffle::<3, 3, 3, 7>(c2);

        let fac0 = u2.mul(v3).sub(v2.mul(u3));
        let fac1 = u1.mul(v3).sub(v1.mul(u3));
        let fac2 = u1.mul(v2).sub(v1.mul(u2));
        let fac3 = u0.mul(v3).sub(v0.mul(u3));
        let fac4 = u0.mul(v2).sub(v0.mul(u2));
        let fac5 = u0.mul(v1).sub(v0.mul(u1));

        let vec0 = c1.shuffle::<0, 4, 4, 4>(c0);
        let vec1 = c1.shuffle::<1, 5, 5, 5>(c0);
        let vec2 = c1.shuffle::<2, 6, 6, 6>(c0);
        let vec3 = c1.shuffle::<3, 7, 7, 7>(c0);

        let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
        let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
        let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
        let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

        let sign_a = Self::new(1.0, -1.0, 1.0, -1.0);
        let sign_b = Self::new(-1.0, 1.0, -1.0, 1.0);
        let c0_inv = inv0.mul(sign_a);
        let c1_inv = inv1.mul(sign_b);
        let c2_inv = inv2.mul(sign_a);
        let c3_inv = inv3.mul(sign_b);

        let t01 = c0_inv.shuffle::<0, 4, 0, 4>(c1_inv);
        let t23 = c2_inv.shuffle::<0, 4, 0, 4>(c3_inv);
        let row0 = t01.shuffle::<0, 1, 4, 5>(t23);
        let det_splat = c0.dot4_splat(row0);
        rglm_debug_assert!(det_splat.to_array()[0] != 0.0);
        let inv_det = Self::splat(1.0).div(det_splat);

        [
            c0_inv.mul(inv_det),
            c1_inv.mul(inv_det),
            c2_inv.mul(inv_det),
            c3_inv.mul(inv_det),
        ]
    }
}
