use crate::math::f32_math;
use crate::{Vec2, Vec3, Vec3A, Vec4};

mod private {
    pub trait Sealed {}
}

pub trait FastMathUnary: private::Sealed + Copy {
    fn fast_inversesqrt_op(self) -> Self;
    fn fast_sin_op(self) -> Self;
    fn fast_cos_op(self) -> Self;
    fn fast_exp_op(self) -> Self;
    fn fast_log_op(self) -> Self;
}

pub trait FastGeomVec: FastMathUnary {
    fn fast_length_op(self) -> f32;
    fn fast_normalize_op(self) -> Self;
}

#[inline(always)]
fn scalar_fast_rsqrt(x: f32) -> f32 {
    if x <= 0.0 || !x.is_finite() {
        if x == 0.0 {
            return f32::INFINITY.copysign(x);
        }
        if x.is_infinite() && x.is_sign_positive() {
            return 0.0;
        }
        return f32::NAN;
    }
    let y = f32::from_bits(0x5f37_59df - (x.to_bits() >> 1));
    y * (1.5 - 0.5 * x * y * y)
}

#[inline(always)]
fn scalar_fast_sin(x: f32) -> f32 {
    if !x.is_finite() {
        return f32::NAN;
    }
    let k = f32_math::round(x * core::f32::consts::FRAC_1_PI);
    let r = x - k * core::f32::consts::PI;
    let r2 = r * r;
    let p = r * (1.0 + r2 * (-0.166_666_67 + r2 * (0.008_333_334 - r2 * 0.000_198_412_7)));
    if ((k as i32) & 1) != 0 { -p } else { p }
}

#[inline(always)]
fn scalar_fast_cos(x: f32) -> f32 {
    scalar_fast_sin(x + core::f32::consts::FRAC_PI_2)
}

#[inline(always)]
fn scalar_fast_exp(x: f32) -> f32 {
    if x.is_nan() {
        return f32::NAN;
    }
    if x < -87.0 {
        return 0.0;
    }
    if x > 88.0 {
        return f32::INFINITY;
    }
    let t = x * core::f32::consts::LOG2_E;
    let fi = f32_math::floor(t);
    let f = t - fi;
    let p = 1.0 + f * (core::f32::consts::LN_2 + f * (0.240_226_5 + f * 0.055_504_11));
    let scale = f32::from_bits((((fi as i32) + 127) as u32) << 23);
    p * scale
}

#[inline(always)]
fn scalar_fast_log(x: f32) -> f32 {
    if x < 0.0 || x.is_nan() {
        return f32::NAN;
    }
    if x == 0.0 {
        return f32::NEG_INFINITY;
    }
    if x.is_infinite() {
        return f32::INFINITY;
    }
    let bits = x.to_bits();
    let e = ((bits >> 23) as i32 - 127) as f32;
    let m = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000);
    let t = (m - 1.0) / (m + 1.0);
    let t2 = t * t;
    let ln_m = 2.0 * t * (1.0 + t2 * (0.333_333_34 + t2 * 0.2));
    e * core::f32::consts::LN_2 + ln_m
}

impl private::Sealed for f32 {}
impl FastMathUnary for f32 {
    #[inline(always)]
    fn fast_inversesqrt_op(self) -> Self {
        scalar_fast_rsqrt(self)
    }

    #[inline(always)]
    fn fast_sin_op(self) -> Self {
        scalar_fast_sin(self)
    }

    #[inline(always)]
    fn fast_cos_op(self) -> Self {
        scalar_fast_cos(self)
    }

    #[inline(always)]
    fn fast_exp_op(self) -> Self {
        scalar_fast_exp(self)
    }

    #[inline(always)]
    fn fast_log_op(self) -> Self {
        scalar_fast_log(self)
    }
}

macro_rules! impl_fast_vec {
    ($v:ty) => {
        impl private::Sealed for $v {}

        impl FastMathUnary for $v {
            #[inline(always)]
            fn fast_inversesqrt_op(self) -> Self {
                Self::from_array(self.to_array().map(scalar_fast_rsqrt))
            }

            #[inline(always)]
            fn fast_sin_op(self) -> Self {
                Self::from_array(self.to_array().map(scalar_fast_sin))
            }

            #[inline(always)]
            fn fast_cos_op(self) -> Self {
                Self::from_array(self.to_array().map(scalar_fast_cos))
            }

            #[inline(always)]
            fn fast_exp_op(self) -> Self {
                Self::from_array(self.to_array().map(scalar_fast_exp))
            }

            #[inline(always)]
            fn fast_log_op(self) -> Self {
                Self::from_array(self.to_array().map(scalar_fast_log))
            }
        }

        impl FastGeomVec for $v {
            #[inline(always)]
            fn fast_length_op(self) -> f32 {
                let d = self.dot(self);
                if d == 0.0 {
                    0.0
                } else {
                    d * scalar_fast_rsqrt(d)
                }
            }

            #[inline(always)]
            fn fast_normalize_op(self) -> Self {
                self * scalar_fast_rsqrt(self.dot(self))
            }
        }
    };
}

impl_fast_vec!(Vec2);
impl_fast_vec!(Vec3);
impl_fast_vec!(Vec3A);
impl_fast_vec!(Vec4);

/// Computes a fast ~12-bit reciprocal square root approximation (`glm::fastInverseSqrt`).
#[inline(always)]
#[must_use]
pub fn fast_inversesqrt<T: FastMathUnary>(x: T) -> T {
    x.fast_inversesqrt_op()
}

/// Computes a fast low-degree polynomial sine approximation (`glm::fastSin`).
#[inline(always)]
#[must_use]
pub fn fast_sin<T: FastMathUnary>(angle: T) -> T {
    angle.fast_sin_op()
}

/// Computes a fast low-degree polynomial cosine approximation (`glm::fastCos`).
#[inline(always)]
#[must_use]
pub fn fast_cos<T: FastMathUnary>(angle: T) -> T {
    angle.fast_cos_op()
}

/// Computes a fast cubic base-`e` exponential approximation (`glm::fastExp`).
#[inline(always)]
#[must_use]
pub fn fast_exp<T: FastMathUnary>(x: T) -> T {
    x.fast_exp_op()
}

/// Computes a fast rational natural logarithm approximation (`glm::fastLog`).
#[inline(always)]
#[must_use]
pub fn fast_log<T: FastMathUnary>(x: T) -> T {
    x.fast_log_op()
}

/// Computes a fast Euclidean length approximation using [`fast_inversesqrt`] (`glm::fastLength`).
#[inline(always)]
#[must_use]
pub fn fast_length<V: FastGeomVec>(v: V) -> f32 {
    v.fast_length_op()
}

/// Normalizes vector `v` using [`fast_inversesqrt`] (`glm::fastNormalize`).
#[inline(always)]
#[must_use]
pub fn fast_normalize<V: FastGeomVec>(v: V) -> V {
    v.fast_normalize_op()
}
