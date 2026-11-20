use crate::math::f32_math;
use crate::simd::SimdF32x4;
use crate::{Mat3, Mat4, Vec3, Vec4};
use crate::{Mat3A, Vec3A};
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct Quat(pub(crate) SimdF32x4);

impl Quat {
    pub const ZERO: Self = Self::from_xyzw(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);
    pub const NAN: Self = Self::from_xyzw(f32::NAN, f32::NAN, f32::NAN, f32::NAN);

    #[inline(always)]
    pub(crate) const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(SimdF32x4::new(x, y, z, w))
    }

    #[inline(always)]
    pub(crate) const fn from_wxyz(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self::from_xyzw(x, y, z, w)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f32; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f32; 4] {
        self.0.to_array()
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f32 {
        self.0.dot4(rhs.0)
    }

    #[inline(always)]
    pub(crate) fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub(crate) fn length(self) -> f32 {
        f32_math::sqrt(self.dot(self))
    }

    #[inline(always)]
    pub(crate) fn normalize(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > 0.0);
        self * f32_math::inversesqrt(len_sq)
    }

    #[inline(always)]
    pub(crate) fn conjugate(self) -> Self {
        Self::from_xyzw(-self.x, -self.y, -self.z, self.w)
    }

    #[inline(always)]
    pub(crate) fn inverse(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > 0.0);
        self.conjugate() / len_sq
    }

    #[inline]
    pub(crate) fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half = angle * 0.5;
        let (s, c) = f32_math::sin_cos(half);
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    #[inline]
    pub(crate) fn to_mat3(self) -> Mat3 {
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;
        let xx = self.x * x2;
        let xy = self.x * y2;
        let xz = self.x * z2;
        let yy = self.y * y2;
        let yz = self.y * z2;
        let zz = self.z * z2;
        let wx = self.w * x2;
        let wy = self.w * y2;
        let wz = self.w * z2;
        let one = 1.0;
        Mat3::from_cols(
            Vec3::new(one - (yy + zz), xy + wz, xz - wy),
            Vec3::new(xy - wz, one - (xx + zz), yz + wx),
            Vec3::new(xz + wy, yz - wx, one - (xx + yy)),
        )
    }

    #[inline]
    pub(crate) fn to_mat4(self) -> Mat4 {
        let m = self.to_mat3();
        let zero = 0.0;
        let one = 1.0;
        Mat4::from_cols(
            Vec4::new(m.x_axis.x, m.x_axis.y, m.x_axis.z, zero),
            Vec4::new(m.y_axis.x, m.y_axis.y, m.y_axis.z, zero),
            Vec4::new(m.z_axis.x, m.z_axis.y, m.z_axis.z, zero),
            Vec4::new(zero, zero, zero, one),
        )
    }

    #[inline]
    pub(crate) fn from_mat3(m: Mat3) -> Self {
        let m00 = m.x_axis.x;
        let m01 = m.x_axis.y;
        let m02 = m.x_axis.z;
        let m10 = m.y_axis.x;
        let m11 = m.y_axis.y;
        let m12 = m.y_axis.z;
        let m20 = m.z_axis.x;
        let m21 = m.z_axis.y;
        let m22 = m.z_axis.z;

        let four_x_sq_minus_1 = m00 - m11 - m22;
        let four_y_sq_minus_1 = m11 - m00 - m22;
        let four_z_sq_minus_1 = m22 - m00 - m11;
        let four_w_sq_minus_1 = m00 + m11 + m22;

        let mut biggest_idx = 0usize;
        let mut four_biggest_sq_minus_1 = four_w_sq_minus_1;
        if four_x_sq_minus_1 > four_biggest_sq_minus_1 {
            four_biggest_sq_minus_1 = four_x_sq_minus_1;
            biggest_idx = 1;
        }
        if four_y_sq_minus_1 > four_biggest_sq_minus_1 {
            four_biggest_sq_minus_1 = four_y_sq_minus_1;
            biggest_idx = 2;
        }
        if four_z_sq_minus_1 > four_biggest_sq_minus_1 {
            four_biggest_sq_minus_1 = four_z_sq_minus_1;
            biggest_idx = 3;
        }

        let biggest_val = f32_math::sqrt(four_biggest_sq_minus_1 + 1.0) * 0.5;
        let mult = 0.25 / biggest_val;

        match biggest_idx {
            0 => Self::from_wxyz(
                biggest_val,
                (m12 - m21) * mult,
                (m20 - m02) * mult,
                (m01 - m10) * mult,
            ),
            1 => Self::from_wxyz(
                (m12 - m21) * mult,
                biggest_val,
                (m01 + m10) * mult,
                (m20 + m02) * mult,
            ),
            2 => Self::from_wxyz(
                (m20 - m02) * mult,
                (m01 + m10) * mult,
                biggest_val,
                (m12 + m21) * mult,
            ),
            _ => Self::from_wxyz(
                (m01 - m10) * mult,
                (m20 + m02) * mult,
                (m12 + m21) * mult,
                biggest_val,
            ),
        }
    }

    #[inline]
    pub(crate) fn slerp(self, mut z: Self, a: f32) -> Self {
        let mut cos_theta = self.dot(z);
        if cos_theta < 0.0 {
            z = -z;
            cos_theta = -cos_theta;
        }
        if cos_theta > 0.9995 {
            return (self * (1.0 - a) + z * a).normalize();
        }
        let angle = f32_math::acos(cos_theta);
        (self * f32_math::sin((1.0 - a) * angle) + z * f32_math::sin(a * angle))
            / f32_math::sin(angle)
    }
}

