#[cfg(feature = "f16")]
use crate::math::f16_math;
use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;

mod private {
    pub trait Sealed {}
}

pub trait GeometricScalarOrVec: private::Sealed + Copy {
    type Scalar: Copy;

    fn length_op(self) -> Self::Scalar;
    fn dot_op(self, rhs: Self) -> Self::Scalar;
    fn normalize_op(self) -> Self;
}

pub trait GeometricGenFloat: GeometricScalarOrVec {
    fn distance_op(self, rhs: Self) -> Self::Scalar;
    fn faceforward_op(self, i: Self, nref: Self) -> Self;
    fn reflect_op(self, n: Self) -> Self;
}

pub trait CrossOp: private::Sealed + Copy {
    fn cross_op(self, rhs: Self) -> Self;
}

pub trait RefractOp<Eta, Out>: private::Sealed {
    fn refract_op(self, n: Self, eta: Eta) -> Out;
}

macro_rules! impl_scalar_geom {
    ($s:ty, $math:ident, $zero:expr, $one:expr, $two:expr) => {
        impl private::Sealed for $s {}

        impl GeometricScalarOrVec for $s {
            type Scalar = $s;

            #[inline(always)]
            fn length_op(self) -> $s {
                $math::abs(self)
            }

            #[inline(always)]
            fn dot_op(self, rhs: Self) -> $s {
                self * rhs
            }

            #[inline(always)]
            fn normalize_op(self) -> Self {
                let len_sq = self * self;
                rglm_debug_assert!(len_sq > $zero);
                self * $math::inversesqrt(len_sq)
            }
        }

        impl GeometricGenFloat for $s {
            #[inline(always)]
            fn distance_op(self, rhs: Self) -> $s {
                $math::abs(self - rhs)
            }

            #[inline(always)]
            fn faceforward_op(self, i: Self, nref: Self) -> Self {
                if (nref * i) < $zero { self } else { -self }
            }

            #[inline(always)]
            fn reflect_op(self, n: Self) -> Self {
                self - ($two * (n * self) * n)
            }
        }

        impl RefractOp<$s, $s> for $s {
            #[inline]
            fn refract_op(self, n: Self, eta: $s) -> $s {
                let d = n * self;
                let k = $one - eta * eta * ($one - d * d);
                if k < $zero {
                    $zero
                } else {
                    eta * self - (eta * d + $math::sqrt(k)) * n
                }
            }
        }
    };
}

impl_scalar_geom!(f32, f32_math, 0.0_f32, 1.0_f32, 2.0_f32);
#[cfg(feature = "f64")]
impl_scalar_geom!(f64, f64_math, 0.0_f64, 1.0_f64, 2.0_f64);
#[cfg(feature = "f16")]
impl_scalar_geom!(
    half::f16,
    f16_math,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0)
);

macro_rules! impl_vec_geom {
    ($v:ty, $s:ty, $math:ident, $zero:expr, $one:expr, $two:expr) => {
        impl private::Sealed for $v {}

        impl GeometricScalarOrVec for $v {
            type Scalar = $s;

            #[inline(always)]
            fn length_op(self) -> $s {
                self.length()
            }

            #[inline(always)]
            fn dot_op(self, rhs: Self) -> $s {
                self.dot(rhs)
            }

            #[inline(always)]
            fn normalize_op(self) -> Self {
                self.normalize()
            }
        }

        impl GeometricGenFloat for $v {
            #[inline(always)]
            fn distance_op(self, rhs: Self) -> $s {
                (self - rhs).length()
            }

            #[inline(always)]
            fn faceforward_op(self, i: Self, nref: Self) -> Self {
                if nref.dot(i) < $zero { self } else { -self }
            }

            #[inline(always)]
            fn reflect_op(self, n: Self) -> Self {
                let factor = n.dot(self) * $two;
                self - n * factor
            }
        }

        impl RefractOp<$s, $v> for $v {
            #[inline]
            fn refract_op(self, n: Self, eta: $s) -> $v {
                let d = n.dot(self);
                let k = $one - eta * eta * ($one - d * d);
                if k < $zero {
                    <$v>::ZERO
                } else {
                    self * eta - n * (eta * d + $math::sqrt(k))
                }
            }
        }
    };
}

impl_vec_geom!(crate::Vec2, f32, f32_math, 0.0_f32, 1.0_f32, 2.0_f32);
impl_vec_geom!(crate::Vec3, f32, f32_math, 0.0_f32, 1.0_f32, 2.0_f32);
impl_vec_geom!(crate::Vec3A, f32, f32_math, 0.0_f32, 1.0_f32, 2.0_f32);
impl_vec_geom!(crate::Vec4, f32, f32_math, 0.0_f32, 1.0_f32, 2.0_f32);

