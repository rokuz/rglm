#[cfg(feature = "f16")]
use crate::math::f16_math;
use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;
use crate::{Mat4, Vec3, Vec3A, Vec4};

#[cfg(feature = "f64")]
use crate::{DMat4, DVec3, DVec4};

#[cfg(feature = "f16")]
use crate::{F16Mat4, F16Vec3, F16Vec4, f16};

mod private {
    pub trait Sealed {}
}

pub trait TransformVec3<M>: private::Sealed + Copy {
    type Scalar: Copy;

    fn translate_op(self, m: M) -> M;
    fn rotate_op(self, m: M, angle: Self::Scalar) -> M;
    fn scale_op(self, m: M) -> M;
}

pub trait CameraVec3: private::Sealed + Copy {
    type Mat4Out;

    fn look_at_rh_op(self, center: Self, up: Self) -> Self::Mat4Out;
    fn look_at_lh_op(self, center: Self, up: Self) -> Self::Mat4Out;
}

pub trait ProjectionScalar: private::Sealed + Copy {
    type Mat4Out;

    fn perspective_rh_zo_op(self, aspect: Self, z_near: Self, z_far: Self) -> Self::Mat4Out;
    fn perspective_vk_op(self, aspect: Self, z_near: Self, z_far: Self) -> Self::Mat4Out;
    fn perspective_rh_zo_rev_op(self, aspect: Self, z_near: Self, z_far: Self) -> Self::Mat4Out;
    fn perspective_vk_rev_op(self, aspect: Self, z_near: Self, z_far: Self) -> Self::Mat4Out;
    fn infinite_perspective_rh_zo_op(self, aspect: Self, z_near: Self) -> Self::Mat4Out;
    fn infinite_perspective_rh_zo_rev_op(self, aspect: Self, z_near: Self) -> Self::Mat4Out;
    fn perspective_lh_zo_op(self, aspect: Self, z_near: Self, z_far: Self) -> Self::Mat4Out;
    fn perspective_rh_no_op(self, aspect: Self, z_near: Self, z_far: Self) -> Self::Mat4Out;
    fn ortho_rh_zo_op(
        self,
        right: Self,
        bottom: Self,
        top: Self,
        z_near: Self,
        z_far: Self,
    ) -> Self::Mat4Out;
    fn ortho_vk_op(
        self,
        right: Self,
        bottom: Self,
        top: Self,
        z_near: Self,
        z_far: Self,
    ) -> Self::Mat4Out;
    fn ortho_lh_zo_op(
        self,
        right: Self,
        bottom: Self,
        top: Self,
        z_near: Self,
        z_far: Self,
    ) -> Self::Mat4Out;
    fn ortho_rh_no_op(
        self,
        right: Self,
        bottom: Self,
        top: Self,
        z_near: Self,
        z_far: Self,
    ) -> Self::Mat4Out;
}

pub trait ProjectVec3<M, V4>: private::Sealed + Copy {
    fn project_zo_op(self, model: M, proj: M, viewport: V4) -> Self;
    fn unproject_zo_op(self, model: M, proj: M, viewport: V4) -> Self;
}

