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
use core::simd::{f64x2, num::SimdFloat};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub(crate) struct Align16F64x2(pub(crate) [f64; 2]);

#[cfg(all(
    target_arch = "aarch64",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawF64x2 = float64x2_t;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawF64x2 = __m128d;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
type RawF64x2 = v128;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
type RawF64x2 = f64x2;

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
type RawF64x2 = Align16F64x2;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct SimdF64x2(pub(crate) RawF64x2);

const _: () =
    assert!(core::mem::size_of::<SimdF64x2>() == 16 && core::mem::align_of::<SimdF64x2>() == 16);

#[cfg(feature = "bytemuck")]
#[expect(
    unsafe_code,
    reason = "SimdF64x2 is 16 bytes with no padding and all bit patterns are valid"
)]
unsafe impl bytemuck::Zeroable for SimdF64x2 {}

#[cfg(feature = "bytemuck")]
#[expect(
    unsafe_code,
    reason = "SimdF64x2 is 16 bytes with no padding and all bit patterns are valid"
)]
unsafe impl bytemuck::Pod for SimdF64x2 {}

#[repr(C)]
union F64x2Union {
    a: [f64; 2],
    u: [u64; 2],
    v: SimdF64x2,
}

impl core::fmt::Debug for SimdF64x2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SimdF64x2").field(&self.to_array()).finish()
    }
}

