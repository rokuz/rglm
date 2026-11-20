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
use core::simd::u32x4;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub(crate) struct Align16U32x4(pub(crate) [u32; 4]);

#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawMask = uint32x4_t;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawMask = __m128;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawMask = v128;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
type RawMask = u32x4;

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
type RawMask = Align16U32x4;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct SimdMask32x4(pub(crate) RawMask);

const _: () = assert!(
    core::mem::size_of::<SimdMask32x4>() == 16 && core::mem::align_of::<SimdMask32x4>() == 16
);

#[cfg(feature = "bytemuck")]
#[expect(
    unsafe_code,
    reason = "SimdMask32x4 is 16 bytes with no padding and all bit patterns are valid"
)]
unsafe impl bytemuck::Zeroable for SimdMask32x4 {}

#[cfg(feature = "bytemuck")]
#[expect(
    unsafe_code,
    reason = "SimdMask32x4 is 16 bytes with no padding and all bit patterns are valid"
)]
unsafe impl bytemuck::Pod for SimdMask32x4 {}

#[repr(C)]
union MaskUnion {
    a: [u32; 4],
    v: SimdMask32x4,
}

const LANE_MASK: [u32; 2] = [0, 0xffff_ffff];

impl core::fmt::Debug for SimdMask32x4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SimdMask32x4")
            .field(&self.to_bool_array())
            .finish()
    }
}

impl SimdMask32x4 {
    #[inline(always)]
    pub(crate) const fn from_u32_array(a: [u32; 4]) -> Self {
        #[expect(
            unsafe_code,
            reason = "SimdMask32x4 and [u32; 4] have identical 16-byte size and valid bit patterns"
        )]
        unsafe {
            MaskUnion { a }.v
        }
    }

    #[inline(always)]
    pub(crate) const fn to_u32_array(self) -> [u32; 4] {
        #[expect(
            unsafe_code,
            reason = "SimdMask32x4 and [u32; 4] have identical 16-byte size and all bit patterns are valid u32"
        )]
        unsafe {
            MaskUnion { v: self }.a
        }
    }

    #[inline(always)]
    pub(crate) const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self::from_u32_array([
            LANE_MASK[x as usize],
            LANE_MASK[y as usize],
            LANE_MASK[z as usize],
            LANE_MASK[w as usize],
        ])
    }

    #[inline(always)]
    pub(crate) const fn new3(x: bool, y: bool, z: bool) -> Self {
        Self::from_u32_array([
            LANE_MASK[x as usize],
            LANE_MASK[y as usize],
            LANE_MASK[z as usize],
            0,
        ])
    }

    #[inline(always)]
    pub(crate) const fn splat(v: bool) -> Self {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn splat3(v: bool) -> Self {
        Self::new3(v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn to_bool_array(self) -> [bool; 4] {
        let a = self.to_u32_array();
        [
            (a[0] >> 31) != 0,
            (a[1] >> 31) != 0,
            (a[2] >> 31) != 0,
            (a[3] >> 31) != 0,
        ]
    }

    #[inline(always)]
    pub(crate) const fn extract(self, idx: usize) -> bool {
        (self.to_u32_array()[idx] >> 31) != 0
    }

    #[inline(always)]
    pub(crate) const fn set(&mut self, idx: usize, val: bool) {
        let mut a = self.to_u32_array();
        a[idx] = LANE_MASK[val as usize];
        *self = Self::from_u32_array(a);
    }

    #[inline(always)]
    pub(crate) fn bitand(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vandq_u32 is always available on aarch64")]
            unsafe {
                Self(vandq_u32(self.0, rhs.0))
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
                Self(_mm_and_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(v128_and(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 & rhs.0)
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
            Self(Align16U32x4([
                a[0] & b[0],
                a[1] & b[1],
                a[2] & b[2],
                a[3] & b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn bitor(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vorrq_u32 is always available on aarch64")]
            unsafe {
                Self(vorrq_u32(self.0, rhs.0))
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
                reason = "SSE2 _mm_or_ps is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_or_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(v128_or(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 | rhs.0)
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
            Self(Align16U32x4([
                a[0] | b[0],
                a[1] | b[1],
                a[2] | b[2],
                a[3] | b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn bitxor(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON veorq_u32 is always available on aarch64")]
            unsafe {
                Self(veorq_u32(self.0, rhs.0))
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
                Self(_mm_xor_ps(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(v128_xor(self.0, rhs.0))
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0 ^ rhs.0)
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
            Self(Align16U32x4([
                a[0] ^ b[0],
                a[1] ^ b[1],
                a[2] ^ b[2],
                a[3] ^ b[3],
            ]))
        }
    }

    #[inline(always)]
    pub(crate) fn not(self) -> Self {
        self.bitxor(Self::splat(true))
    }

    #[inline(always)]
    pub(crate) fn not3(self) -> Self {
        self.bitxor(Self::splat3(true))
    }

    #[inline(always)]
    pub(crate) fn bitmask4(self) -> u32 {
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse2",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "SSE2 _mm_movemask_ps is guarded by target_feature sse2"
            )]
            unsafe {
                _mm_movemask_ps(self.0) as u32
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            u32x4_bitmask(self.0) as u32
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
            let a = self.to_u32_array();
            (a[0] >> 31) | ((a[1] >> 31) << 1) | ((a[2] >> 31) << 2) | ((a[3] >> 31) << 3)
        }
    }

    #[inline(always)]
    pub(crate) fn bitmask3(self) -> u32 {
        self.bitmask4() & 0x7
    }

    #[inline(always)]
    pub(crate) fn any4(self) -> bool {
        self.bitmask4() != 0
    }

    #[inline(always)]
    pub(crate) fn all4(self) -> bool {
        self.bitmask4() == 0xf
    }

    #[inline(always)]
    pub(crate) fn any3(self) -> bool {
        self.bitmask3() != 0
    }

    #[inline(always)]
    pub(crate) fn all3(self) -> bool {
        self.bitmask3() == 0x7
    }
}
