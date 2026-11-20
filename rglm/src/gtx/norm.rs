#[cfg(feature = "f16")]
use crate::math::f16_math;
use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;
use crate::{Quat, Vec2, Vec3, Vec3A, Vec4};

#[cfg(feature = "f64")]
use crate::{DQuat, DVec2, DVec3, DVec4};

#[cfg(feature = "f16")]
use crate::{F16Quat, F16Vec2, F16Vec3, F16Vec4, f16};

mod private {
    pub trait Sealed {}
}

pub trait Length2Op: private::Sealed + Copy {
    type Scalar: Copy;

    fn length2_op(self) -> Self::Scalar;
}

pub trait Distance2Op: Length2Op {
    fn distance2_op(self, rhs: Self) -> Self::Scalar;
}

pub trait VecNormOp: Distance2Op {
    fn l1_norm_op(self) -> Self::Scalar;
    fn l2_norm_op(self) -> Self::Scalar;
    fn lx_norm_op(self, depth: u32) -> Self::Scalar;
    fn l_max_norm_op(self) -> Self::Scalar;
}

macro_rules! impl_scalar_norm {
    ($s:ty) => {
        impl private::Sealed for $s {}

        impl Length2Op for $s {
            type Scalar = $s;

            #[inline(always)]
            fn length2_op(self) -> $s {
                self * self
            }
        }

        impl Distance2Op for $s {
            #[inline(always)]
            fn distance2_op(self, rhs: Self) -> $s {
                let d = self - rhs;
                d * d
            }
        }
    };
}

impl_scalar_norm!(f32);
#[cfg(feature = "f64")]
impl_scalar_norm!(f64);
#[cfg(feature = "f16")]
impl_scalar_norm!(f16);

macro_rules! impl_quat_norm {
    ($q:ty, $s:ty) => {
        impl private::Sealed for $q {}

        impl Length2Op for $q {
            type Scalar = $s;

            #[inline(always)]
            fn length2_op(self) -> $s {
                self.length_squared()
            }
        }
    };
}

impl_quat_norm!(Quat, f32);
#[cfg(feature = "f64")]
impl_quat_norm!(DQuat, f64);
#[cfg(feature = "f16")]
impl_quat_norm!(F16Quat, f16);

macro_rules! impl_float_vec_norm {
    ($v:ty, $s:ty, $math:ident, $n:literal, $zero:expr, $one:expr) => {
        impl private::Sealed for $v {}

        impl Length2Op for $v {
            type Scalar = $s;

            #[inline(always)]
            fn length2_op(self) -> $s {
                self.dot(self)
            }
        }

        impl Distance2Op for $v {
            #[inline(always)]
            fn distance2_op(self, rhs: Self) -> $s {
                let d = self - rhs;
                d.dot(d)
            }
        }

        impl VecNormOp for $v {
            #[inline(always)]
            fn l1_norm_op(self) -> $s {
                let a = self.abs().to_array();
                let mut sum = $zero;
                let mut k = 0;
                while k < $n {
                    sum += a[k];
                    k += 1;
                }
                sum
            }

            #[inline(always)]
            fn l2_norm_op(self) -> $s {
                self.length()
            }

            #[inline]
            fn lx_norm_op(self, depth: u32) -> $s {
                if depth == 0 {
                    return $zero;
                }
                let a = self.abs().to_array();
                let mut sum = $zero;
                let mut k = 0;
                while k < $n {
                    let mut term = $one;
                    let mut c = 0u32;
                    while c < depth {
                        term *= a[k];
                        c += 1;
                    }
                    sum += term;
                    k += 1;
                }
                $math::powf(sum, $one / Self::splat_depth(depth))
            }

            #[inline(always)]
            fn l_max_norm_op(self) -> $s {
                let a = self.abs().to_array();
                let mut m = a[0];
                let mut k = 1;
                while k < $n {
                    m = m.max(a[k]);
                    k += 1;
                }
                m
            }
        }
    };
}

trait SplatDepth<S> {
    fn splat_depth(depth: u32) -> S;
}