#[cfg(feature = "f64")]
impl_vec_geom!(crate::DVec2, f64, f64_math, 0.0_f64, 1.0_f64, 2.0_f64);
#[cfg(feature = "f64")]
impl_vec_geom!(crate::DVec3, f64, f64_math, 0.0_f64, 1.0_f64, 2.0_f64);
#[cfg(feature = "f64")]
impl_vec_geom!(crate::DVec4, f64, f64_math, 0.0_f64, 1.0_f64, 2.0_f64);

#[cfg(feature = "f16")]
impl_vec_geom!(
    crate::F16Vec2,
    half::f16,
    f16_math,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0)
);
#[cfg(feature = "f16")]
impl_vec_geom!(
    crate::F16Vec3,
    half::f16,
    f16_math,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0)
);
#[cfg(feature = "f16")]
impl_vec_geom!(
    crate::F16Vec4,
    half::f16,
    f16_math,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0)
);

macro_rules! impl_quat_geom {
    ($q:ty, $s:ty) => {
        impl private::Sealed for $q {}

        impl GeometricScalarOrVec for $q {
            type Scalar = $s;

            #[inline(always)]
            fn length_op(self) -> $s {
                self.length()
            }

            #[inline(always)]
            fn dot_op(self, rhs: Self) -> $s {
                self.dot(rhs)
            }

            #[inline(always)]
            fn normalize_op(self) -> Self {
                self.normalize()
            }
        }

        impl CrossOp for $q {
            #[inline(always)]
            fn cross_op(self, rhs: Self) -> Self {
                self * rhs
            }
        }
    };
}

impl_quat_geom!(crate::Quat, f32);
#[cfg(feature = "f64")]
impl_quat_geom!(crate::DQuat, f64);
#[cfg(feature = "f16")]
impl_quat_geom!(crate::F16Quat, half::f16);

macro_rules! impl_vec3_cross {
    ($($v:ty),+ $(,)?) => {
        $(
            impl CrossOp for $v {
                #[inline(always)]
                fn cross_op(self, rhs: Self) -> Self {
                    self.cross(rhs)
                }
            }
        )+
    };
}

impl_vec3_cross!(crate::Vec3, crate::Vec3A);
#[cfg(feature = "f64")]
impl_vec3_cross!(crate::DVec3);
#[cfg(feature = "f16")]
impl_vec3_cross!(crate::F16Vec3);

/// Returns the Euclidean length (norm) of scalar, vector, or quaternion `x`.
#[inline(always)]
#[must_use]
pub fn length<T: GeometricScalarOrVec>(x: T) -> T::Scalar {
    x.length_op()
}

/// Returns the Euclidean distance `length(p0 - p1)` between `p0` and `p1`.
#[inline(always)]
#[must_use]
pub fn distance<T: GeometricGenFloat>(p0: T, p1: T) -> T::Scalar {
    p0.distance_op(p1)
}

/// Returns the dot (inner) product of `x` and `y`.
#[inline(always)]
#[must_use]
pub fn dot<T: GeometricScalarOrVec>(x: T, y: T) -> T::Scalar {
    x.dot_op(y)
}

/// Returns the 3D cross product of `x` and `y` (or Hamilton product `x * y` for quaternions).
#[inline(always)]
#[must_use]
pub fn cross<T: CrossOp>(x: T, y: T) -> T {
    x.cross_op(y)
}

/// Returns a vector or quaternion in the same direction as `x` with unit length (`NaN` if `x` has zero length).
#[inline(always)]
#[must_use]
pub fn normalize<T: GeometricScalarOrVec>(x: T) -> T {
    x.normalize_op()
}

/// Returns `n` if `dot(nref, i) < 0`, otherwise returns `-n`.
#[inline(always)]
#[must_use]
pub fn faceforward<T: GeometricGenFloat>(n: T, i: T, nref: T) -> T {
    n.faceforward_op(i, nref)
}

/// Computes the reflection direction `i - 2 * dot(n, i) * n` for incident vector `i` and unit normal `n`.
#[inline(always)]
#[must_use]
pub fn reflect<T: GeometricGenFloat>(i: T, n: T) -> T {
    i.reflect_op(n)
}

/// Computes the refraction vector for incident vector `i`, unit normal `n`, and index ratio `eta` (or zero under total internal reflection).
#[inline]
#[must_use]
pub fn refract<V, Eta, Out>(i: V, n: V, eta: Eta) -> Out
where
    V: RefractOp<Eta, Out>,
{
    i.refract_op(n, eta)
}
