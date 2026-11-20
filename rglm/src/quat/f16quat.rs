use crate::math::f16_math;
use crate::{F16Mat3, F16Mat4, F16Vec3, F16Vec4};
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use half::f16;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct F16Quat {
    pub x: f16,
    pub y: f16,
    pub z: f16,
    pub w: f16,
}

impl F16Quat {
    pub const ZERO: Self = Self::from_xyzw(f16::ZERO, f16::ZERO, f16::ZERO, f16::ZERO);
    pub const IDENTITY: Self = Self::from_xyzw(f16::ZERO, f16::ZERO, f16::ZERO, f16::ONE);
    pub const NAN: Self = Self::from_xyzw(f16::NAN, f16::NAN, f16::NAN, f16::NAN);

    #[inline(always)]
    pub(crate) const fn from_xyzw(x: f16, y: f16, z: f16, w: f16) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    pub(crate) const fn from_wxyz(w: f16, x: f16, y: f16, z: f16) -> Self {
        Self::from_xyzw(x, y, z, w)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f16; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f16; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f16 {
        (self.x * rhs.x + self.z * rhs.z) + (self.y * rhs.y + self.w * rhs.w)
    }

    #[inline(always)]
    pub(crate) fn length_squared(self) -> f16 {
        self.dot(self)
    }

    #[inline(always)]
    pub(crate) fn length(self) -> f16 {
        f16_math::sqrt(self.dot(self))
    }

    #[inline(always)]
    pub(crate) fn normalize(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > f16::ZERO);
        self * f16_math::inversesqrt(len_sq)
    }

    #[inline(always)]
    pub(crate) fn conjugate(self) -> Self {
        Self::from_xyzw(-self.x, -self.y, -self.z, self.w)
    }

    #[inline(always)]
    pub(crate) fn inverse(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > f16::ZERO);
        self.conjugate() / len_sq
    }

    #[inline]
    pub(crate) fn from_axis_angle(axis: F16Vec3, angle: f16) -> Self {
        let half = angle * f16::from_f32_const(0.5);
        let (s, c) = f16_math::sin_cos(half);
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    #[inline]
    pub(crate) fn to_mat3(self) -> F16Mat3 {
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
        let one = f16::ONE;
        F16Mat3::from_cols(
            F16Vec3::new(one - (yy + zz), xy + wz, xz - wy),
            F16Vec3::new(xy - wz, one - (xx + zz), yz + wx),
            F16Vec3::new(xz + wy, yz - wx, one - (xx + yy)),
        )
    }

    #[inline]
    pub(crate) fn to_mat4(self) -> F16Mat4 {
        let m = self.to_mat3();
        let zero = f16::ZERO;
        let one = f16::ONE;
        F16Mat4::from_cols(
            F16Vec4::new(m.x_axis.x, m.x_axis.y, m.x_axis.z, zero),
            F16Vec4::new(m.y_axis.x, m.y_axis.y, m.y_axis.z, zero),
            F16Vec4::new(m.z_axis.x, m.z_axis.y, m.z_axis.z, zero),
            F16Vec4::new(zero, zero, zero, one),
        )
    }

    #[inline]
    pub(crate) fn from_mat3(m: F16Mat3) -> Self {
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

        let biggest_val =
            f16_math::sqrt(four_biggest_sq_minus_1 + f16::ONE) * f16::from_f32_const(0.5);
        let mult = f16::from_f32_const(0.25) / biggest_val;

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
    pub(crate) fn slerp(self, mut z: Self, a: f16) -> Self {
        let mut cos_theta = self.dot(z);
        if cos_theta < f16::ZERO {
            z = -z;
            cos_theta = -cos_theta;
        }
        if cos_theta > f16::from_f32_const(0.995) {
            return (self * (f16::ONE - a) + z * a).normalize();
        }
        let angle = f16_math::acos(cos_theta);
        (self * f16_math::sin((f16::ONE - a) * angle) + z * f16_math::sin(a * angle))
            / f16_math::sin(angle)
    }
}

impl Default for F16Quat {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl fmt::Debug for F16Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(F16Quat))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for F16Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[f16; 4]> for F16Quat {
    #[inline(always)]
    fn as_ref(&self) -> &[f16; 4] {
        #[expect(unsafe_code, reason = "F16Quat stores 4 contiguous f16 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f16; 4]>())
        }
    }
}

impl AsMut<[f16; 4]> for F16Quat {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f16; 4] {
        #[expect(unsafe_code, reason = "F16Quat stores 4 contiguous f16 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f16; 4]>())
        }
    }
}

impl Index<usize> for F16Quat {
    type Output = f16;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for F16Quat {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for F16Quat {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_xyzw(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl AddAssign for F16Quat {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for F16Quat {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_xyzw(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl SubAssign for F16Quat {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for F16Quat {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_xyzw(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f16> for F16Quat {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f16) -> Self {
        Self::from_xyzw(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul<F16Quat> for f16 {
    type Output = F16Quat;
    #[inline(always)]
    fn mul(self, rhs: F16Quat) -> F16Quat {
        rhs * self
    }
}

impl MulAssign<f16> for F16Quat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f16) {
        *self = *self * rhs;
    }
}

impl Div<f16> for F16Quat {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f16) -> Self {
        Self::from_xyzw(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl DivAssign<f16> for F16Quat {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f16) {
        *self = *self / rhs;
    }
}

impl Mul for F16Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let [lx, ly, lz, lw] = self.to_array();
        let [rx, ry, rz, rw] = rhs.to_array();
        Self::from_xyzw(
            lw * rx + lx * rw + ly * rz - lz * ry,
            lw * ry - lx * rz + ly * rw + lz * rx,
            lw * rz + lx * ry - ly * rx + lz * rw,
            lw * rw - lx * rx - ly * ry - lz * rz,
        )
    }
}

impl MulAssign for F16Quat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<F16Vec3> for F16Quat {
    type Output = F16Vec3;
    #[inline]
    fn mul(self, rhs: F16Vec3) -> F16Vec3 {
        let [qx, qy, qz, qw] = self.to_array();
        let qv = F16Vec3::new(qx, qy, qz);
        let uv = qv.cross(rhs);
        let uuv = qv.cross(uv);
        rhs + ((uv * qw) + uuv) * f16::from_f32_const(2.0)
    }
}

impl Mul<F16Quat> for F16Vec3 {
    type Output = F16Vec3;
    #[inline]
    fn mul(self, rhs: F16Quat) -> F16Vec3 {
        rhs.inverse() * self
    }
}

impl From<[f16; 4]> for F16Quat {
    #[inline(always)]
    fn from(a: [f16; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<F16Quat> for [f16; 4] {
    #[inline(always)]
    fn from(q: F16Quat) -> Self {
        q.to_array()
    }
}
