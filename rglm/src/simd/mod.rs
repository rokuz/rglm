#![expect(
    dead_code,
    reason = "internal SIMD kernels are wired to public free functions in M2-M4"
)]

pub(crate) mod f32x4;
pub(crate) mod mask32x4;
pub(crate) mod math_poly;

#[cfg(feature = "f64")]
pub(crate) mod f64x2;

#[cfg(feature = "f16")]
pub(crate) mod f16x4;

pub(crate) use f32x4::SimdF32x4;
pub(crate) use mask32x4::SimdMask32x4;

#[cfg(feature = "f64")]
pub(crate) use f64x2::SimdF64x2;

#[cfg(feature = "f16")]
pub(crate) use f16x4::SimdF16x4;
