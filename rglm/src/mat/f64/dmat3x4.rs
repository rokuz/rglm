use crate::DMat2x3;
use crate::DMat2x4;
use crate::DMat3;
use crate::DMat4;
use crate::DMat4x3;
use crate::DVec3;
use crate::DVec4;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct DMat3x4 {
    pub x_axis: DVec4,
    pub y_axis: DVec4,
    pub z_axis: DVec4,
}

impl DMat3x4 {
    pub const ZERO: Self = Self::from_cols(DVec4::ZERO, DVec4::ZERO, DVec4::ZERO);
    pub const NAN: Self = Self::from_cols(DVec4::NAN, DVec4::NAN, DVec4::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: DVec4, y_axis: DVec4, z_axis: DVec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f64) -> Self {
        Self::from_cols(
            DVec4::new(s, 0.0, 0.0, 0.0),
            DVec4::new(0.0, s, 0.0, 0.0),
            DVec4::new(0.0, 0.0, s, 0.0),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> DVec4 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => self.z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f64; 12]) -> Self {
        Self::from_cols(
            DVec4::new(m[0], m[1], m[2], m[3]),
            DVec4::new(m[4], m[5], m[6], m[7]),
            DVec4::new(m[8], m[9], m[10], m[11]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f64; 12] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        [
            c0[0], c0[1], c0[2], c0[3], c1[0], c1[1], c1[2], c1[3], c2[0], c2[1], c2[2], c2[3],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f64; 4]; 3]) -> Self {
        Self::from_cols(
            DVec4::from_array(m[0]),
            DVec4::from_array(m[1]),
            DVec4::from_array(m[2]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f64; 4]; 3] {
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
    pub(crate) fn transpose(self) -> DMat4x3 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        DMat4x3::from_cols(
            DVec3::new(c0[0], c1[0], c2[0]),
            DVec3::new(c0[1], c1[1], c2[1]),
            DVec3::new(c0[2], c1[2], c2[2]),
            DVec3::new(c0[3], c1[3], c2[3]),
        )
    }
}

impl Default for DMat3x4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for DMat3x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(DMat3x4))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for DMat3x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl AsRef<[DVec4; 3]> for DMat3x4 {
    #[inline(always)]
    fn as_ref(&self) -> &[DVec4; 3] {
        #[expect(unsafe_code, reason = "DMat3x4 consists of 3 contiguous DVec4 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[DVec4; 3]>())
        }
    }
}

impl AsMut<[DVec4; 3]> for DMat3x4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [DVec4; 3] {
        #[expect(unsafe_code, reason = "DMat3x4 consists of 3 contiguous DVec4 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[DVec4; 3]>())
        }
    }
}

impl Index<usize> for DMat3x4 {
    type Output = DVec4;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DMat3x4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DMat3x4 {
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

impl Add<f64> for DMat3x4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs, self.z_axis + rhs)
    }
}

impl Add<DMat3x4> for f64 {
    type Output = DMat3x4;
    #[inline(always)]
    fn add(self, rhs: DMat3x4) -> DMat3x4 {
        rhs + self
    }
}

impl AddAssign for DMat3x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DMat3x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DMat3x4 {
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

impl Sub<f64> for DMat3x4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs, self.z_axis - rhs)
    }
}

impl Sub<DMat3x4> for f64 {
    type Output = DMat3x4;
    #[inline(always)]
    fn sub(self, rhs: DMat3x4) -> DMat3x4 {
        DMat3x4::from_cols(self - rhs.x_axis, self - rhs.y_axis, self - rhs.z_axis)
    }
}

impl SubAssign for DMat3x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DMat3x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Neg for DMat3x4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis)
    }
}

impl Mul<f64> for DMat3x4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs, self.z_axis * rhs)
    }
}

impl Mul<DMat3x4> for f64 {
    type Output = DMat3x4;
    #[inline(always)]
    fn mul(self, rhs: DMat3x4) -> DMat3x4 {
        rhs * self
    }
}

impl MulAssign<f64> for DMat3x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for DMat3x4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs, self.z_axis / rhs)
    }
}

impl Div<DMat3x4> for f64 {
    type Output = DMat3x4;
    #[inline(always)]
    fn div(self, rhs: DMat3x4) -> DMat3x4 {
        DMat3x4::from_cols(self / rhs.x_axis, self / rhs.y_axis, self / rhs.z_axis)
    }
}

impl DivAssign<f64> for DMat3x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<DVec3> for DMat3x4 {
    type Output = DVec4;
    #[inline(always)]
    fn mul(self, rhs: DVec3) -> DVec4 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(DVec4::splat(r[1]), res);
        res = self.z_axis.mul_add(DVec4::splat(r[2]), res);
        res
    }
}

impl Mul<DMat3x4> for DVec4 {
    type Output = DVec3;
    #[inline(always)]
    fn mul(self, rhs: DMat3x4) -> DVec3 {
        DVec3::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
        )
    }
}

impl Mul<DMat2x3> for DMat3x4 {
    type Output = DMat2x4;
    #[inline]
    fn mul(self, rhs: DMat2x3) -> DMat2x4 {
        DMat2x4::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<DMat3> for DMat3x4 {
    type Output = DMat3x4;
    #[inline]
    fn mul(self, rhs: DMat3) -> DMat3x4 {
        DMat3x4::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<DMat4x3> for DMat3x4 {
    type Output = DMat4;
    #[inline]
    fn mul(self, rhs: DMat4x3) -> DMat4 {
        DMat4::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f64; 4]; 3]> for DMat3x4 {
    #[inline(always)]
    fn from(m: [[f64; 4]; 3]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<DMat3x4> for [[f64; 4]; 3] {
    #[inline(always)]
    fn from(m: DMat3x4) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f64; 12]> for DMat3x4 {
    #[inline(always)]
    fn from(m: [f64; 12]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<DMat3x4> for [f64; 12] {
    #[inline(always)]
    fn from(m: DMat3x4) -> Self {
        m.to_cols_array()
    }
}