macro_rules! impl_transform_and_camera {
    ($v3:ty, $v4:ty, $m4:ty, $s:ty, $math:ident, $zero:expr, $half:expr, $one:expr, $two:expr) => {
        impl private::Sealed for $v3 {}

        impl TransformVec3<$m4> for $v3 {
            type Scalar = $s;

            #[inline]
            fn translate_op(self, m: $m4) -> $m4 {
                let w = m.x_axis * self.x + m.y_axis * self.y + m.z_axis * self.z + m.w_axis;
                <$m4>::from_cols(m.x_axis, m.y_axis, m.z_axis, w)
            }

            #[inline]
            fn rotate_op(self, m: $m4, angle: $s) -> $m4 {
                let (s, c) = $math::sin_cos(angle);
                let axis = self.normalize();
                let temp = axis * ($one - c);

                let r00 = c + temp.x * axis.x;
                let r01 = temp.x * axis.y + s * axis.z;
                let r02 = temp.x * axis.z - s * axis.y;

                let r10 = temp.y * axis.x - s * axis.z;
                let r11 = c + temp.y * axis.y;
                let r12 = temp.y * axis.z + s * axis.x;

                let r20 = temp.z * axis.x + s * axis.y;
                let r21 = temp.z * axis.y - s * axis.x;
                let r22 = c + temp.z * axis.z;

                <$m4>::from_cols(
                    m.x_axis * r00 + m.y_axis * r01 + m.z_axis * r02,
                    m.x_axis * r10 + m.y_axis * r11 + m.z_axis * r12,
                    m.x_axis * r20 + m.y_axis * r21 + m.z_axis * r22,
                    m.w_axis,
                )
            }

            #[inline(always)]
            fn scale_op(self, m: $m4) -> $m4 {
                <$m4>::from_cols(
                    m.x_axis * self.x,
                    m.y_axis * self.y,
                    m.z_axis * self.z,
                    m.w_axis,
                )
            }
        }

        impl CameraVec3 for $v3 {
            type Mat4Out = $m4;

            #[inline]
            fn look_at_rh_op(self, center: Self, up: Self) -> $m4 {
                let f = (center - self).normalize();
                let s = f.cross(up).normalize();
                let u = s.cross(f);
                <$m4>::from_cols(
                    <$v4>::new(s.x, u.x, -f.x, $zero),
                    <$v4>::new(s.y, u.y, -f.y, $zero),
                    <$v4>::new(s.z, u.z, -f.z, $zero),
                    <$v4>::new(-s.dot(self), -u.dot(self), f.dot(self), $one),
                )
            }

            #[inline]
            fn look_at_lh_op(self, center: Self, up: Self) -> $m4 {
                let f = (center - self).normalize();
                let s = up.cross(f).normalize();
                let u = f.cross(s);
                <$m4>::from_cols(
                    <$v4>::new(s.x, u.x, f.x, $zero),
                    <$v4>::new(s.y, u.y, f.y, $zero),
                    <$v4>::new(s.z, u.z, f.z, $zero),
                    <$v4>::new(-s.dot(self), -u.dot(self), -f.dot(self), $one),
                )
            }
        }

        impl ProjectVec3<$m4, $v4> for $v3 {
            #[inline]
            fn project_zo_op(self, model: $m4, proj: $m4, viewport: $v4) -> Self {
                let mut tmp = proj * (model * <$v4>::new(self.x, self.y, self.z, $one));
                tmp /= tmp.w;
                let x = (tmp.x * $half + $half) * viewport.z + viewport.x;
                let y = (tmp.y * $half + $half) * viewport.w + viewport.y;
                Self::new(x, y, tmp.z)
            }

            #[inline]
            fn unproject_zo_op(self, model: $m4, proj: $m4, viewport: $v4) -> Self {
                let inv = (proj * model).inverse();
                let nx = ((self.x - viewport.x) / viewport.z) * $two - $one;
                let ny = ((self.y - viewport.y) / viewport.w) * $two - $one;
                let obj = inv * <$v4>::new(nx, ny, self.z, $one);
                Self::new(obj.x / obj.w, obj.y / obj.w, obj.z / obj.w)
            }
        }
    };
}