impl Default for Quat {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for Quat {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.cmp_eq(other.0).all4()
    }
}

impl Deref for Quat {
    type Target = crate::deref::Vec4<f32>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(
            unsafe_code,
            reason = "Quat stores 4 contiguous f32 components [x, y, z, w]"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for Quat {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(
            unsafe_code,
            reason = "Quat stores 4 contiguous f32 components [x, y, z, w]"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(Quat))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[f32; 4]> for Quat {
    #[inline(always)]
    fn as_ref(&self) -> &[f32; 4] {
        #[expect(unsafe_code, reason = "Quat stores 4 contiguous f32 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f32; 4]>())
        }
    }
}

impl AsMut<[f32; 4]> for Quat {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        #[expect(unsafe_code, reason = "Quat stores 4 contiguous f32 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f32; 4]>())
        }
    }
}

impl Index<usize> for Quat {
    type Output = f32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Quat {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Quat {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }
}

impl AddAssign for Quat {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Quat {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0))
    }
}

impl SubAssign for Quat {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Quat {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0.mul(SimdF32x4::splat(rhs)))
    }
}

impl Mul<Quat> for f32 {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        rhs * self
    }
}

impl MulAssign<f32> for Quat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Self(self.0.div(SimdF32x4::splat(rhs)))
    }
}

impl DivAssign<f32> for Quat {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.quat_mul(rhs.0))
    }
}

impl MulAssign for Quat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        let [qx, qy, qz, qw] = self.to_array();
        let qv = Vec3::new(qx, qy, qz);
        let uv = qv.cross(rhs);
        let uuv = qv.cross(uv);
        rhs + ((uv * qw) + uuv) * 2.0
    }
}

impl Mul<Quat> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Quat) -> Vec3 {
        rhs.inverse() * self
    }
}

impl Mul<Vec3A> for Quat {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        let w = self.0.swizzle::<3, 3, 3, 3>();
        let two = SimdF32x4::splat(2.0);
        let uv = self.0.cross3(rhs.0);
        let uuv = self.0.cross3(uv);
        Vec3A(uv.mul_add(w, uuv).mul_add(two, rhs.0))
    }
}

impl Mul<Quat> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Quat) -> Vec3A {
        rhs.inverse() * self
    }
}

impl Quat {
    #[inline]
    pub(crate) fn from_mat3a(m: Mat3A) -> Self {
        Self::from_mat3(crate::Mat3::from_cols(
            crate::Vec3::from(m.x_axis),
            crate::Vec3::from(m.y_axis),
            crate::Vec3::from(m.z_axis),
        ))
    }

    #[inline]
    pub(crate) fn to_mat3a(self) -> Mat3A {
        let m = self.to_mat3();
        Mat3A::from_cols(
            Vec3A::from(m.x_axis),
            Vec3A::from(m.y_axis),
            Vec3A::from(m.z_axis),
        )
    }
}

impl From<[f32; 4]> for Quat {
    #[inline(always)]
    fn from(a: [f32; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<Quat> for [f32; 4] {
    #[inline(always)]
    fn from(q: Quat) -> Self {
        q.to_array()
    }
}
