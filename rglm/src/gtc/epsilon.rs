#[cfg(feature = "f16")]
use crate::math::f16_math;
use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;
use crate::{BVec2, BVec3, BVec4, Quat, Vec2, Vec3, Vec3A, Vec4};

#[cfg(feature = "f64")]
use crate::{DQuat, DVec2, DVec3, DVec4};

#[cfg(feature = "f16")]
use crate::{F16Quat, F16Vec2, F16Vec3, F16Vec4, f16};

mod private {
    pub trait Sealed {}
}

pub trait EpsilonCmpOp<Eps, Out>: private::Sealed + Copy {
    fn epsilon_equal_op(self, y: Self, eps: Eps) -> Out;
    fn epsilon_not_equal_op(self, y: Self, eps: Eps) -> Out;
}

macro_rules! impl_scalar_eps {
    ($s:ty, $math:ident) => {
        impl private::Sealed for $s {}

        impl EpsilonCmpOp<$s, bool> for $s {
            #[inline(always)]
            fn epsilon_equal_op(self, y: Self, eps: $s) -> bool {
                $math::abs(self - y) < eps
            }

            #[inline(always)]
            fn epsilon_not_equal_op(self, y: Self, eps: $s) -> bool {
                $math::abs(self - y) >= eps
            }
        }
    };
}

impl_scalar_eps!(f32, f32_math);
#[cfg(feature = "f64")]
impl_scalar_eps!(f64, f64_math);
#[cfg(feature = "f16")]
impl_scalar_eps!(f16, f16_math);

macro_rules! impl_vec_eps {
    ($v:ty, $s:ty, $m:ty) => {
        impl private::Sealed for $v {}

        impl EpsilonCmpOp<$v, $m> for $v {
            #[inline(always)]
            fn epsilon_equal_op(self, y: Self, eps: $v) -> $m {
                (self - y).abs().cmp_lt(eps)
            }

            #[inline(always)]
            fn epsilon_not_equal_op(self, y: Self, eps: $v) -> $m {
                (self - y).abs().cmp_ge(eps)
            }
        }

        impl EpsilonCmpOp<$s, $m> for $v {
            #[inline(always)]
            fn epsilon_equal_op(self, y: Self, eps: $s) -> $m {
                (self - y).abs().cmp_lt(<$v>::splat(eps))
            }

            #[inline(always)]
            fn epsilon_not_equal_op(self, y: Self, eps: $s) -> $m {
                (self - y).abs().cmp_ge(<$v>::splat(eps))
            }
        }
    };
}

impl_vec_eps!(Vec2, f32, BVec2);
impl_vec_eps!(Vec3, f32, BVec3);
impl_vec_eps!(Vec3A, f32, BVec3);
impl_vec_eps!(Vec4, f32, BVec4);

#[cfg(feature = "f64")]
impl_vec_eps!(DVec2, f64, BVec2);
#[cfg(feature = "f64")]
impl_vec_eps!(DVec3, f64, BVec3);
#[cfg(feature = "f64")]
impl_vec_eps!(DVec4, f64, BVec4);

#[cfg(feature = "f16")]
impl_vec_eps!(F16Vec2, f16, BVec2);
#[cfg(feature = "f16")]
impl_vec_eps!(F16Vec3, f16, BVec3);
#[cfg(feature = "f16")]
impl_vec_eps!(F16Vec4, f16, BVec4);

macro_rules! impl_quat_eps {
    ($q:ty, $s:ty, $math:ident) => {
        impl private::Sealed for $q {}

        impl EpsilonCmpOp<$s, BVec4> for $q {
            #[inline(always)]
            fn epsilon_equal_op(self, y: Self, eps: $s) -> BVec4 {
                let d = (self - y).to_array();
                BVec4::new(
                    $math::abs(d[0]) < eps,
                    $math::abs(d[1]) < eps,
                    $math::abs(d[2]) < eps,
                    $math::abs(d[3]) < eps,
                )
            }

            #[inline(always)]
            fn epsilon_not_equal_op(self, y: Self, eps: $s) -> BVec4 {
                let d = (self - y).to_array();
                BVec4::new(
                    $math::abs(d[0]) >= eps,
                    $math::abs(d[1]) >= eps,
                    $math::abs(d[2]) >= eps,
                    $math::abs(d[3]) >= eps,
                )
            }
        }
    };
}

impl_quat_eps!(Quat, f32, f32_math);
#[cfg(feature = "f64")]
impl_quat_eps!(DQuat, f64, f64_math);
#[cfg(feature = "f16")]
impl_quat_eps!(F16Quat, f16, f16_math);

/// Returns component-wise `|x - y| < epsilon` (`glm::epsilonEqual`).
#[inline(always)]
#[must_use]
pub fn epsilon_equal<T, Eps, Out>(x: T, y: T, epsilon: Eps) -> Out
where
    T: EpsilonCmpOp<Eps, Out>,
{
    x.epsilon_equal_op(y, epsilon)
}

/// Returns component-wise `|x - y| >= epsilon` (`glm::epsilonNotEqual`).
#[inline(always)]
#[must_use]
pub fn epsilon_not_equal<T, Eps, Out>(x: T, y: T, epsilon: Eps) -> Out
where
    T: EpsilonCmpOp<Eps, Out>,
{
    x.epsilon_not_equal_op(y, epsilon)
}
