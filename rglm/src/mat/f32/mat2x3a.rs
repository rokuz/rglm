use crate::Mat2;
use crate::Mat3A;
use crate::Mat3x2;
use crate::Mat4x2;
use crate::Mat4x3A;
use crate::Vec2;
use crate::Vec3A;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Mat2x3A {
    pub x_axis: Vec3A,
    pub y_axis: Vec3A,
}

impl Mat2x3A {
    pub const ZERO: Self = Self::from_cols(Vec3A::ZERO, Vec3A::ZERO);
    pub const NAN: Self = Self::from_cols(Vec3A::NAN, Vec3A::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: Vec3A, y_axis: Vec3A) -> Self {
        Self { x_axis, y_axis }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f32) -> Self {
        Self::from_cols(Vec3A::new(s, 0.0, 0.0), Vec3A::new(0.0, s, 0.0))
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> Vec3A {
        match idx {
            0 => self.x_axis,
            _ => self.y_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f32; 6]) -> Self {
        Self::from_cols(Vec3A::new(m[0], m[1], m[2]), Vec3A::new(m[3], m[4], m[5]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f32; 6] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        [c0[0], c0[1], c0[2], c1[0], c1[1], c1[2]]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f32; 3]; 2]) -> Self {
        Self::from_cols(Vec3A::from_array(m[0]), Vec3A::from_array(m[1]))
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

impl Default for Mat2x3A {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for Mat2x3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat2x3A))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

impl fmt::Display for Mat2x3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x_axis, self.y_axis)
    }
}

impl AsRef<[Vec3A; 2]> for Mat2x3A {
    #[inline(always)]
    fn as_ref(&self) -> &[Vec3A; 2] {
        #[expect(unsafe_code, reason = "Mat2x3A consists of 2 contiguous Vec3A columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[Vec3A; 2]>())
        }
    }
}

impl AsMut<[Vec3A; 2]> for Mat2x3A {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [Vec3A; 2] {
        #[expect(unsafe_code, reason = "Mat2x3A consists of 2 contiguous Vec3A columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[Vec3A; 2]>())
        }
    }
}

impl Index<usize> for Mat2x3A {
    type Output = Vec3A;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Mat2x3A {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis + rhs.x_axis, self.y_axis + rhs.y_axis)
    }
}

impl Add<f32> for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs)
    }
}

impl Add<Mat2x3A> for f32 {
    type Output = Mat2x3A;
    #[inline(always)]
    fn add(self, rhs: Mat2x3A) -> Mat2x3A {
        rhs + self
    }
}

impl AddAssign for Mat2x3A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Mat2x3A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis - rhs.x_axis, self.y_axis - rhs.y_axis)
    }
}

impl Sub<f32> for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs)
    }
}

impl Sub<Mat2x3A> for f32 {
    type Output = Mat2x3A;
    #[inline(always)]
    fn sub(self, rhs: Mat2x3A) -> Mat2x3A {
        Mat2x3A::from_cols(self - rhs.x_axis, self - rhs.y_axis)
    }
}

impl SubAssign for Mat2x3A {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Mat2x3A {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Neg for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis)
    }
}

impl Mul<f32> for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs)
    }
}

impl Mul<Mat2x3A> for f32 {
    type Output = Mat2x3A;
    #[inline(always)]
    fn mul(self, rhs: Mat2x3A) -> Mat2x3A {
        rhs * self
    }
}

impl MulAssign<f32> for Mat2x3A {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Mat2x3A {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs)
    }
}

impl Div<Mat2x3A> for f32 {
    type Output = Mat2x3A;
    #[inline(always)]
    fn div(self, rhs: Mat2x3A) -> Mat2x3A {
        Mat2x3A::from_cols(self / rhs.x_axis, self / rhs.y_axis)
    }
}

impl DivAssign<f32> for Mat2x3A {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Vec2> for Mat2x3A {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, rhs: Vec2) -> Vec3A {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(Vec3A::splat(r[1]), res);
        res
    }
}

impl Mul<Mat2x3A> for Vec3A {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, rhs: Mat2x3A) -> Vec2 {
        Vec2::new(self.dot(rhs.x_axis), self.dot(rhs.y_axis))
    }
}

impl Mul<Mat2> for Mat2x3A {
    type Output = Mat2x3A;
    #[inline]
    fn mul(self, rhs: Mat2) -> Mat2x3A {
        Mat2x3A::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<Mat3x2> for Mat2x3A {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: Mat3x2) -> Mat3A {
        Mat3A::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<Mat4x2> for Mat2x3A {
    type Output = Mat4x3A;
    #[inline]
    fn mul(self, rhs: Mat4x2) -> Mat4x3A {
        Mat4x3A::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f32; 3]; 2]> for Mat2x3A {
    #[inline(always)]
    fn from(m: [[f32; 3]; 2]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<Mat2x3A> for [[f32; 3]; 2] {
    #[inline(always)]
    fn from(m: Mat2x3A) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f32; 6]> for Mat2x3A {
    #[inline(always)]
    fn from(m: [f32; 6]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<Mat2x3A> for [f32; 6] {
    #[inline(always)]
    fn from(m: Mat2x3A) -> Self {
        m.to_cols_array()
    }
}
