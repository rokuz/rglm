#[cfg(feature = "f16")]
use crate::math::f16_math;
use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;
use crate::{Mat3, Quat, Vec3, Vec3A};

#[cfg(feature = "f64")]
use crate::{DMat3, DQuat, DVec3};

#[cfg(feature = "f16")]
use crate::{F16Mat3, F16Quat, F16Vec3, f16};

mod private {
    pub trait Sealed {}
}

pub trait QuatExt: private::Sealed + Copy {
    type Scalar: Copy;
    type Vec3Out: Copy;

    fn conjugate_op(self) -> Self;
    fn slerp_op(self, y: Self, a: Self::Scalar) -> Self;
    fn lerp_op(self, y: Self, a: Self::Scalar) -> Self;
    fn roll_op(self) -> Self::Scalar;
    fn pitch_op(self) -> Self::Scalar;
    fn yaw_op(self) -> Self::Scalar;
    fn euler_angles_op(self) -> Self::Vec3Out;
}

pub trait QuatLookAt: private::Sealed + Copy {
    type QuatOut: Copy;

    fn quat_look_at_rh_op(self, up: Self) -> Self::QuatOut;
    fn quat_look_at_lh_op(self, up: Self) -> Self::QuatOut;
}

macro_rules! impl_quat_ext {
    ($q:ty, $v3:ty, $m3:ty, $s:ty, $math:ident, $zero:expr, $one:expr, $two:expr, $neg_one:expr) => {
        impl private::Sealed for $q {}

        impl QuatExt for $q {
            type Scalar = $s;
            type Vec3Out = $v3;

            #[inline(always)]
            fn conjugate_op(self) -> Self {
                self.conjugate()
            }

            #[inline]
            fn slerp_op(self, y: Self, a: $s) -> Self {
                self.slerp(y, a)
            }

            #[inline(always)]
            fn lerp_op(self, y: Self, a: $s) -> Self {
                self * ($one - a) + y * a
            }

            #[inline]
            fn roll_op(self) -> $s {
                let y = $two * (self.x * self.y + self.w * self.z);
                let x = self.w * self.w + self.x * self.x - self.y * self.y - self.z * self.z;
                if y == $zero && x == $zero {
                    $zero
                } else {
                    $math::atan2(y, x)
                }
            }

            #[inline]
            fn pitch_op(self) -> $s {
                let y = $two * (self.y * self.z + self.w * self.x);
                let x = self.w * self.w - self.x * self.x - self.y * self.y + self.z * self.z;
                if y == $zero && x == $zero {
                    $two * $math::atan2(self.x, self.w)
                } else {
                    $math::atan2(y, x)
                }
            }

            #[inline]
            fn yaw_op(self) -> $s {
                let v = -$two * (self.x * self.z - self.w * self.y);
                $math::asin(v.clamp($neg_one, $one))
            }

            #[inline]
            fn euler_angles_op(self) -> $v3 {
                <$v3>::new(self.pitch_op(), self.yaw_op(), self.roll_op())
            }
        }

        impl private::Sealed for $v3 {}

        impl QuatLookAt for $v3 {
            type QuatOut = $q;

            #[inline]
            fn quat_look_at_rh_op(self, up: Self) -> $q {
                let col2 = -self;
                let col0 = up.cross(col2).normalize();
                let col1 = col2.cross(col0);
                <$q>::from_mat3(<$m3>::from_cols(col0, col1, col2))
            }

            #[inline]
            fn quat_look_at_lh_op(self, up: Self) -> $q {
                let col2 = self;
                let col0 = up.cross(col2).normalize();
                let col1 = col2.cross(col0);
                <$q>::from_mat3(<$m3>::from_cols(col0, col1, col2))
            }
        }
    };
}

impl_quat_ext!(
    Quat, Vec3, Mat3, f32, f32_math, 0.0_f32, 1.0_f32, 2.0_f32, -1.0_f32
);

impl private::Sealed for Vec3A {}
impl QuatLookAt for Vec3A {
    type QuatOut = Quat;

    #[inline]
    fn quat_look_at_rh_op(self, up: Self) -> Quat {
        Vec3::from(self).quat_look_at_rh_op(Vec3::from(up))
    }

    #[inline]
    fn quat_look_at_lh_op(self, up: Self) -> Quat {
        Vec3::from(self).quat_look_at_lh_op(Vec3::from(up))
    }
}

#[cfg(feature = "f64")]
impl_quat_ext!(
    DQuat, DVec3, DMat3, f64, f64_math, 0.0_f64, 1.0_f64, 2.0_f64, -1.0_f64
);

#[cfg(feature = "f16")]
impl_quat_ext!(
    F16Quat,
    F16Vec3,
    F16Mat3,
    f16,
    f16_math,
    f16::ZERO,
    f16::ONE,
    f16::from_f32_const(2.0),
    f16::NEG_ONE
);

/// Returns the quaternion conjugate `(-x, -y, -z, w)` (`glm::conjugate`).
#[inline(always)]
#[must_use]
pub fn conjugate<Q: QuatExt>(q: Q) -> Q {
    q.conjugate_op()
}

/// Spherical linear interpolation along the shortest arc between unit quaternions `x` and `y` (`glm::slerp`).
#[inline]
#[must_use]
pub fn slerp<Q: QuatExt>(x: Q, y: Q, a: Q::Scalar) -> Q {
    x.slerp_op(y, a)
}

/// Component-wise linear interpolation `(1 - a) * x + a * y` between quaternions `x` and `y` (`glm::lerp`).
#[inline(always)]
#[must_use]
pub fn quat_lerp<Q: QuatExt>(x: Q, y: Q, a: Q::Scalar) -> Q {
    x.lerp_op(y, a)
}

/// Extracts the roll angle in radians around the Z axis from quaternion `q` (`glm::roll`).
#[inline]
#[must_use]
pub fn roll<Q: QuatExt>(q: Q) -> Q::Scalar {
    q.roll_op()
}

/// Extracts the pitch angle in radians around the X axis from quaternion `q` (`glm::pitch`).
#[inline]
#[must_use]
pub fn pitch<Q: QuatExt>(q: Q) -> Q::Scalar {
    q.pitch_op()
}

/// Extracts the yaw angle in radians around the Y axis from quaternion `q` (`glm::yaw`).
#[inline]
#[must_use]
pub fn yaw<Q: QuatExt>(q: Q) -> Q::Scalar {
    q.yaw_op()
}

/// Extracts `(pitch, yaw, roll)` Euler angles in radians from quaternion `q` (`glm::eulerAngles`).
#[inline]
#[must_use]
pub fn euler_angles<Q: QuatExt>(q: Q) -> Q::Vec3Out {
    q.euler_angles_op()
}

/// Builds an orientation quaternion looking along unit `direction` in a right-handed coordinate system (`glm::quatLookAtRH`).
#[inline]
#[must_use]
pub fn quat_look_at_rh<V: QuatLookAt>(direction: V, up: V) -> V::QuatOut {
    direction.quat_look_at_rh_op(up)
}

/// Builds an orientation quaternion looking along unit `direction` in a left-handed coordinate system (`glm::quatLookAtLH`).
#[inline]
#[must_use]
pub fn quat_look_at_lh<V: QuatLookAt>(direction: V, up: V) -> V::QuatOut {
    direction.quat_look_at_lh_op(up)
}
