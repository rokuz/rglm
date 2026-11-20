use crate::math::f32_math;
use crate::simd::SimdF32x4;

mod private {
    pub trait Sealed {}
}

pub trait TrigUnary: private::Sealed + Copy {
    fn to_radians_op(self) -> Self;
    fn to_degrees_op(self) -> Self;
    fn sin_op(self) -> Self;
    fn cos_op(self) -> Self;
    fn sincos_op(self) -> (Self, Self);
    fn tan_op(self) -> Self;
    fn asin_op(self) -> Self;
    fn acos_op(self) -> Self;
    fn atan_op(self) -> Self;
    fn atan2_op(self, x: Self) -> Self;
    fn sinh_op(self) -> Self;
    fn cosh_op(self) -> Self;
    fn tanh_op(self) -> Self;
    fn asinh_op(self) -> Self;
    fn acosh_op(self) -> Self;
    fn atanh_op(self) -> Self;
}

impl private::Sealed for f32 {}
impl TrigUnary for f32 {
    #[inline(always)]
    fn to_radians_op(self) -> Self {
        self.to_radians()
    }
    #[inline(always)]
    fn to_degrees_op(self) -> Self {
        self.to_degrees()
    }
    #[inline(always)]
    fn sin_op(self) -> Self {
        f32_math::sin(self)
    }
    #[inline(always)]
    fn cos_op(self) -> Self {
        f32_math::cos(self)
    }
    #[inline(always)]
    fn sincos_op(self) -> (Self, Self) {
        f32_math::sin_cos(self)
    }
    #[inline(always)]
    fn tan_op(self) -> Self {
        f32_math::tan(self)
    }
    #[inline(always)]
    fn asin_op(self) -> Self {
        f32_math::asin(self)
    }
    #[inline(always)]
    fn acos_op(self) -> Self {
        f32_math::acos(self)
    }
    #[inline(always)]
    fn atan_op(self) -> Self {
        f32_math::atan(self)
    }
    #[inline(always)]
    fn atan2_op(self, x: Self) -> Self {
        f32_math::atan2(self, x)
    }
    #[inline(always)]
    fn sinh_op(self) -> Self {
        f32_math::sinh(self)
    }
    #[inline(always)]
    fn cosh_op(self) -> Self {
        f32_math::cosh(self)
    }
    #[inline(always)]
    fn tanh_op(self) -> Self {
        f32_math::tanh(self)
    }
    #[inline(always)]
    fn asinh_op(self) -> Self {
        f32_math::asinh(self)
    }
    #[inline(always)]
    fn acosh_op(self) -> Self {
        f32_math::acosh(self)
    }
    #[inline(always)]
    fn atanh_op(self) -> Self {
        f32_math::atanh(self)
    }
}

#[cfg(feature = "f64")]
impl private::Sealed for f64 {}
#[cfg(feature = "f64")]
impl TrigUnary for f64 {
    #[inline(always)]
    fn to_radians_op(self) -> Self {
        self.to_radians()
    }
    #[inline(always)]
    fn to_degrees_op(self) -> Self {
        self.to_degrees()
    }
    #[inline(always)]
    fn sin_op(self) -> Self {
        crate::math::f64_math::sin(self)
    }
    #[inline(always)]
    fn cos_op(self) -> Self {
        crate::math::f64_math::cos(self)
    }
    #[inline(always)]
    fn sincos_op(self) -> (Self, Self) {
        crate::math::f64_math::sin_cos(self)
    }
    #[inline(always)]
    fn tan_op(self) -> Self {
        crate::math::f64_math::tan(self)
    }
    #[inline(always)]
    fn asin_op(self) -> Self {
        crate::math::f64_math::asin(self)
    }
    #[inline(always)]
    fn acos_op(self) -> Self {
        crate::math::f64_math::acos(self)
    }
    #[inline(always)]
    fn atan_op(self) -> Self {
        crate::math::f64_math::atan(self)
    }
    #[inline(always)]
    fn atan2_op(self, x: Self) -> Self {
        crate::math::f64_math::atan2(self, x)
    }
    #[inline(always)]
    fn sinh_op(self) -> Self {
        crate::math::f64_math::sinh(self)
    }
    #[inline(always)]
    fn cosh_op(self) -> Self {
        crate::math::f64_math::cosh(self)
    }
    #[inline(always)]
    fn tanh_op(self) -> Self {
        crate::math::f64_math::tanh(self)
    }
    #[inline(always)]
    fn asinh_op(self) -> Self {
        crate::math::f64_math::asinh(self)
    }
    #[inline(always)]
    fn acosh_op(self) -> Self {
        crate::math::f64_math::acosh(self)
    }
    #[inline(always)]
    fn atanh_op(self) -> Self {
        crate::math::f64_math::atanh(self)
    }
}

