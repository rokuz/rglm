use crate::Mat2x3;
use crate::Mat2x4;
use crate::Mat3;
use crate::Mat3x4;
use crate::Mat4;
use crate::Vec3;
use crate::Vec4;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Mat4x3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
    pub w_axis: Vec3,
}

impl Mat4x3 {
    pub const ZERO: Self = Self::from_cols(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
    pub const NAN: Self = Self::from_cols(Vec3::NAN, Vec3::NAN, Vec3::NAN, Vec3::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3, w_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f32) -> Self {
        Self::from_cols(
            Vec3::new(s, 0.0, 0.0),
            Vec3::new(0.0, s, 0.0),
            Vec3::new(0.0, 0.0, s),
            Vec3::new(0.0, 0.0, 0.0),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> Vec3 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            2 => self.z_axis,
            _ => self.w_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f32; 12]) -> Self {
        Self::from_cols(
            Vec3::new(m[0], m[1], m[2]),
            Vec3::new(m[3], m[4], m[5]),
            Vec3::new(m[6], m[7], m[8]),
            Vec3::new(m[9], m[10], m[11]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f32; 12] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let c3 = self.col(3).to_array();
        [
            c0[0], c0[1], c0[2], c1[0], c1[1], c1[2], c2[0], c2[1], c2[2], c3[0], c3[1], c3[2],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f32; 3]; 4]) -> Self {
        Self::from_cols(
            Vec3::from_array(m[0]),
            Vec3::from_array(m[1]),
            Vec3::from_array(m[2]),
            Vec3::from_array(m[3]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f32; 3]; 4] {
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
    pub(crate) fn transpose(self) -> Mat3x4 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let c3 = self.col(3).to_array();
        Mat3x4::from_cols(
            Vec4::new(c0[0], c1[0], c2[0], c3[0]),
            Vec4::new(c0[1], c1[1], c2[1], c3[1]),
            Vec4::new(c0[2], c1[2], c2[2], c3[2]),
        )
    }
}

impl Default for Mat4x3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for Mat4x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat4x3))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .field("w_axis", &self.w_axis)
            .finish()
    }
}

impl fmt::Display for Mat4x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.x_axis, self.y_axis, self.z_axis, self.w_axis
        )
    }
}

impl AsRef<[Vec3; 4]> for Mat4x3 {
    #[inline(always)]
    fn as_ref(&self) -> &[Vec3; 4] {
        #[expect(unsafe_code, reason = "Mat4x3 consists of 4 contiguous Vec3 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[Vec3; 4]>())
        }
    }
}

impl AsMut<[Vec3; 4]> for Mat4x3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [Vec3; 4] {
        #[expect(unsafe_code, reason = "Mat4x3 consists of 4 contiguous Vec3 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[Vec3; 4]>())
        }
    }
}

impl Index<usize> for Mat4x3 {
    type Output = Vec3;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Mat4x3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Mat4x3 {
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

impl Add<f32> for Mat4x3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        Self::from_cols(
            self.x_axis + rhs,
            self.y_axis + rhs,
            self.z_axis + rhs,
            self.w_axis + rhs,
        )
    }
}

impl Add<Mat4x3> for f32 {
    type Output = Mat4x3;
    #[inline(always)]
    fn add(self, rhs: Mat4x3) -> Mat4x3 {
        rhs + self
    }
}

impl AddAssign for Mat4x3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Mat4x3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Mat4x3 {
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

impl Sub<f32> for Mat4x3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        Self::from_cols(
            self.x_axis - rhs,
            self.y_axis - rhs,
            self.z_axis - rhs,
            self.w_axis - rhs,
        )
    }
}

impl Sub<Mat4x3> for f32 {
    type Output = Mat4x3;
    #[inline(always)]
    fn sub(self, rhs: Mat4x3) -> Mat4x3 {
        Mat4x3::from_cols(
            self - rhs.x_axis,
            self - rhs.y_axis,
            self - rhs.z_axis,
            self - rhs.w_axis,
        )
    }
}

impl SubAssign for Mat4x3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Mat4x3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Neg for Mat4x3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis, -self.w_axis)
    }
}

impl Mul<f32> for Mat4x3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self::from_cols(
            self.x_axis * rhs,
            self.y_axis * rhs,
            self.z_axis * rhs,
            self.w_axis * rhs,
        )
    }
}

impl Mul<Mat4x3> for f32 {
    type Output = Mat4x3;
    #[inline(always)]
    fn mul(self, rhs: Mat4x3) -> Mat4x3 {
        rhs * self
    }
}

impl MulAssign<f32> for Mat4x3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Mat4x3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Self::from_cols(
            self.x_axis / rhs,
            self.y_axis / rhs,
            self.z_axis / rhs,
            self.w_axis / rhs,
        )
    }
}

impl Div<Mat4x3> for f32 {
    type Output = Mat4x3;
    #[inline(always)]
    fn div(self, rhs: Mat4x3) -> Mat4x3 {
        Mat4x3::from_cols(
            self / rhs.x_axis,
            self / rhs.y_axis,
            self / rhs.z_axis,
            self / rhs.w_axis,
        )
    }
}

impl DivAssign<f32> for Mat4x3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Vec4> for Mat4x3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec4) -> Vec3 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(Vec3::splat(r[1]), res);
        res = self.z_axis.mul_add(Vec3::splat(r[2]), res);
        res = self.w_axis.mul_add(Vec3::splat(r[3]), res);
        res
    }
}

impl Mul<Mat4x3> for Vec3 {
    type Output = Vec4;
    #[inline(always)]
    fn mul(self, rhs: Mat4x3) -> Vec4 {
        Vec4::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
            self.dot(rhs.w_axis),
        )
    }
}

impl Mul<Mat2x4> for Mat4x3 {
    type Output = Mat2x3;
    #[inline]
    fn mul(self, rhs: Mat2x4) -> Mat2x3 {
        Mat2x3::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<Mat3x4> for Mat4x3 {
    type Output = Mat3;
    #[inline]
    fn mul(self, rhs: Mat3x4) -> Mat3 {
        Mat3::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<Mat4> for Mat4x3 {
    type Output = Mat4x3;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4x3 {
        Mat4x3::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f32; 3]; 4]> for Mat4x3 {
    #[inline(always)]
    fn from(m: [[f32; 3]; 4]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<Mat4x3> for [[f32; 3]; 4] {
    #[inline(always)]
    fn from(m: Mat4x3) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f32; 12]> for Mat4x3 {
    #[inline(always)]
    fn from(m: [f32; 12]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<Mat4x3> for [f32; 12] {
    #[inline(always)]
    fn from(m: Mat4x3) -> Self {
        m.to_cols_array()
    }
}
