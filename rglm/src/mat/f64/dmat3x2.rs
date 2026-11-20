use crate::DMat2;
use crate::DMat2x3;
use crate::DMat3;
use crate::DMat4x2;
use crate::DMat4x3;
use crate::DVec2;
use crate::DVec3;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct DMat3x2 {
    pub x_axis: DVec2,
    pub y_axis: DVec2,
    pub z_axis: DVec2,
}

impl DMat3x2 {
    pub const ZERO: Self = Self::from_cols(DVec2::ZERO, DVec2::ZERO, DVec2::ZERO);
    pub const NAN: Self = Self::from_cols(DVec2::NAN, DVec2::NAN, DVec2::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: DVec2, y_axis: DVec2, z_axis: DVec2) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f64) -> Self {
        Self::from_cols(DVec2::new(s, 0.0), DVec2::new(0.0, s), DVec2::new(0.0, 0.0))
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> DVec2 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => self.z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f64; 6]) -> Self {
        Self::from_cols(
            DVec2::new(m[0], m[1]),
            DVec2::new(m[2], m[3]),
            DVec2::new(m[4], m[5]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f64; 6] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        [c0[0], c0[1], c1[0], c1[1], c2[0], c2[1]]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f64; 2]; 3]) -> Self {
        Self::from_cols(
            DVec2::from_array(m[0]),
            DVec2::from_array(m[1]),
            DVec2::from_array(m[2]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f64; 2]; 3] {
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
    pub(crate) fn transpose(self) -> DMat2x3 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        DMat2x3::from_cols(
            DVec3::new(c0[0], c1[0], c2[0]),
            DVec3::new(c0[1], c1[1], c2[1]),
        )
    }
}

impl Default for DMat3x2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for DMat3x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(DMat3x2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for DMat3x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl AsRef<[DVec2; 3]> for DMat3x2 {
    #[inline(always)]
    fn as_ref(&self) -> &[DVec2; 3] {
        #[expect(unsafe_code, reason = "DMat3x2 consists of 3 contiguous DVec2 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[DVec2; 3]>())
        }
    }
}

impl AsMut<[DVec2; 3]> for DMat3x2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [DVec2; 3] {
        #[expect(unsafe_code, reason = "DMat3x2 consists of 3 contiguous DVec2 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[DVec2; 3]>())
        }
    }
}

impl Index<usize> for DMat3x2 {
    type Output = DVec2;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DMat3x2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DMat3x2 {
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

impl Add<f64> for DMat3x2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs, self.z_axis + rhs)
    }
}

impl Add<DMat3x2> for f64 {
    type Output = DMat3x2;
    #[inline(always)]
    fn add(self, rhs: DMat3x2) -> DMat3x2 {
        rhs + self
    }
}

impl AddAssign for DMat3x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DMat3x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DMat3x2 {
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

impl Sub<f64> for DMat3x2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs, self.z_axis - rhs)
    }
}

impl Sub<DMat3x2> for f64 {
    type Output = DMat3x2;
    #[inline(always)]
    fn sub(self, rhs: DMat3x2) -> DMat3x2 {
        DMat3x2::from_cols(self - rhs.x_axis, self - rhs.y_axis, self - rhs.z_axis)
    }
}

impl SubAssign for DMat3x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DMat3x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Neg for DMat3x2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis)
    }
}

impl Mul<f64> for DMat3x2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs, self.z_axis * rhs)
    }
}

impl Mul<DMat3x2> for f64 {
    type Output = DMat3x2;
    #[inline(always)]
    fn mul(self, rhs: DMat3x2) -> DMat3x2 {
        rhs * self
    }
}

impl MulAssign<f64> for DMat3x2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for DMat3x2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs, self.z_axis / rhs)
    }
}

impl Div<DMat3x2> for f64 {
    type Output = DMat3x2;
    #[inline(always)]
    fn div(self, rhs: DMat3x2) -> DMat3x2 {
        DMat3x2::from_cols(self / rhs.x_axis, self / rhs.y_axis, self / rhs.z_axis)
    }
}

impl DivAssign<f64> for DMat3x2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<DVec3> for DMat3x2 {
    type Output = DVec2;
    #[inline(always)]
    fn mul(self, rhs: DVec3) -> DVec2 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(DVec2::splat(r[1]), res);
        res = self.z_axis.mul_add(DVec2::splat(r[2]), res);
        res
    }
}

impl Mul<DMat3x2> for DVec2 {
    type Output = DVec3;
    #[inline(always)]
    fn mul(self, rhs: DMat3x2) -> DVec3 {
        DVec3::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
        )
    }
}

impl Mul<DMat2x3> for DMat3x2 {
    type Output = DMat2;
    #[inline]
    fn mul(self, rhs: DMat2x3) -> DMat2 {
        DMat2::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<DMat3> for DMat3x2 {
    type Output = DMat3x2;
    #[inline]
    fn mul(self, rhs: DMat3) -> DMat3x2 {
        DMat3x2::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<DMat4x3> for DMat3x2 {
    type Output = DMat4x2;
    #[inline]
    fn mul(self, rhs: DMat4x3) -> DMat4x2 {
        DMat4x2::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f64; 2]; 3]> for DMat3x2 {
    #[inline(always)]
    fn from(m: [[f64; 2]; 3]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<DMat3x2> for [[f64; 2]; 3] {
    #[inline(always)]
    fn from(m: DMat3x2) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f64; 6]> for DMat3x2 {
    #[inline(always)]
    fn from(m: [f64; 6]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<DMat3x2> for [f64; 6] {
    #[inline(always)]
    fn from(m: DMat3x2) -> Self {
        m.to_cols_array()
    }
}