#[cfg(feature = "f16")]
impl private::Sealed for half::f16 {}
#[cfg(feature = "f16")]
impl TrigUnary for half::f16 {
    #[inline(always)]
    fn to_radians_op(self) -> Self {
        half::f16::from_f32(self.to_f32().to_radians())
    }
    #[inline(always)]
    fn to_degrees_op(self) -> Self {
        half::f16::from_f32(self.to_f32().to_degrees())
    }
    #[inline(always)]
    fn sin_op(self) -> Self {
        crate::math::f16_math::sin(self)
    }
    #[inline(always)]
    fn cos_op(self) -> Self {
        crate::math::f16_math::cos(self)
    }
    #[inline(always)]
    fn sincos_op(self) -> (Self, Self) {
        crate::math::f16_math::sin_cos(self)
    }
    #[inline(always)]
    fn tan_op(self) -> Self {
        crate::math::f16_math::tan(self)
    }
    #[inline(always)]
    fn asin_op(self) -> Self {
        crate::math::f16_math::asin(self)
    }
    #[inline(always)]
    fn acos_op(self) -> Self {
        crate::math::f16_math::acos(self)
    }
    #[inline(always)]
    fn atan_op(self) -> Self {
        crate::math::f16_math::atan(self)
    }
    #[inline(always)]
    fn atan2_op(self, x: Self) -> Self {
        crate::math::f16_math::atan2(self, x)
    }
    #[inline(always)]
    fn sinh_op(self) -> Self {
        crate::math::f16_math::sinh(self)
    }
    #[inline(always)]
    fn cosh_op(self) -> Self {
        crate::math::f16_math::cosh(self)
    }
    #[inline(always)]
    fn tanh_op(self) -> Self {
        crate::math::f16_math::tanh(self)
    }
    #[inline(always)]
    fn asinh_op(self) -> Self {
        crate::math::f16_math::asinh(self)
    }
    #[inline(always)]
    fn acosh_op(self) -> Self {
        crate::math::f16_math::acosh(self)
    }
    #[inline(always)]
    fn atanh_op(self) -> Self {
        crate::math::f16_math::atanh(self)
    }
}

macro_rules! impl_trig_vec_scalar {
    ($vec_t:ty, $n:literal, $deg_to_rad:expr, $rad_to_deg:expr) => {
        impl private::Sealed for $vec_t {}
        impl TrigUnary for $vec_t {
            #[inline(always)]
            fn to_radians_op(self) -> Self {
                self * $deg_to_rad
            }
            #[inline(always)]
            fn to_degrees_op(self) -> Self {
                self * $rad_to_deg
            }
            #[inline]
            fn sin_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::sin_op))
            }
            #[inline]
            fn cos_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::cos_op))
            }
            #[inline]
            fn sincos_op(self) -> (Self, Self) {
                let a = self.to_array();
                let mut s = a;
                let mut c = a;
                let mut i = 0;
                while i < $n {
                    let (si, ci) = a[i].sincos_op();
                    s[i] = si;
                    c[i] = ci;
                    i += 1;
                }
                (Self::from_array(s), Self::from_array(c))
            }
            #[inline]
            fn tan_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::tan_op))
            }
            #[inline]
            fn asin_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::asin_op))
            }
            #[inline]
            fn acos_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::acos_op))
            }
            #[inline]
            fn atan_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::atan_op))
            }
            #[inline]
            fn atan2_op(self, x: Self) -> Self {
                let ya = self.to_array();
                let xa = x.to_array();
                let mut out = ya;
                let mut i = 0;
                while i < $n {
                    out[i] = ya[i].atan2_op(xa[i]);
                    i += 1;
                }
                Self::from_array(out)
            }
            #[inline]
            fn sinh_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::sinh_op))
            }
            #[inline]
            fn cosh_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::cosh_op))
            }
            #[inline]
            fn tanh_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::tanh_op))
            }
            #[inline]
            fn asinh_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::asinh_op))
            }
            #[inline]
            fn acosh_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::acosh_op))
            }
            #[inline]
            fn atanh_op(self) -> Self {
                Self::from_array(self.to_array().map(TrigUnary::atanh_op))
            }
        }
    };
}

const DEG_TO_RAD_F32: f32 = (core::f64::consts::PI / 180.0) as f32;
const RAD_TO_DEG_F32: f32 = (180.0 / core::f64::consts::PI) as f32;

