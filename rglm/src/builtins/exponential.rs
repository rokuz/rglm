use crate::math::f32_math;

mod private {
    pub trait Sealed {}
}

pub trait ExpUnary: private::Sealed + Copy {
    fn exp_op(self) -> Self;
    fn log_op(self) -> Self;
    fn exp2_op(self) -> Self;
    fn log2_op(self) -> Self;
    fn sqrt_op(self) -> Self;
    fn inversesqrt_op(self) -> Self;
}

pub trait PowOp<Y, Out>: private::Sealed {
    fn pow_op(self, y: Y) -> Out;
}

impl private::Sealed for f32 {}
impl ExpUnary for f32 {
    #[inline(always)]
    fn exp_op(self) -> Self {
        f32_math::exp(self)
    }
    #[inline(always)]
    fn log_op(self) -> Self {
        f32_math::ln(self)
    }
    #[inline(always)]
    fn exp2_op(self) -> Self {
        f32_math::exp2(self)
    }
    #[inline(always)]
    fn log2_op(self) -> Self {
        f32_math::log2(self)
    }
    #[inline(always)]
    fn sqrt_op(self) -> Self {
        f32_math::sqrt(self)
    }
    #[inline(always)]
    fn inversesqrt_op(self) -> Self {
        f32_math::inversesqrt(self)
    }
}

impl PowOp<f32, f32> for f32 {
    #[inline(always)]
    fn pow_op(self, y: f32) -> f32 {
        f32_math::powf(self, y)
    }
}

#[cfg(feature = "f64")]
impl private::Sealed for f64 {}
#[cfg(feature = "f64")]
impl ExpUnary for f64 {
    #[inline(always)]
    fn exp_op(self) -> Self {
        crate::math::f64_math::exp(self)
    }
    #[inline(always)]
    fn log_op(self) -> Self {
        crate::math::f64_math::ln(self)
    }
    #[inline(always)]
    fn exp2_op(self) -> Self {
        crate::math::f64_math::exp2(self)
    }
    #[inline(always)]
    fn log2_op(self) -> Self {
        crate::math::f64_math::log2(self)
    }
    #[inline(always)]
    fn sqrt_op(self) -> Self {
        crate::math::f64_math::sqrt(self)
    }
    #[inline(always)]
    fn inversesqrt_op(self) -> Self {
        crate::math::f64_math::inversesqrt(self)
    }
}

#[cfg(feature = "f64")]
impl PowOp<f64, f64> for f64 {
    #[inline(always)]
    fn pow_op(self, y: f64) -> f64 {
        crate::math::f64_math::powf(self, y)
    }
}

#[cfg(feature = "f16")]
impl private::Sealed for half::f16 {}
#[cfg(feature = "f16")]
impl ExpUnary for half::f16 {
    #[inline(always)]
    fn exp_op(self) -> Self {
        crate::math::f16_math::exp(self)
    }
    #[inline(always)]
    fn log_op(self) -> Self {
        crate::math::f16_math::ln(self)
    }
    #[inline(always)]
    fn exp2_op(self) -> Self {
        crate::math::f16_math::exp2(self)
    }
    #[inline(always)]
    fn log2_op(self) -> Self {
        crate::math::f16_math::log2(self)
    }
    #[inline(always)]
    fn sqrt_op(self) -> Self {
        crate::math::f16_math::sqrt(self)
    }
    #[inline(always)]
    fn inversesqrt_op(self) -> Self {
        crate::math::f16_math::inversesqrt(self)
    }
}

#[cfg(feature = "f16")]
impl PowOp<half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn pow_op(self, y: half::f16) -> half::f16 {
        crate::math::f16_math::powf(self, y)
    }
}

macro_rules! impl_exp_vec_scalar {
    ($vec_t:ty, $s:ty, $n:literal) => {
        impl private::Sealed for $vec_t {}
        impl ExpUnary for $vec_t {
            #[inline]
            fn exp_op(self) -> Self {
                Self::from_array(self.to_array().map(ExpUnary::exp_op))
            }
            #[inline]
            fn log_op(self) -> Self {
                Self::from_array(self.to_array().map(ExpUnary::log_op))
            }
            #[inline]
            fn exp2_op(self) -> Self {
                Self::from_array(self.to_array().map(ExpUnary::exp2_op))
            }
            #[inline]
            fn log2_op(self) -> Self {
                Self::from_array(self.to_array().map(ExpUnary::log2_op))
            }
            #[inline(always)]
            fn sqrt_op(self) -> Self {
                self.sqrt()
            }
            #[inline(always)]
            fn inversesqrt_op(self) -> Self {
                self.inversesqrt()
            }
        }

        impl PowOp<$vec_t, $vec_t> for $vec_t {
            #[inline]
            fn pow_op(self, y: $vec_t) -> $vec_t {
                let xa = self.to_array();
                let ya = y.to_array();
                let mut out = xa;
                let mut i = 0;
                while i < $n {
                    out[i] = xa[i].pow_op(ya[i]);
                    i += 1;
                }
                Self::from_array(out)
            }
        }

        impl PowOp<$s, $vec_t> for $vec_t {
            #[inline]
            fn pow_op(self, y: $s) -> $vec_t {
                self.pow_op(Self::splat(y))
            }
        }
    };
}

