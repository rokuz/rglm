use crate::DMat2x4;
use crate::DMat3x4;
use crate::DVec4;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct DMat4 {
    pub x_axis: DVec4,
    pub y_axis: DVec4,
    pub z_axis: DVec4,
    pub w_axis: DVec4,
}

impl DMat4 {
    pub const ZERO: Self = Self::from_cols(DVec4::ZERO, DVec4::ZERO, DVec4::ZERO, DVec4::ZERO);
    pub const NAN: Self = Self::from_cols(DVec4::NAN, DVec4::NAN, DVec4::NAN, DVec4::NAN);
    pub const IDENTITY: Self = Self::diagonal(1.0);
    #[inline(always)]
    pub(crate) const fn from_cols(
        x_axis: DVec4,
        y_axis: DVec4,
        z_axis: DVec4,
        w_axis: DVec4,
    ) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f64) -> Self {
        Self::from_cols(
            DVec4::new(s, 0.0, 0.0, 0.0),
            DVec4::new(0.0, s, 0.0, 0.0),
            DVec4::new(0.0, 0.0, s, 0.0),
            DVec4::new(0.0, 0.0, 0.0, s),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> DVec4 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            2 => self.z_axis,
            _ => self.w_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f64; 16]) -> Self {
        Self::from_cols(
            DVec4::new(m[0], m[1], m[2], m[3]),
            DVec4::new(m[4], m[5], m[6], m[7]),
            DVec4::new(m[8], m[9], m[10], m[11]),
            DVec4::new(m[12], m[13], m[14], m[15]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f64; 16] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let c3 = self.col(3).to_array();
        [
            c0[0], c0[1], c0[2], c0[3], c1[0], c1[1], c1[2], c1[3], c2[0], c2[1], c2[2], c2[3],
            c3[0], c3[1], c3[2], c3[3],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f64; 4]; 4]) -> Self {
        Self::from_cols(
            DVec4::from_array(m[0]),
            DVec4::from_array(m[1]),
            DVec4::from_array(m[2]),
            DVec4::from_array(m[3]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f64; 4]; 4] {
        [
            self.col(0).to_array(),
            self.col(1).to_array(),
            self.col(2).to_array(),
            self.col(3).to_array(),
        ]
    }

    #[inline(always)]
    pub(crate) fn matrix_comp_mult(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis * rhs.x_axis,
            self.y_axis * rhs.y_axis,
            self.z_axis * rhs.z_axis,
            self.w_axis * rhs.w_axis,
        )
    }

    #[inline(always)]
    pub(crate) fn transpose(self) -> DMat4 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let c3 = self.col(3).to_array();
        DMat4::from_cols(
            DVec4::new(c0[0], c1[0], c2[0], c3[0]),
            DVec4::new(c0[1], c1[1], c2[1], c3[1]),
            DVec4::new(c0[2], c1[2], c2[2], c3[2]),
            DVec4::new(c0[3], c1[3], c2[3], c3[3]),
        )
    }

    #[inline]
    pub(crate) fn determinant(self) -> f64 {
        let m0 = self.x_axis.to_array();
        let m1 = self.y_axis.to_array();
        let m2 = self.z_axis.to_array();
        let m3 = self.w_axis.to_array();
        let s0 = m0[0] * m1[1] - m1[0] * m0[1];
        let s1 = m0[0] * m2[1] - m2[0] * m0[1];
        let s2 = m0[0] * m3[1] - m3[0] * m0[1];
        let s3 = m1[0] * m2[1] - m2[0] * m1[1];
        let s4 = m1[0] * m3[1] - m3[0] * m1[1];
        let s5 = m2[0] * m3[1] - m3[0] * m2[1];
        let k5 = m2[2] * m3[3] - m3[2] * m2[3];
        let k4 = m1[2] * m3[3] - m3[2] * m1[3];
        let k3 = m1[2] * m2[3] - m2[2] * m1[3];
        let k2 = m0[2] * m3[3] - m3[2] * m0[3];
        let k1 = m0[2] * m2[3] - m2[2] * m0[3];
        let k0 = m0[2] * m1[3] - m1[2] * m0[3];
        s0 * k5 - s1 * k4 + s2 * k3 + s3 * k2 - s4 * k1 + s5 * k0
    }

    #[inline]
    pub(crate) fn inverse(self) -> Self {
        let m0 = self.x_axis.to_array();
        let m1 = self.y_axis.to_array();
        let m2 = self.z_axis.to_array();
        let m3 = self.w_axis.to_array();
        let coef00 = m2[2] * m3[3] - m3[2] * m2[3];
        let coef02 = m1[2] * m3[3] - m3[2] * m1[3];
        let coef03 = m1[2] * m2[3] - m2[2] * m1[3];
        let coef04 = m2[1] * m3[3] - m3[1] * m2[3];
        let coef06 = m1[1] * m3[3] - m3[1] * m1[3];
        let coef07 = m1[1] * m2[3] - m2[1] * m1[3];
        let coef08 = m2[1] * m3[2] - m3[1] * m2[2];
        let coef10 = m1[1] * m3[2] - m3[1] * m1[2];
        let coef11 = m1[1] * m2[2] - m2[1] * m1[2];
        let coef12 = m2[0] * m3[3] - m3[0] * m2[3];
        let coef14 = m1[0] * m3[3] - m3[0] * m1[3];
        let coef15 = m1[0] * m2[3] - m2[0] * m1[3];
        let coef16 = m2[0] * m3[2] - m3[0] * m2[2];
        let coef18 = m1[0] * m3[2] - m3[0] * m1[2];
        let coef19 = m1[0] * m2[2] - m2[0] * m1[2];
        let coef20 = m2[0] * m3[1] - m3[0] * m2[1];
        let coef22 = m1[0] * m3[1] - m3[0] * m1[1];
        let coef23 = m1[0] * m2[1] - m2[0] * m1[1];

        let fac0 = DVec4::new(coef00, coef00, coef02, coef03);
        let fac1 = DVec4::new(coef04, coef04, coef06, coef07);
        let fac2 = DVec4::new(coef08, coef08, coef10, coef11);
        let fac3 = DVec4::new(coef12, coef12, coef14, coef15);
        let fac4 = DVec4::new(coef16, coef16, coef18, coef19);
        let fac5 = DVec4::new(coef20, coef20, coef22, coef23);

        let vec0 = DVec4::new(m1[0], m0[0], m0[0], m0[0]);
        let vec1 = DVec4::new(m1[1], m0[1], m0[1], m0[1]);
        let vec2 = DVec4::new(m1[2], m0[2], m0[2], m0[2]);
        let vec3 = DVec4::new(m1[3], m0[3], m0[3], m0[3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = DVec4::new(1.0, -1.0, 1.0, -1.0);
        let sign_b = DVec4::new(-1.0, 1.0, -1.0, 1.0);
        let c0_inv = inv0 * sign_a;
        let c1_inv = inv1 * sign_b;
        let c2_inv = inv2 * sign_a;
        let c3_inv = inv3 * sign_b;

        let row0 = DVec4::new(c0_inv.x, c1_inv.x, c2_inv.x, c3_inv.x);
        let det = self.x_axis.dot(row0);
        rglm_debug_assert!(det != 0.0);
        let inv_det = 1.0 / det;
        Self::from_cols(
            c0_inv * inv_det,
            c1_inv * inv_det,
            c2_inv * inv_det,
            c3_inv * inv_det,
        )
    }
}

impl Default for DMat4 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl fmt::Debug for DMat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(DMat4))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .field("w_axis", &self.w_axis)
            .finish()
    }
}