impl_trig_vec_scalar!(crate::Vec2, 2, DEG_TO_RAD_F32, RAD_TO_DEG_F32);
impl_trig_vec_scalar!(crate::Vec3, 3, DEG_TO_RAD_F32, RAD_TO_DEG_F32);

#[cfg(feature = "f64")]
impl_trig_vec_scalar!(
    crate::DVec2,
    2,
    core::f64::consts::PI / 180.0,
    180.0 / core::f64::consts::PI
);
#[cfg(feature = "f64")]
impl_trig_vec_scalar!(
    crate::DVec3,
    3,
    core::f64::consts::PI / 180.0,
    180.0 / core::f64::consts::PI
);
#[cfg(feature = "f64")]
impl_trig_vec_scalar!(
    crate::DVec4,
    4,
    core::f64::consts::PI / 180.0,
    180.0 / core::f64::consts::PI
);

#[cfg(feature = "f16")]
impl_trig_vec_scalar!(
    crate::F16Vec2,
    2,
    half::f16::from_f32_const(core::f32::consts::PI / 180.0),
    half::f16::from_f32_const(180.0 / core::f32::consts::PI)
);
#[cfg(feature = "f16")]
impl_trig_vec_scalar!(
    crate::F16Vec3,
    3,
    half::f16::from_f32_const(core::f32::consts::PI / 180.0),
    half::f16::from_f32_const(180.0 / core::f32::consts::PI)
);
#[cfg(feature = "f16")]
impl_trig_vec_scalar!(
    crate::F16Vec4,
    4,
    half::f16::from_f32_const(core::f32::consts::PI / 180.0),
    half::f16::from_f32_const(180.0 / core::f32::consts::PI)
);

macro_rules! impl_trig_simd_f32x4 {
    ($vec_t:ty) => {
        impl private::Sealed for $vec_t {}
        impl TrigUnary for $vec_t {
            #[inline(always)]
            fn to_radians_op(self) -> Self {
                self * DEG_TO_RAD_F32
            }
            #[inline(always)]
            fn to_degrees_op(self) -> Self {
                self * RAD_TO_DEG_F32
            }
            #[inline]
            fn sin_op(self) -> Self {
                Self(self.0.sin())
            }
            #[inline]
            fn cos_op(self) -> Self {
                Self(self.0.cos())
            }
            #[inline]
            fn sincos_op(self) -> (Self, Self) {
                let (s, c) = self.0.sin_cos();
                (Self(s), Self(c))
            }
            #[inline]
            fn tan_op(self) -> Self {
                Self(self.0.tan())
            }
            #[inline]
            fn asin_op(self) -> Self {
                Self(self.0.asin())
            }
            #[inline]
            fn acos_op(self) -> Self {
                Self(self.0.acos())
            }
            #[inline]
            fn atan_op(self) -> Self {
                Self(self.0.atan())
            }
            #[inline]
            fn atan2_op(self, x: Self) -> Self {
                Self(self.0.atan2(x.0))
            }
            #[inline]
            fn sinh_op(self) -> Self {
                let a = self.0.to_array();
                Self(SimdF32x4::from_array([
                    f32_math::sinh(a[0]),
                    f32_math::sinh(a[1]),
                    f32_math::sinh(a[2]),
                    f32_math::sinh(a[3]),
                ]))
            }
            #[inline]
            fn cosh_op(self) -> Self {
                let a = self.0.to_array();
                Self(SimdF32x4::from_array([
                    f32_math::cosh(a[0]),
                    f32_math::cosh(a[1]),
                    f32_math::cosh(a[2]),
                    f32_math::cosh(a[3]),
                ]))
            }
            #[inline]
            fn tanh_op(self) -> Self {
                let a = self.0.to_array();
                Self(SimdF32x4::from_array([
                    f32_math::tanh(a[0]),
                    f32_math::tanh(a[1]),
                    f32_math::tanh(a[2]),
                    f32_math::tanh(a[3]),
                ]))
            }
            #[inline]
            fn asinh_op(self) -> Self {
                let a = self.0.to_array();
                Self(SimdF32x4::from_array([
                    f32_math::asinh(a[0]),
                    f32_math::asinh(a[1]),
                    f32_math::asinh(a[2]),
                    f32_math::asinh(a[3]),
                ]))
            }
            #[inline]
            fn acosh_op(self) -> Self {
                let a = self.0.to_array();
                Self(SimdF32x4::from_array([
                    f32_math::acosh(a[0]),
                    f32_math::acosh(a[1]),
                    f32_math::acosh(a[2]),
                    f32_math::acosh(a[3]),
                ]))
            }
            #[inline]
            fn atanh_op(self) -> Self {
                let a = self.0.to_array();
                Self(SimdF32x4::from_array([
                    f32_math::atanh(a[0]),
                    f32_math::atanh(a[1]),
                    f32_math::atanh(a[2]),
                    f32_math::atanh(a[3]),
                ]))
            }
        }
    };
}

