use crate::Mat3x2;
use crate::Mat4x2;
use crate::Vec2;
use crate::simd::SimdF32x4;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct Mat2(pub(crate) SimdF32x4);

impl Mat2 {
    pub const ZERO: Self = Self::from_cols(Vec2::ZERO, Vec2::ZERO);
    pub const NAN: Self = Self::from_cols(Vec2::NAN, Vec2::NAN);
    pub const IDENTITY: Self = Self::diagonal(1.0);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self(SimdF32x4::new(x_axis.x, x_axis.y, y_axis.x, y_axis.y))
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f32) -> Self {
        Self::from_cols(Vec2::new(s, 0.0), Vec2::new(0.0, s))
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> Vec2 {
        let a = self.0.to_array();
        match idx {
            0 => Vec2::new(a[0], a[1]),
            _ => Vec2::new(a[2], a[3]),
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f32; 4]) -> Self {
        Self::from_cols(Vec2::new(m[0], m[1]), Vec2::new(m[2], m[3]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f32; 4] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        [c0[0], c0[1], c1[0], c1[1]]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f32; 2]; 2]) -> Self {
        Self::from_cols(Vec2::from_array(m[0]), Vec2::from_array(m[1]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f32; 2]; 2] {
        [self.col(0).to_array(), self.col(1).to_array()]
    }

    #[inline(always)]
    pub(crate) fn matrix_comp_mult(self, rhs: Self) -> Self {
        Self(self.0.mul(rhs.0))
    }

    #[inline(always)]
    pub(crate) fn transpose(self) -> Mat2 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        Mat2::from_cols(Vec2::new(c0[0], c1[0]), Vec2::new(c0[1], c1[1]))
    }

    #[inline]
    pub(crate) fn determinant(self) -> f32 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        c0[0] * c1[1] - c0[1] * c1[0]
    }

    #[inline]
    pub(crate) fn inverse(self) -> Self {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let det = c0[0] * c1[1] - c0[1] * c1[0];
        rglm_debug_assert!(det != 0.0);
        let inv_det = 1.0 / det;
        Self::from_cols(
            Vec2::new(c1[1] * inv_det, -c0[1] * inv_det),
            Vec2::new(-c1[0] * inv_det, c0[0] * inv_det),
        )
    }
}

impl Default for Mat2 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for Mat2 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_cols_array() == other.to_cols_array()
    }
}

impl Deref for Mat2 {
    type Target = crate::deref::Cols2<Vec2>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(unsafe_code, reason = "Mat2 stores 2 contiguous Vec2 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for Mat2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(unsafe_code, reason = "Mat2 stores 2 contiguous Vec2 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x_axis, self.y_axis)
    }
}

impl AsRef<[Vec2; 2]> for Mat2 {
    #[inline(always)]
    fn as_ref(&self) -> &[Vec2; 2] {
        #[expect(unsafe_code, reason = "Mat2 consists of 2 contiguous Vec2 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[Vec2; 2]>())
        }
    }
}

impl AsMut<[Vec2; 2]> for Mat2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [Vec2; 2] {
        #[expect(unsafe_code, reason = "Mat2 consists of 2 contiguous Vec2 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[Vec2; 2]>())
        }
    }
}

impl Index<usize> for Mat2 {
    type Output = Vec2;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Mat2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }
}

impl Add<f32> for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        Self(self.0.add(SimdF32x4::splat(rhs)))
    }
}

impl Add<Mat2> for f32 {
    type Output = Mat2;
    #[inline(always)]
    fn add(self, rhs: Mat2) -> Mat2 {
        rhs + self
    }
}

impl AddAssign for Mat2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Mat2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0))
    }
}

impl Sub<f32> for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        Self(self.0.sub(SimdF32x4::splat(rhs)))
    }
}

impl Sub<Mat2> for f32 {
    type Output = Mat2;
    #[inline(always)]
    fn sub(self, rhs: Mat2) -> Mat2 {
        Mat2(SimdF32x4::splat(self).sub(rhs.0))
    }
}

impl SubAssign for Mat2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Mat2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Neg for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0.mul(SimdF32x4::splat(rhs)))
    }
}

impl Mul<Mat2> for f32 {
    type Output = Mat2;
    #[inline(always)]
    fn mul(self, rhs: Mat2) -> Mat2 {
        rhs * self
    }
}

impl MulAssign<f32> for Mat2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Self(self.0.div(SimdF32x4::splat(rhs)))
    }
}

impl Div<Mat2> for f32 {
    type Output = Mat2;
    #[inline(always)]
    fn div(self, rhs: Mat2) -> Mat2 {
        Mat2(SimdF32x4::splat(self).div(rhs.0))
    }
}

impl DivAssign<f32> for Mat2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, rhs: Vec2) -> Vec2 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(Vec2::splat(r[1]), res);
        res
    }
}

impl Mul<Mat2> for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, rhs: Mat2) -> Vec2 {
        Vec2::new(self.dot(rhs.x_axis), self.dot(rhs.y_axis))
    }
}

impl MulAssign<Mat2> for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Mat2) {
        *self = *self * rhs;
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Mat2 {
        Mat2::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<Mat3x2> for Mat2 {
    type Output = Mat3x2;
    #[inline]
    fn mul(self, rhs: Mat3x2) -> Mat3x2 {
        Mat3x2::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<Mat4x2> for Mat2 {
    type Output = Mat4x2;
    #[inline]
    fn mul(self, rhs: Mat4x2) -> Mat4x2 {
        Mat4x2::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl MulAssign for Mat2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<[[f32; 2]; 2]> for Mat2 {
    #[inline(always)]
    fn from(m: [[f32; 2]; 2]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<Mat2> for [[f32; 2]; 2] {
    #[inline(always)]
    fn from(m: Mat2) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f32; 4]> for Mat2 {
    #[inline(always)]
    fn from(m: [f32; 4]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<Mat2> for [f32; 4] {
    #[inline(always)]
    fn from(m: Mat2) -> Self {
        m.to_cols_array()
    }
}