macro_rules! impl_projection_scalar {
    ($s:ty, $v4:ty, $m4:ty, $math:ident, $zero:expr, $half:expr, $one:expr, $two:expr) => {
        impl private::Sealed for $s {}

        impl ProjectionScalar for $s {
            type Mat4Out = $m4;

            #[inline]
            fn perspective_rh_zo_op(self, aspect: Self, z_near: Self, z_far: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = $one / tan_half_fovy;
                let m22 = z_far / (z_near - z_far);
                let m32 = -(z_far * z_near) / (z_far - z_near);
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, m22, -$one),
                    <$v4>::new($zero, $zero, m32, $zero),
                )
            }

            #[inline]
            fn perspective_vk_op(self, aspect: Self, z_near: Self, z_far: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = -$one / tan_half_fovy;
                let m22 = z_far / (z_near - z_far);
                let m32 = -(z_far * z_near) / (z_far - z_near);
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, m22, -$one),
                    <$v4>::new($zero, $zero, m32, $zero),
                )
            }

            #[inline]
            fn perspective_rh_zo_rev_op(self, aspect: Self, z_near: Self, z_far: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = $one / tan_half_fovy;
                let m22 = z_near / (z_far - z_near);
                let m32 = (z_far * z_near) / (z_far - z_near);
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, m22, -$one),
                    <$v4>::new($zero, $zero, m32, $zero),
                )
            }

            #[inline]
            fn perspective_vk_rev_op(self, aspect: Self, z_near: Self, z_far: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = -$one / tan_half_fovy;
                let m22 = z_near / (z_far - z_near);
                let m32 = (z_far * z_near) / (z_far - z_near);
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, m22, -$one),
                    <$v4>::new($zero, $zero, m32, $zero),
                )
            }

            #[inline]
            fn infinite_perspective_rh_zo_op(self, aspect: Self, z_near: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = $one / tan_half_fovy;
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, -$one, -$one),
                    <$v4>::new($zero, $zero, -z_near, $zero),
                )
            }

            #[inline]
            fn infinite_perspective_rh_zo_rev_op(self, aspect: Self, z_near: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = $one / tan_half_fovy;
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, $zero, -$one),
                    <$v4>::new($zero, $zero, z_near, $zero),
                )
            }

            #[inline]
            fn perspective_lh_zo_op(self, aspect: Self, z_near: Self, z_far: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = $one / tan_half_fovy;
                let m22 = z_far / (z_far - z_near);
                let m32 = -(z_far * z_near) / (z_far - z_near);
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, m22, $one),
                    <$v4>::new($zero, $zero, m32, $zero),
                )
            }

            #[inline]
            fn perspective_rh_no_op(self, aspect: Self, z_near: Self, z_far: Self) -> $m4 {
                rglm_debug_assert!($math::abs(aspect) > $zero);
                let tan_half_fovy = $math::tan(self * $half);
                let m00 = $one / (aspect * tan_half_fovy);
                let m11 = $one / tan_half_fovy;
                let m22 = -(z_far + z_near) / (z_far - z_near);
                let m32 = -($two * z_far * z_near) / (z_far - z_near);
                <$m4>::from_cols(
                    <$v4>::new(m00, $zero, $zero, $zero),
                    <$v4>::new($zero, m11, $zero, $zero),
                    <$v4>::new($zero, $zero, m22, -$one),
                    <$v4>::new($zero, $zero, m32, $zero),
                )
            }

            #[inline]
            fn ortho_rh_zo_op(
                self,
                right: Self,
                bottom: Self,
                top: Self,
                z_near: Self,
                z_far: Self,
            ) -> $m4 {
                let rl = right - self;
                let tb = top - bottom;
                let fn_d = z_far - z_near;
                <$m4>::from_cols(
                    <$v4>::new($two / rl, $zero, $zero, $zero),
                    <$v4>::new($zero, $two / tb, $zero, $zero),
                    <$v4>::new($zero, $zero, -$one / fn_d, $zero),
                    <$v4>::new(
                        -(right + self) / rl,
                        -(top + bottom) / tb,
                        -z_near / fn_d,
                        $one,
                    ),
                )
            }

            #[inline]
            fn ortho_vk_op(
                self,
                right: Self,
                bottom: Self,
                top: Self,
                z_near: Self,
                z_far: Self,
            ) -> $m4 {
                let rl = right - self;
                let tb = top - bottom;
                let fn_d = z_far - z_near;
                <$m4>::from_cols(
                    <$v4>::new($two / rl, $zero, $zero, $zero),
                    <$v4>::new($zero, -$two / tb, $zero, $zero),
                    <$v4>::new($zero, $zero, -$one / fn_d, $zero),
                    <$v4>::new(
                        -(right + self) / rl,
                        (top + bottom) / tb,
                        -z_near / fn_d,
                        $one,
                    ),
                )
            }

            #[inline]
            fn ortho_lh_zo_op(
                self,
                right: Self,
                bottom: Self,
                top: Self,
                z_near: Self,
                z_far: Self,
            ) -> $m4 {
                let rl = right - self;
                let tb = top - bottom;
                let fn_d = z_far - z_near;
                <$m4>::from_cols(
                    <$v4>::new($two / rl, $zero, $zero, $zero),
                    <$v4>::new($zero, $two / tb, $zero, $zero),
                    <$v4>::new($zero, $zero, $one / fn_d, $zero),
                    <$v4>::new(
                        -(right + self) / rl,
                        -(top + bottom) / tb,
                        -z_near / fn_d,
                        $one,
                    ),
                )
            }

            #[inline]
            fn ortho_rh_no_op(
                self,
                right: Self,
                bottom: Self,
                top: Self,
                z_near: Self,
                z_far: Self,
            ) -> $m4 {
                let rl = right - self;
                let tb = top - bottom;
                let fn_d = z_far - z_near;
                <$m4>::from_cols(
                    <$v4>::new($two / rl, $zero, $zero, $zero),
                    <$v4>::new($zero, $two / tb, $zero, $zero),
                    <$v4>::new($zero, $zero, -$two / fn_d, $zero),
                    <$v4>::new(
                        -(right + self) / rl,
                        -(top + bottom) / tb,
                        -(z_far + z_near) / fn_d,
                        $one,
                    ),
                )
            }
        }
    };
}

