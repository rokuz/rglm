use crate::Mat2;
use crate::Mat3;
use crate::Mat3x2;
use crate::Mat4x2;
use crate::Mat4x3;
use crate::Vec2;
use crate::Vec3;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Mat2x3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
}

impl Mat2x3 {
    pub const ZERO: Self = Self::from_cols(Vec3::ZERO, Vec3::ZERO);
    pub const NAN: Self = Self::from_cols(Vec3::NAN, Vec3::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: Vec3, y_axis: Vec3) -> Self {
        Self { x_axis, y_axis }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f32) -> Self {
        Self::from_cols(Vec3::new(s, 0.0, 0.0), Vec3::new(0.0, s, 0.0))
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> Vec3 {
        match idx {
            0 => self.x_axis,
            _ => self.y_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f32; 6]) -> Self {
        Self::from_cols(Vec3::new(m[0], m[1], m[2]), Vec3::new(m[3], m[4], m[5]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f32; 6] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        [c0[0], c0[1], c0[2], c1[0], c1[1], c1[2]]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f32; 3]; 2]) -> Self {
        Self::from_cols(Vec3::from_array(m[0]), Vec3::from_array(m[1]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f32; 3]; 2] {
        [self.col(0).to_array(), self.col(1).to_array()]
    }

    #[inline(always)]
    pub(crate) fn matrix_comp_mult(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis * rhs.x_axis, self.y_axis * rhs.y_axis)
    }

    #[inline(always)]
    pub(crate) fn transpose(self) -> Mat3x2 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        Mat3x2::from_cols(
            Vec2::new(c0[0], c1[0]),
            Vec2::new(c0[1], c1[1]),
            Vec2::new(c0[2], c1[2]),
        )
    }
}

impl Default for Mat2x3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for Mat2x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat2x3))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

impl fmt::Display for Mat2x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x_axis, self.y_axis)
    }
}

impl AsRef<[Vec3; 2]> for Mat2x3 {
    #[inline(always)]
    fn as_ref(&self) -> &[Vec3; 2] {
        #[expect(unsafe_code, reason = "Mat2x3 consists of 2 contiguous Vec3 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[Vec3; 2]>())
        }
    }
}

impl AsMut<[Vec3; 2]> for Mat2x3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [Vec3; 2] {
        #[expect(unsafe_code, reason = "Mat2x3 consists of 2 contiguous Vec3 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[Vec3; 2]>())
        }
    }
}

impl Index<usize> for Mat2x3 {
    type Output = Vec3;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Mat2x3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis + rhs.x_axis, self.y_axis + rhs.y_axis)
    }
}

impl Add<f32> for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs)
    }
}

impl Add<Mat2x3> for f32 {
    type Output = Mat2x3;
    #[inline(always)]
    fn add(self, rhs: Mat2x3) -> Mat2x3 {
        rhs + self
    }
}

impl AddAssign for Mat2x3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Mat2x3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis - rhs.x_axis, self.y_axis - rhs.y_axis)
    }
}

impl Sub<f32> for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs)
    }
}

impl Sub<Mat2x3> for f32 {
    type Output = Mat2x3;
    #[inline(always)]
    fn sub(self, rhs: Mat2x3) -> Mat2x3 {
        Mat2x3::from_cols(self - rhs.x_axis, self - rhs.y_axis)
    }
}

impl SubAssign for Mat2x3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Mat2x3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Neg for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis)
    }
}

impl Mul<f32> for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs)
    }
}

impl Mul<Mat2x3> for f32 {
    type Output = Mat2x3;
    #[inline(always)]
    fn mul(self, rhs: Mat2x3) -> Mat2x3 {
        rhs * self
    }
}

impl MulAssign<f32> for Mat2x3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Mat2x3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs)
    }
}

impl Div<Mat2x3> for f32 {
    type Output = Mat2x3;
    #[inline(always)]
    fn div(self, rhs: Mat2x3) -> Mat2x3 {
        Mat2x3::from_cols(self / rhs.x_axis, self / rhs.y_axis)
    }
}

impl DivAssign<f32> for Mat2x3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Vec2> for Mat2x3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec2) -> Vec3 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(Vec3::splat(r[1]), res);
        res
    }
}

impl Mul<Mat2x3> for Vec3 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, rhs: Mat2x3) -> Vec2 {
        Vec2::new(self.dot(rhs.x_axis), self.dot(rhs.y_axis))
    }
}

impl Mul<Mat2> for Mat2x3 {
    type Output = Mat2x3;
    #[inline]
    fn mul(self, rhs: Mat2) -> Mat2x3 {
        Mat2x3::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<Mat3x2> for Mat2x3 {
    type Output = Mat3;
    #[inline]
    fn mul(self, rhs: Mat3x2) -> Mat3 {
        Mat3::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<Mat4x2> for Mat2x3 {
    type Output = Mat4x3;
    #[inline]
    fn mul(self, rhs: Mat4x2) -> Mat4x3 {
        Mat4x3::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f32; 3]; 2]> for Mat2x3 {
    #[inline(always)]
    fn from(m: [[f32; 3]; 2]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<Mat2x3> for [[f32; 3]; 2] {
    #[inline(always)]
    fn from(m: Mat2x3) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f32; 6]> for Mat2x3 {
    #[inline(always)]
    fn from(m: [f32; 6]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<Mat2x3> for [f32; 6] {
    #[inline(always)]
    fn from(m: Mat2x3) -> Self {
        m.to_cols_array()
    }
}