impl SimdF64x2 {
    #[inline(always)]
    pub(crate) const fn from_array(a: [f64; 2]) -> Self {
        #[expect(
            unsafe_code,
            reason = "SimdF64x2 and [f64; 2] have identical 16-byte size and valid bit patterns"
        )]
        unsafe {
            F64x2Union { a }.v
        }
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f64; 2] {
        #[expect(
            unsafe_code,
            reason = "SimdF64x2 and [f64; 2] have identical 16-byte size and valid bit patterns"
        )]
        unsafe {
            F64x2Union { v: self }.a
        }
    }

    #[inline(always)]
    pub(crate) const fn new(x: f64, y: f64) -> Self {
        Self::from_array([x, y])
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f64) -> Self {
        Self::from_array([v, v])
    }

    #[inline(always)]
    pub(crate) fn add(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vaddq_f64 is always available on aarch64")]
            unsafe {
                Self(vaddq_f64(self.0, rhs.0))
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
                reason = "SSE2 _mm_add_pd is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_add_pd(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f64x2_add(self.0, rhs.0))
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
            Self(Align16F64x2([a[0] + b[0], a[1] + b[1]]))
        }
    }

    #[inline(always)]
    pub(crate) fn sub(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vsubq_f64 is always available on aarch64")]
            unsafe {
                Self(vsubq_f64(self.0, rhs.0))
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
                reason = "SSE2 _mm_sub_pd is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_sub_pd(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f64x2_sub(self.0, rhs.0))
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
            Self(Align16F64x2([a[0] - b[0], a[1] - b[1]]))
        }
    }

    #[inline(always)]
    pub(crate) fn mul(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vmulq_f64 is always available on aarch64")]
            unsafe {
                Self(vmulq_f64(self.0, rhs.0))
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
                reason = "SSE2 _mm_mul_pd is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_mul_pd(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f64x2_mul(self.0, rhs.0))
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
            Self(Align16F64x2([a[0] * b[0], a[1] * b[1]]))
        }
    }

    #[inline(always)]
    pub(crate) fn div(self, rhs: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vdivq_f64 is always available on aarch64")]
            unsafe {
                Self(vdivq_f64(self.0, rhs.0))
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
                reason = "SSE2 _mm_div_pd is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_div_pd(self.0, rhs.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f64x2_div(self.0, rhs.0))
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
            Self(Align16F64x2([a[0] / b[0], a[1] / b[1]]))
        }
    }

    #[inline(always)]
    pub(crate) fn rem(self, rhs: Self) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        Self::from_array([a[0] % b[0], a[1] % b[1]])
    }

    #[inline(always)]
    pub(crate) fn neg(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vnegq_f64 is always available on aarch64")]
            unsafe {
                Self(vnegq_f64(self.0))
            }
        }
        #[cfg(all(
            any(target_arch = "wasm32", target_arch = "wasm64"),
            target_feature = "simd128",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            Self(f64x2_neg(self.0))
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
            Self::from_array([-a[0], -a[1]])
        }
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vfmaq_f64 is always available on aarch64")]
            unsafe {
                Self(vfmaq_f64(c.0, self.0, b.0))
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
                reason = "FMA _mm_fmadd_pd is guarded by target_feature fma"
            )]
            unsafe {
                Self(_mm_fmadd_pd(self.0, b.0, c.0))
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
            #[expect(unsafe_code, reason = "NEON vfmaq_f64 is true IEEE-754 FMA")]
            unsafe {
                Self(vfmaq_f64(c.0, self.0, b.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let x = self.to_array();
            let y = b.to_array();
            let z = c.to_array();
            Self::from_array([
                crate::math::f64_math::mul_add(x[0], y[0], z[0]),
                crate::math::f64_math::mul_add(x[1], y[1], z[1]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn abs(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vabsq_f64 is always available on aarch64")]
            unsafe {
                Self(vabsq_f64(self.0))
            }
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0.abs())
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(feature = "core-simd", not(feature = "scalar-math")),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::abs(a[0]),
                crate::math::f64_math::abs(a[1]),
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
                reason = "NEON vminnmq_f64 is always available on aarch64"
            )]
            unsafe {
                Self(vminnmq_f64(self.0, rhs.0))
            }
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0.simd_min(rhs.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(feature = "core-simd", not(feature = "scalar-math")),
        )))]
        {
            let a = self.to_array();
            let b = rhs.to_array();
            Self::from_array([a[0].min(b[0]), a[1].min(b[1])])
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
                reason = "NEON vmaxnmq_f64 is always available on aarch64"
            )]
            unsafe {
                Self(vmaxnmq_f64(self.0, rhs.0))
            }
        }
        #[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
        {
            Self(self.0.simd_max(rhs.0))
        }
        #[cfg(not(any(
            all(
                target_arch = "aarch64",
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
            all(feature = "core-simd", not(feature = "scalar-math")),
        )))]
        {
            let a = self.to_array();
            let b = rhs.to_array();
            Self::from_array([a[0].max(b[0]), a[1].max(b[1])])
        }
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vrndmq_f64 is always available on aarch64")]
            unsafe {
                Self(vrndmq_f64(self.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::floor(a[0]),
                crate::math::f64_math::floor(a[1]),
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
            #[expect(unsafe_code, reason = "NEON vrndpq_f64 is always available on aarch64")]
            unsafe {
                Self(vrndpq_f64(self.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::ceil(a[0]),
                crate::math::f64_math::ceil(a[1]),
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
            #[expect(unsafe_code, reason = "NEON vrndq_f64 is always available on aarch64")]
            unsafe {
                Self(vrndq_f64(self.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::trunc(a[0]),
                crate::math::f64_math::trunc(a[1]),
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
            #[expect(unsafe_code, reason = "NEON vrndaq_f64 is always available on aarch64")]
            unsafe {
                Self(vrndaq_f64(self.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::round(a[0]),
                crate::math::f64_math::round(a[1]),
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
            #[expect(unsafe_code, reason = "NEON vrndnq_f64 is always available on aarch64")]
            unsafe {
                Self(vrndnq_f64(self.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::round_even(a[0]),
                crate::math::f64_math::round_even(a[1]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn fract(self) -> Self {
        self.sub(self.floor())
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        let a = self.to_array();
        Self::from_array([
            crate::math::f64_math::sign(a[0]),
            crate::math::f64_math::sign(a[1]),
        ])
    }

    #[inline(always)]
    pub(crate) fn sqrt(self) -> Self {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(unsafe_code, reason = "NEON vsqrtq_f64 is always available on aarch64")]
            unsafe {
                Self(vsqrtq_f64(self.0))
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
                reason = "SSE2 _mm_sqrt_pd is guarded by target_feature sse2"
            )]
            unsafe {
                Self(_mm_sqrt_pd(self.0))
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
                not(any(feature = "core-simd", feature = "scalar-math"))
            ),
        )))]
        {
            let a = self.to_array();
            Self::from_array([
                crate::math::f64_math::sqrt(a[0]),
                crate::math::f64_math::sqrt(a[1]),
            ])
        }
    }

    #[inline(always)]
    pub(crate) fn dot2(self, rhs: Self) -> f64 {
        #[cfg(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        ))]
        {
            #[expect(
                unsafe_code,
                reason = "NEON vmulq_f64 and vaddvq_f64 are available on aarch64"
            )]
            unsafe {
                vaddvq_f64(vmulq_f64(self.0, rhs.0))
            }
        }
        #[cfg(not(all(
            target_arch = "aarch64",
            not(any(feature = "core-simd", feature = "scalar-math"))
        )))]
        {
            let p = self.mul(rhs).to_array();
            p[0] + p[1]
        }
    }
}