impl_transform_and_camera!(
    Vec3, Vec4, Mat4, f32, f32_math, 0.0_f32, 0.5_f32, 1.0_f32, 2.0_f32
);
impl_transform_and_camera!(
    Vec3A, Vec4, Mat4, f32, f32_math, 0.0_f32, 0.5_f32, 1.0_f32, 2.0_f32
);
impl_projection_scalar!(
    f32, Vec4, Mat4, f32_math, 0.0_f32, 0.5_f32, 1.0_f32, 2.0_f32
);

#[cfg(feature = "f64")]
impl_transform_and_camera!(
    DVec3, DVec4, DMat4, f64, f64_math, 0.0_f64, 0.5_f64, 1.0_f64, 2.0_f64
);
#[cfg(feature = "f64")]
impl_projection_scalar!(
    f64, DVec4, DMat4, f64_math, 0.0_f64, 0.5_f64, 1.0_f64, 2.0_f64
);

#[cfg(feature = "f16")]
impl_transform_and_camera!(
    F16Vec3,
    F16Vec4,
    F16Mat4,
    f16,
    f16_math,
    f16::ZERO,
    f16::from_f32_const(0.5),
    f16::ONE,
    f16::from_f32_const(2.0)
);
#[cfg(feature = "f16")]
impl_projection_scalar!(
    f16,
    F16Vec4,
    F16Mat4,
    f16_math,
    f16::ZERO,
    f16::from_f32_const(0.5),
    f16::ONE,
    f16::from_f32_const(2.0)
);

/// Post-multiplies 4x4 matrix `m` by a 3D translation `v` (`glm::translate`).
#[inline]
#[must_use]
pub fn translate<M, V: TransformVec3<M>>(m: M, v: V) -> M {
    v.translate_op(m)
}

/// Post-multiplies 4x4 matrix `m` by a rotation of `angle` radians around `axis` (`glm::rotate`).
#[inline]
#[must_use]
pub fn rotate<M, V: TransformVec3<M>>(m: M, angle: V::Scalar, axis: V) -> M {
    axis.rotate_op(m, angle)
}

/// Post-multiplies 4x4 matrix `m` by a 3D non-uniform scale `v` (`glm::scale`).
#[inline(always)]
#[must_use]
pub fn scale<M, V: TransformVec3<M>>(m: M, v: V) -> M {
    v.scale_op(m)
}

/// Builds a right-handed view matrix looking from `eye` toward `center` with `up` (`glm::lookAtRH`).
#[inline]
#[must_use]
pub fn look_at_rh<V: CameraVec3>(eye: V, center: V, up: V) -> V::Mat4Out {
    eye.look_at_rh_op(center, up)
}

/// Builds a left-handed view matrix looking from `eye` toward `center` with `up` (`glm::lookAtLH`).
#[inline]
#[must_use]
pub fn look_at_lh<V: CameraVec3>(eye: V, center: V, up: V) -> V::Mat4Out {
    eye.look_at_lh_op(center, up)
}

/// Builds a right-handed `[0, 1]` depth (`ZO`) perspective projection matrix (`glm::perspectiveRH_ZO`).
#[inline]
#[must_use]
pub fn perspective_rh_zo<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    fovy.perspective_rh_zo_op(aspect, z_near, z_far)
}

/// Builds a Vulkan clip-space (`RH`, `[0, 1]` depth `ZO`, `Y-down` NDC) perspective projection matrix.
#[inline]
#[must_use]
pub fn perspective_vk<S: ProjectionScalar>(fovy: S, aspect: S, z_near: S, z_far: S) -> S::Mat4Out {
    fovy.perspective_vk_op(aspect, z_near, z_far)
}