impl SplatDepth<f32> for Vec2 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f32 {
        depth as f32
    }
}
impl SplatDepth<f32> for Vec3 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f32 {
        depth as f32
    }
}
impl SplatDepth<f32> for Vec3A {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f32 {
        depth as f32
    }
}
impl SplatDepth<f32> for Vec4 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f32 {
        depth as f32
    }
}

#[cfg(feature = "f64")]
impl SplatDepth<f64> for DVec2 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f64 {
        depth as f64
    }
}
#[cfg(feature = "f64")]
impl SplatDepth<f64> for DVec3 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f64 {
        depth as f64
    }
}
#[cfg(feature = "f64")]
impl SplatDepth<f64> for DVec4 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f64 {
        depth as f64
    }
}

#[cfg(feature = "f16")]
impl SplatDepth<f16> for F16Vec2 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f16 {
        f16::from_f32(depth as f32)
    }
}
#[cfg(feature = "f16")]
impl SplatDepth<f16> for F16Vec3 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f16 {
        f16::from_f32(depth as f32)
    }
}
#[cfg(feature = "f16")]
impl SplatDepth<f16> for F16Vec4 {
    #[inline(always)]
    fn splat_depth(depth: u32) -> f16 {
        f16::from_f32(depth as f32)
    }
}

impl_float_vec_norm!(Vec2, f32, f32_math, 2, 0.0_f32, 1.0_f32);
impl_float_vec_norm!(Vec3, f32, f32_math, 3, 0.0_f32, 1.0_f32);
impl_float_vec_norm!(Vec3A, f32, f32_math, 3, 0.0_f32, 1.0_f32);
impl_float_vec_norm!(Vec4, f32, f32_math, 4, 0.0_f32, 1.0_f32);

#[cfg(feature = "f64")]
impl_float_vec_norm!(DVec2, f64, f64_math, 2, 0.0_f64, 1.0_f64);
#[cfg(feature = "f64")]
impl_float_vec_norm!(DVec3, f64, f64_math, 3, 0.0_f64, 1.0_f64);
#[cfg(feature = "f64")]
impl_float_vec_norm!(DVec4, f64, f64_math, 4, 0.0_f64, 1.0_f64);

#[cfg(feature = "f16")]
impl_float_vec_norm!(F16Vec2, f16, f16_math, 2, f16::ZERO, f16::ONE);
#[cfg(feature = "f16")]
impl_float_vec_norm!(F16Vec3, f16, f16_math, 3, f16::ZERO, f16::ONE);
#[cfg(feature = "f16")]
impl_float_vec_norm!(F16Vec4, f16, f16_math, 4, f16::ZERO, f16::ONE);

/// Returns the squared Euclidean norm `dot(x, x)` (`glm::length2`).
#[inline(always)]
#[must_use]
pub fn length2<T: Length2Op>(x: T) -> T::Scalar {
    x.length2_op()
}

/// Returns the squared Euclidean distance `length2(p0 - p1)` (`glm::distance2`).
#[inline(always)]
#[must_use]
pub fn distance2<T: Distance2Op>(p0: T, p1: T) -> T::Scalar {
    p0.distance2_op(p1)
}

/// Returns the Manhattan ($L_1$) norm of vector `v` (`glm::l1Norm`).
#[inline(always)]
#[must_use]
pub fn l1_norm<V: VecNormOp>(v: V) -> V::Scalar {
    v.l1_norm_op()
}

/// Returns the Euclidean ($L_2$) norm of vector `v` (`glm::l2Norm`).
#[inline(always)]
#[must_use]
pub fn l2_norm<V: VecNormOp>(v: V) -> V::Scalar {
    v.l2_norm_op()
}

/// Returns the $L_p$ norm `(sum |v_i|^depth)^(1 / depth)` of vector `v` (`glm::lxNorm`).
#[inline]
#[must_use]
pub fn lx_norm<V: VecNormOp>(v: V, depth: u32) -> V::Scalar {
    v.lx_norm_op(depth)
}

/// Returns the Chebyshev ($L_\infty$) norm `max |v_i|` of vector `v` (`glm::lMaxNorm`).
#[inline(always)]
#[must_use]
pub fn l_max_norm<V: VecNormOp>(v: V) -> V::Scalar {
    v.l_max_norm_op()
}