impl_exp_vec_scalar!(crate::Vec2, f32, 2);
impl_exp_vec_scalar!(crate::Vec3, f32, 3);

#[cfg(feature = "f64")]
impl_exp_vec_scalar!(crate::DVec2, f64, 2);
#[cfg(feature = "f64")]
impl_exp_vec_scalar!(crate::DVec3, f64, 3);
#[cfg(feature = "f64")]
impl_exp_vec_scalar!(crate::DVec4, f64, 4);

#[cfg(feature = "f16")]
impl_exp_vec_scalar!(crate::F16Vec2, half::f16, 2);
#[cfg(feature = "f16")]
impl_exp_vec_scalar!(crate::F16Vec3, half::f16, 3);
#[cfg(feature = "f16")]
impl_exp_vec_scalar!(crate::F16Vec4, half::f16, 4);

macro_rules! impl_exp_simd_f32x4 {
    ($vec_t:ty) => {
        impl private::Sealed for $vec_t {}
        impl ExpUnary for $vec_t {
            #[inline]
            fn exp_op(self) -> Self {
                Self(self.0.exp())
            }
            #[inline]
            fn log_op(self) -> Self {
                Self(self.0.ln())
            }
            #[inline]
            fn exp2_op(self) -> Self {
                Self(self.0.exp2())
            }
            #[inline]
            fn log2_op(self) -> Self {
                Self(self.0.log2())
            }
            #[inline(always)]
            fn sqrt_op(self) -> Self {
                self.sqrt()
            }
            #[inline(always)]
            fn inversesqrt_op(self) -> Self {
                self.inversesqrt()
            }
        }

        impl PowOp<$vec_t, $vec_t> for $vec_t {
            #[inline]
            fn pow_op(self, y: $vec_t) -> $vec_t {
                Self(self.0.powf(y.0))
            }
        }

        impl PowOp<f32, $vec_t> for $vec_t {
            #[inline]
            fn pow_op(self, y: f32) -> $vec_t {
                Self(self.0.powf(crate::simd::SimdF32x4::splat(y)))
            }
        }
    };
}

impl_exp_simd_f32x4!(crate::Vec3A);
impl_exp_simd_f32x4!(crate::Vec4);

/// Raises `x` to the power `y` component-wise (`x^y`).
#[inline]
#[must_use]
pub fn pow<X, Y>(x: X, y: Y) -> X
where
    X: PowOp<Y, X>,
{
    x.pow_op(y)
}

/// Computes the natural base-`e` exponential `e^x` component-wise.
#[inline]
#[must_use]
pub fn exp<T: ExpUnary>(x: T) -> T {
    x.exp_op()
}

/// Computes the natural base-`e` logarithm `ln(x)` component-wise; returns `NaN` for `x < 0` and `-Inf` for `x == 0`.
#[inline]
#[must_use]
pub fn log<T: ExpUnary>(x: T) -> T {
    x.log_op()
}

/// Computes `2^x` component-wise.
#[inline]
#[must_use]
pub fn exp2<T: ExpUnary>(x: T) -> T {
    x.exp2_op()
}

/// Computes the base-2 logarithm `log2(x)` component-wise; returns `NaN` for `x < 0` and `-Inf` for `x == 0`.
#[inline]
#[must_use]
pub fn log2<T: ExpUnary>(x: T) -> T {
    x.log2_op()
}

/// Computes the non-negative square root `sqrt(x)` component-wise; returns `NaN` for `x < 0`.
#[inline(always)]
#[must_use]
pub fn sqrt<T: ExpUnary>(x: T) -> T {
    x.sqrt_op()
}

/// Computes the reciprocal square root `1 / sqrt(x)` component-wise.
#[inline(always)]
#[must_use]
pub fn inversesqrt<T: ExpUnary>(x: T) -> T {
    x.inversesqrt_op()
}