/// Builds a right-handed Reversed-Z (`z_near -> 1`, `z_far -> 0`) perspective projection matrix.
#[inline]
#[must_use]
pub fn perspective_rh_zo_rev<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    fovy.perspective_rh_zo_rev_op(aspect, z_near, z_far)
}

/// Builds a Vulkan clip-space (`RH`, `Y-down` NDC) Reversed-Z (`z_near -> 1`, `z_far -> 0`) perspective matrix.
#[inline]
#[must_use]
pub fn perspective_vk_rev<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    fovy.perspective_vk_rev_op(aspect, z_near, z_far)
}

/// Builds a right-handed `[0, 1]` depth (`z_near -> 0`, `inf -> 1`) infinite perspective matrix.
#[inline]
#[must_use]
pub fn infinite_perspective_rh_zo<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
) -> S::Mat4Out {
    fovy.infinite_perspective_rh_zo_op(aspect, z_near)
}

/// Builds a right-handed Reversed-Z (`z_near -> 1`, `inf -> 0`) infinite perspective matrix.
#[inline]
#[must_use]
pub fn infinite_perspective_rh_zo_rev<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
) -> S::Mat4Out {
    fovy.infinite_perspective_rh_zo_rev_op(aspect, z_near)
}

/// Builds a left-handed `[0, 1]` depth (`ZO`) perspective projection matrix (`glm::perspectiveLH_ZO`).
#[inline]
#[must_use]
pub fn perspective_lh_zo<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    fovy.perspective_lh_zo_op(aspect, z_near, z_far)
}

/// Builds a right-handed `[-1, 1]` depth (`NO`) perspective projection matrix (`glm::perspectiveRH_NO`).
#[inline]
#[must_use]
pub fn perspective_rh_no<S: ProjectionScalar>(
    fovy: S,
    aspect: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    fovy.perspective_rh_no_op(aspect, z_near, z_far)
}

/// Builds a right-handed `[0, 1]` depth (`ZO`) orthographic projection matrix (`glm::orthoRH_ZO`).
#[inline]
#[must_use]
pub fn ortho_rh_zo<S: ProjectionScalar>(
    left: S,
    right: S,
    bottom: S,
    top: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    left.ortho_rh_zo_op(right, bottom, top, z_near, z_far)
}

/// Builds a Vulkan clip-space (`RH`, `[0, 1]` depth `ZO`, `Y-down` NDC) orthographic projection matrix.
#[inline]
#[must_use]
pub fn ortho_vk<S: ProjectionScalar>(
    left: S,
    right: S,
    bottom: S,
    top: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    left.ortho_vk_op(right, bottom, top, z_near, z_far)
}

/// Builds a left-handed `[0, 1]` depth (`ZO`) orthographic projection matrix (`glm::orthoLH_ZO`).
#[inline]
#[must_use]
pub fn ortho_lh_zo<S: ProjectionScalar>(
    left: S,
    right: S,
    bottom: S,
    top: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    left.ortho_lh_zo_op(right, bottom, top, z_near, z_far)
}

/// Builds a right-handed `[-1, 1]` depth (`NO`) orthographic projection matrix (`glm::orthoRH_NO`).
#[inline]
#[must_use]
pub fn ortho_rh_no<S: ProjectionScalar>(
    left: S,
    right: S,
    bottom: S,
    top: S,
    z_near: S,
    z_far: S,
) -> S::Mat4Out {
    left.ortho_rh_no_op(right, bottom, top, z_near, z_far)
}

/// Maps object-space coordinates `obj` into window coordinates using `[0, 1]` depth (`glm::projectZO`).
#[inline]
#[must_use]
pub fn project_zo<M, V4, V: ProjectVec3<M, V4>>(obj: V, model: M, proj: M, viewport: V4) -> V {
    obj.project_zo_op(model, proj, viewport)
}

/// Maps window coordinates `win` back into object-space coordinates using `[0, 1]` depth (`glm::unProjectZO`).
#[inline]
#[must_use]
pub fn unproject_zo<M, V4, V: ProjectVec3<M, V4>>(win: V, model: M, proj: M, viewport: V4) -> V {
    win.unproject_zo_op(model, proj, viewport)
}