impl fmt::Display for DMat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.x_axis, self.y_axis, self.z_axis, self.w_axis
        )
    }
}

impl AsRef<[DVec4; 4]> for DMat4 {
    #[inline(always)]
    fn as_ref(&self) -> &[DVec4; 4] {
        #[expect(unsafe_code, reason = "DMat4 consists of 4 contiguous DVec4 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[DVec4; 4]>())
        }
    }
}

impl AsMut<[DVec4; 4]> for DMat4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [DVec4; 4] {
        #[expect(unsafe_code, reason = "DMat4 consists of 4 contiguous DVec4 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[DVec4; 4]>())
        }
    }
}

impl Index<usize> for DMat4 {
    type Output = DVec4;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DMat4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis + rhs.x_axis,
            self.y_axis + rhs.y_axis,
            self.z_axis + rhs.z_axis,
            self.w_axis + rhs.w_axis,
        )
    }
}

impl Add<f64> for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        Self::from_cols(
            self.x_axis + rhs,
            self.y_axis + rhs,
            self.z_axis + rhs,
            self.w_axis + rhs,
        )
    }
}

impl Add<DMat4> for f64 {
    type Output = DMat4;
    #[inline(always)]
    fn add(self, rhs: DMat4) -> DMat4 {
        rhs + self
    }
}