impl_trig_simd_f32x4!(crate::Vec3A);
impl_trig_simd_f32x4!(crate::Vec4);

/// Converts degrees to radians component-wise (`pi / 180 * degrees`).
#[inline(always)]
#[must_use]
pub fn radians<T: TrigUnary>(degrees: T) -> T {
    degrees.to_radians_op()
}

/// Converts radians to degrees component-wise (`180 / pi * radians`).
#[inline(always)]
#[must_use]
pub fn degrees<T: TrigUnary>(radians: T) -> T {
    radians.to_degrees_op()
}

/// Computes the trigonometric sine of `angle` in radians component-wise.
#[inline]
#[must_use]
pub fn sin<T: TrigUnary>(angle: T) -> T {
    angle.sin_op()
}

/// Computes the trigonometric cosine of `angle` in radians component-wise.
#[inline]
#[must_use]
pub fn cos<T: TrigUnary>(angle: T) -> T {
    angle.cos_op()
}

/// Simultaneously computes `(sin(angle), cos(angle))` in radians component-wise.
#[inline]
#[must_use]
pub fn sincos<T: TrigUnary>(angle: T) -> (T, T) {
    angle.sincos_op()
}

/// Computes the trigonometric tangent of `angle` in radians component-wise.
#[inline]
#[must_use]
pub fn tan<T: TrigUnary>(angle: T) -> T {
    angle.tan_op()
}

/// Computes the principal arc-sine in `[-pi/2, pi/2]` component-wise; returns `NaN` where `|x| > 1`.
#[inline]
#[must_use]
pub fn asin<T: TrigUnary>(x: T) -> T {
    x.asin_op()
}

/// Computes the principal arc-cosine in `[0, pi]` component-wise; returns `NaN` where `|x| > 1`.
#[inline]
#[must_use]
pub fn acos<T: TrigUnary>(x: T) -> T {
    x.acos_op()
}

/// Computes the 1-argument principal arc-tangent in `[-pi/2, pi/2]` component-wise.
#[inline]
#[must_use]
pub fn atan<T: TrigUnary>(y_over_x: T) -> T {
    y_over_x.atan_op()
}

/// Computes the 2-argument quadrant-aware arc-tangent `atan(y, x)` in `[-pi, pi]` component-wise.
#[inline]
#[must_use]
pub fn atan2<T: TrigUnary>(y: T, x: T) -> T {
    y.atan2_op(x)
}

/// Variadic GLSL `atan` macro dispatching `atan!(x)` to [`atan`] and `atan!(y, x)` to [`atan2`].
#[macro_export]
macro_rules! atan {
    ($y_over_x:expr $(,)?) => {
        $crate::atan($y_over_x)
    };
    ($y:expr, $x:expr $(,)?) => {
        $crate::atan2($y, $x)
    };
}

/// Computes the hyperbolic sine `(e^x - e^-x) / 2` component-wise.
#[inline]
#[must_use]
pub fn sinh<T: TrigUnary>(x: T) -> T {
    x.sinh_op()
}

/// Computes the hyperbolic cosine `(e^x + e^-x) / 2` component-wise.
#[inline]
#[must_use]
pub fn cosh<T: TrigUnary>(x: T) -> T {
    x.cosh_op()
}

/// Computes the hyperbolic tangent `sinh(x) / cosh(x)` component-wise.
#[inline]
#[must_use]
pub fn tanh<T: TrigUnary>(x: T) -> T {
    x.tanh_op()
}

/// Computes the inverse hyperbolic sine `ln(x + sqrt(x^2 + 1))` component-wise.
#[inline]
#[must_use]
pub fn asinh<T: TrigUnary>(x: T) -> T {
    x.asinh_op()
}

/// Computes the non-negative inverse hyperbolic cosine component-wise; returns `NaN` where `x < 1`.
#[inline]
#[must_use]
pub fn acosh<T: TrigUnary>(x: T) -> T {
    x.acosh_op()
}

/// Computes the inverse hyperbolic tangent `0.5 * ln((1 + x) / (1 - x))` component-wise; returns `NaN` where `|x| > 1`.
#[inline]
#[must_use]
pub fn atanh<T: TrigUnary>(x: T) -> T {
    x.atanh_op()
}
