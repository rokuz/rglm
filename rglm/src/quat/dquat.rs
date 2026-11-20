use crate::math::f64_math;
use crate::simd::SimdF64x2;
use crate::{DMat3, DMat4, DVec3, DVec4};
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub struct DQuat(pub(crate) [SimdF64x2; 2]);

impl DQuat {
    pub const ZERO: Self = Self::from_xyzw(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);
    pub const NAN: Self = Self::from_xyzw(f64::NAN, f64::NAN, f64::NAN, f64::NAN);

    #[inline(always)]
    pub(crate) const fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self([SimdF64x2::new(x, y), SimdF64x2::new(z, w)])
    }

    #[inline(always)]
    pub(crate) const fn from_wxyz(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self::from_xyzw(x, y, z, w)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f64; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f64; 4] {
        let a0 = self.0[0].to_array();
        let a1 = self.0[1].to_array();
        [a0[0], a0[1], a1[0], a1[1]]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f64 {
        self.0[0].dot2(rhs.0[0]) + self.0[1].dot2(rhs.0[1])
    }

    #[inline(always)]
    pub(crate) fn length_squared(self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub(crate) fn length(self) -> f64 {
        f64_math::sqrt(self.dot(self))
    }

    #[inline(always)]
    pub(crate) fn normalize(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > 0.0);
        self * f64_math::inversesqrt(len_sq)
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
    pub(crate) fn from_axis_angle(axis: DVec3, angle: f64) -> Self {
        let half = angle * 0.5;
        let (s, c) = f64_math::sin_cos(half);
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    #[inline]
    pub(crate) fn to_mat3(self) -> DMat3 {
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
        DMat3::from_cols(
            DVec3::new(one - (yy + zz), xy + wz, xz - wy),
            DVec3::new(xy - wz, one - (xx + zz), yz + wx),
            DVec3::new(xz + wy, yz - wx, one - (xx + yy)),
        )
    }

    #[inline]
    pub(crate) fn to_mat4(self) -> DMat4 {
        let m = self.to_mat3();
        let zero = 0.0;
        let one = 1.0;
        DMat4::from_cols(
            DVec4::new(m.x_axis.x, m.x_axis.y, m.x_axis.z, zero),
            DVec4::new(m.y_axis.x, m.y_axis.y, m.y_axis.z, zero),
            DVec4::new(m.z_axis.x, m.z_axis.y, m.z_axis.z, zero),
            DVec4::new(zero, zero, zero, one),
        )
    }

    #[inline]
    pub(crate) fn from_mat3(m: DMat3) -> Self {
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

        let biggest_val = f64_math::sqrt(four_biggest_sq_minus_1 + 1.0) * 0.5;
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
    pub(crate) fn slerp(self, mut z: Self, a: f64) -> Self {
        let mut cos_theta = self.dot(z);
        if cos_theta < 0.0 {
            z = -z;
            cos_theta = -cos_theta;
        }
        if cos_theta > 0.9999995 {
            return (self * (1.0 - a) + z * a).normalize();
        }
        let angle = f64_math::acos(cos_theta);
        (self * f64_math::sin((1.0 - a) * angle) + z * f64_math::sin(a * angle))
            / f64_math::sin(angle)
    }
}

impl Default for DQuat {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for DQuat {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_array() == other.to_array()
    }
}

impl Deref for DQuat {
    type Target = crate::deref::Vec4<f64>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(
            unsafe_code,
            reason = "DQuat stores 4 contiguous f64 components [x, y, z, w]"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for DQuat {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(
            unsafe_code,
            reason = "DQuat stores 4 contiguous f64 components [x, y, z, w]"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for DQuat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(DQuat))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for DQuat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[f64; 4]> for DQuat {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 4] {
        #[expect(unsafe_code, reason = "DQuat stores 4 contiguous f64 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f64; 4]>())
        }
    }
}

impl AsMut<[f64; 4]> for DQuat {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 4] {
        #[expect(unsafe_code, reason = "DQuat stores 4 contiguous f64 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f64; 4]>())
        }
    }
}

impl Index<usize> for DQuat {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DQuat {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DQuat {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self([self.0[0].add(rhs.0[0]), self.0[1].add(rhs.0[1])])
    }
}

impl AddAssign for DQuat {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for DQuat {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self([self.0[0].sub(rhs.0[0]), self.0[1].sub(rhs.0[1])])
    }
}

impl SubAssign for DQuat {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for DQuat {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self([self.0[0].neg(), self.0[1].neg()])
    }
}

impl Mul<f64> for DQuat {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        let s = SimdF64x2::splat(rhs);
        Self([self.0[0].mul(s), self.0[1].mul(s)])
    }
}

impl Mul<DQuat> for f64 {
    type Output = DQuat;
    #[inline(always)]
    fn mul(self, rhs: DQuat) -> DQuat {
        rhs * self
    }
}

impl MulAssign<f64> for DQuat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for DQuat {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        let s = SimdF64x2::splat(rhs);
        Self([self.0[0].div(s), self.0[1].div(s)])
    }
}

impl DivAssign<f64> for DQuat {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul for DQuat {
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

impl MulAssign for DQuat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<DVec3> for DQuat {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: DVec3) -> DVec3 {
        let [qx, qy, qz, qw] = self.to_array();
        let qv = DVec3::new(qx, qy, qz);
        let uv = qv.cross(rhs);
        let uuv = qv.cross(uv);
        rhs + ((uv * qw) + uuv) * 2.0
    }
}

impl Mul<DQuat> for DVec3 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: DQuat) -> DVec3 {
        rhs.inverse() * self
    }
}

impl From<[f64; 4]> for DQuat {
    #[inline(always)]
    fn from(a: [f64; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<DQuat> for [f64; 4] {
    #[inline(always)]
    fn from(q: DQuat) -> Self {
        q.to_array()
    }
}