impl AddAssign for DMat4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DMat4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis - rhs.x_axis,
            self.y_axis - rhs.y_axis,
            self.z_axis - rhs.z_axis,
            self.w_axis - rhs.w_axis,
        )
    }
}

impl Sub<f64> for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        Self::from_cols(
            self.x_axis - rhs,
            self.y_axis - rhs,
            self.z_axis - rhs,
            self.w_axis - rhs,
        )
    }
}

impl Sub<DMat4> for f64 {
    type Output = DMat4;
    #[inline(always)]
    fn sub(self, rhs: DMat4) -> DMat4 {
        DMat4::from_cols(
            self - rhs.x_axis,
            self - rhs.y_axis,
            self - rhs.z_axis,
            self - rhs.w_axis,
        )
    }
}

impl SubAssign for DMat4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DMat4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Neg for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis, -self.w_axis)
    }
}

impl Mul<f64> for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self::from_cols(
            self.x_axis * rhs,
            self.y_axis * rhs,
            self.z_axis * rhs,
            self.w_axis * rhs,
        )
    }
}

impl Mul<DMat4> for f64 {
    type Output = DMat4;
    #[inline(always)]
    fn mul(self, rhs: DMat4) -> DMat4 {
        rhs * self
    }
}

impl MulAssign<f64> for DMat4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for DMat4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self::from_cols(
            self.x_axis / rhs,
            self.y_axis / rhs,
            self.z_axis / rhs,
            self.w_axis / rhs,
        )
    }
}

impl Div<DMat4> for f64 {
    type Output = DMat4;
    #[inline(always)]
    fn div(self, rhs: DMat4) -> DMat4 {
        DMat4::from_cols(
            self / rhs.x_axis,
            self / rhs.y_axis,
            self / rhs.z_axis,
            self / rhs.w_axis,
        )
    }
}

impl DivAssign<f64> for DMat4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<DVec4> for DMat4 {
    type Output = DVec4;
    #[inline(always)]
    fn mul(self, rhs: DVec4) -> DVec4 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(DVec4::splat(r[1]), res);
        res = self.z_axis.mul_add(DVec4::splat(r[2]), res);
        res = self.w_axis.mul_add(DVec4::splat(r[3]), res);
        res
    }
}

impl Mul<DMat4> for DVec4 {
    type Output = DVec4;
    #[inline(always)]
    fn mul(self, rhs: DMat4) -> DVec4 {
        DVec4::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
            self.dot(rhs.w_axis),
        )
    }
}

impl MulAssign<DMat4> for DVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: DMat4) {
        *self = *self * rhs;
    }
}

impl Mul<DMat2x4> for DMat4 {
    type Output = DMat2x4;
    #[inline]
    fn mul(self, rhs: DMat2x4) -> DMat2x4 {
        DMat2x4::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<DMat3x4> for DMat4 {
    type Output = DMat3x4;
    #[inline]
    fn mul(self, rhs: DMat3x4) -> DMat3x4 {
        DMat3x4::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<DMat4> for DMat4 {
    type Output = DMat4;
    #[inline]
    fn mul(self, rhs: DMat4) -> DMat4 {
        DMat4::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl MulAssign for DMat4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<[[f64; 4]; 4]> for DMat4 {
    #[inline(always)]
    fn from(m: [[f64; 4]; 4]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<DMat4> for [[f64; 4]; 4] {
    #[inline(always)]
    fn from(m: DMat4) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f64; 16]> for DMat4 {
    #[inline(always)]
    fn from(m: [f64; 16]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<DMat4> for [f64; 16] {
    #[inline(always)]
    fn from(m: DMat4) -> Self {
        m.to_cols_array()
    }
}
