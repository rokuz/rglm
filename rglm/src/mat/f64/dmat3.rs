use crate::DMat2x3;
use crate::DMat4x3;
use crate::DVec3;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct DMat3 {
    pub x_axis: DVec3,
    pub y_axis: DVec3,
    pub z_axis: DVec3,
}

impl DMat3 {
    pub const ZERO: Self = Self::from_cols(DVec3::ZERO, DVec3::ZERO, DVec3::ZERO);
    pub const NAN: Self = Self::from_cols(DVec3::NAN, DVec3::NAN, DVec3::NAN);
    pub const IDENTITY: Self = Self::diagonal(1.0);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f64) -> Self {
        Self::from_cols(
            DVec3::new(s, 0.0, 0.0),
            DVec3::new(0.0, s, 0.0),
            DVec3::new(0.0, 0.0, s),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> DVec3 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => self.z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f64; 9]) -> Self {
        Self::from_cols(
            DVec3::new(m[0], m[1], m[2]),
            DVec3::new(m[3], m[4], m[5]),
            DVec3::new(m[6], m[7], m[8]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f64; 9] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        [
            c0[0], c0[1], c0[2], c1[0], c1[1], c1[2], c2[0], c2[1], c2[2],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f64; 3]; 3]) -> Self {
        Self::from_cols(
            DVec3::from_array(m[0]),
            DVec3::from_array(m[1]),
            DVec3::from_array(m[2]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f64; 3]; 3] {
        [
            self.col(0).to_array(),
            self.col(1).to_array(),
            self.col(2).to_array(),
        ]
    }

    #[inline(always)]
    pub(crate) fn matrix_comp_mult(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis * rhs.x_axis,
            self.y_axis * rhs.y_axis,
            self.z_axis * rhs.z_axis,
        )
    }

    #[inline(always)]
    pub(crate) fn transpose(self) -> DMat3 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        DMat3::from_cols(
            DVec3::new(c0[0], c1[0], c2[0]),
            DVec3::new(c0[1], c1[1], c2[1]),
            DVec3::new(c0[2], c1[2], c2[2]),
        )
    }

    #[inline]
    pub(crate) fn determinant(self) -> f64 {
        self.x_axis.dot(self.y_axis.cross(self.z_axis))
    }

    #[inline]
    pub(crate) fn inverse(self) -> Self {
        let k0 = self.y_axis.cross(self.z_axis);
        let k1 = self.z_axis.cross(self.x_axis);
        let k2 = self.x_axis.cross(self.y_axis);
        let det = self.x_axis.dot(k0);
        rglm_debug_assert!(det != 0.0);
        let inv_det = 1.0 / det;
        Self::from_cols(k0 * inv_det, k1 * inv_det, k2 * inv_det).transpose()
    }
}

impl Default for DMat3 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl fmt::Debug for DMat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(DMat3))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for DMat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl AsRef<[DVec3; 3]> for DMat3 {
    #[inline(always)]
    fn as_ref(&self) -> &[DVec3; 3] {
        #[expect(unsafe_code, reason = "DMat3 consists of 3 contiguous DVec3 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[DVec3; 3]>())
        }
    }
}

impl AsMut<[DVec3; 3]> for DMat3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [DVec3; 3] {
        #[expect(unsafe_code, reason = "DMat3 consists of 3 contiguous DVec3 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[DVec3; 3]>())
        }
    }
}

impl Index<usize> for DMat3 {
    type Output = DVec3;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DMat3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis + rhs.x_axis,
            self.y_axis + rhs.y_axis,
            self.z_axis + rhs.z_axis,
        )
    }
}

impl Add<f64> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs, self.z_axis + rhs)
    }
}

impl Add<DMat3> for f64 {
    type Output = DMat3;
    #[inline(always)]
    fn add(self, rhs: DMat3) -> DMat3 {
        rhs + self
    }
}

impl AddAssign for DMat3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DMat3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis - rhs.x_axis,
            self.y_axis - rhs.y_axis,
            self.z_axis - rhs.z_axis,
        )
    }
}

impl Sub<f64> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs, self.z_axis - rhs)
    }
}

impl Sub<DMat3> for f64 {
    type Output = DMat3;
    #[inline(always)]
    fn sub(self, rhs: DMat3) -> DMat3 {
        DMat3::from_cols(self - rhs.x_axis, self - rhs.y_axis, self - rhs.z_axis)
    }
}

impl SubAssign for DMat3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DMat3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Neg for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis)
    }
}

impl Mul<f64> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs, self.z_axis * rhs)
    }
}

impl Mul<DMat3> for f64 {
    type Output = DMat3;
    #[inline(always)]
    fn mul(self, rhs: DMat3) -> DMat3 {
        rhs * self
    }
}

impl MulAssign<f64> for DMat3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs, self.z_axis / rhs)
    }
}

impl Div<DMat3> for f64 {
    type Output = DMat3;
    #[inline(always)]
    fn div(self, rhs: DMat3) -> DMat3 {
        DMat3::from_cols(self / rhs.x_axis, self / rhs.y_axis, self / rhs.z_axis)
    }
}

impl DivAssign<f64> for DMat3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<DVec3> for DMat3 {
    type Output = DVec3;
    #[inline(always)]
    fn mul(self, rhs: DVec3) -> DVec3 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(DVec3::splat(r[1]), res);
        res = self.z_axis.mul_add(DVec3::splat(r[2]), res);
        res
    }
}

impl Mul<DMat3> for DVec3 {
    type Output = DVec3;
    #[inline(always)]
    fn mul(self, rhs: DMat3) -> DVec3 {
        DVec3::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
        )
    }
}

impl MulAssign<DMat3> for DVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: DMat3) {
        *self = *self * rhs;
    }
}

impl Mul<DMat2x3> for DMat3 {
    type Output = DMat2x3;
    #[inline]
    fn mul(self, rhs: DMat2x3) -> DMat2x3 {
        DMat2x3::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<DMat3> for DMat3 {
    type Output = DMat3;
    #[inline]
    fn mul(self, rhs: DMat3) -> DMat3 {
        DMat3::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<DMat4x3> for DMat3 {
    type Output = DMat4x3;
    #[inline]
    fn mul(self, rhs: DMat4x3) -> DMat4x3 {
        DMat4x3::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl MulAssign for DMat3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<[[f64; 3]; 3]> for DMat3 {
    #[inline(always)]
    fn from(m: [[f64; 3]; 3]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<DMat3> for [[f64; 3]; 3] {
    #[inline(always)]
    fn from(m: DMat3) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f64; 9]> for DMat3 {
    #[inline(always)]
    fn from(m: [f64; 9]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<DMat3> for [f64; 9] {
    #[inline(always)]
    fn from(m: DMat3) -> Self {
        m.to_cols_array()
    }
}
