use crate::Mat2;
use crate::Mat2x4;
use crate::Mat3x2;
use crate::Mat3x4;
use crate::Mat4;
use crate::Vec2;
use crate::Vec4;
use crate::simd::SimdF32x4;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub struct Mat4x2(pub(crate) [SimdF32x4; 2]);

impl Mat4x2 {
    pub const ZERO: Self = Self::from_cols(Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO);
    pub const NAN: Self = Self::from_cols(Vec2::NAN, Vec2::NAN, Vec2::NAN, Vec2::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: Vec2, y_axis: Vec2, z_axis: Vec2, w_axis: Vec2) -> Self {
        Self([
            SimdF32x4::new(x_axis.x, x_axis.y, y_axis.x, y_axis.y),
            SimdF32x4::new(z_axis.x, z_axis.y, w_axis.x, w_axis.y),
        ])
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f32) -> Self {
        Self::from_cols(
            Vec2::new(s, 0.0),
            Vec2::new(0.0, s),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> Vec2 {
        let a0 = self.0[0].to_array();
        let a1 = self.0[1].to_array();
        match idx {
            0 => Vec2::new(a0[0], a0[1]),
            1 => Vec2::new(a0[2], a0[3]),
            2 => Vec2::new(a1[0], a1[1]),
            _ => Vec2::new(a1[2], a1[3]),
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f32; 8]) -> Self {
        Self::from_cols(
            Vec2::new(m[0], m[1]),
            Vec2::new(m[2], m[3]),
            Vec2::new(m[4], m[5]),
            Vec2::new(m[6], m[7]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f32; 8] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let c3 = self.col(3).to_array();
        [c0[0], c0[1], c1[0], c1[1], c2[0], c2[1], c3[0], c3[1]]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f32; 2]; 4]) -> Self {
        Self::from_cols(
            Vec2::from_array(m[0]),
            Vec2::from_array(m[1]),
            Vec2::from_array(m[2]),
            Vec2::from_array(m[3]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f32; 2]; 4] {
        [
            self.col(0).to_array(),
            self.col(1).to_array(),
            self.col(2).to_array(),
            self.col(3).to_array(),
        ]
    }

    #[inline(always)]
    pub(crate) fn matrix_comp_mult(self, rhs: Self) -> Self {
        Self([self.0[0].mul(rhs.0[0]), self.0[1].mul(rhs.0[1])])
    }

    #[inline(always)]
    pub(crate) fn transpose(self) -> Mat2x4 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let c3 = self.col(3).to_array();
        Mat2x4::from_cols(
            Vec4::new(c0[0], c1[0], c2[0], c3[0]),
            Vec4::new(c0[1], c1[1], c2[1], c3[1]),
        )
    }
}

impl Default for Mat4x2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Mat4x2 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_cols_array() == other.to_cols_array()
    }
}

impl Deref for Mat4x2 {
    type Target = crate::deref::Cols4<Vec2>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(unsafe_code, reason = "Mat4x2 stores 4 contiguous Vec2 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for Mat4x2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(unsafe_code, reason = "Mat4x2 stores 4 contiguous Vec2 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for Mat4x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat4x2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .field("w_axis", &self.w_axis)
            .finish()
    }
}

impl fmt::Display for Mat4x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.x_axis, self.y_axis, self.z_axis, self.w_axis
        )
    }
}

impl AsRef<[Vec2; 4]> for Mat4x2 {
    #[inline(always)]
    fn as_ref(&self) -> &[Vec2; 4] {
        #[expect(unsafe_code, reason = "Mat4x2 consists of 4 contiguous Vec2 columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[Vec2; 4]>())
        }
    }
}

impl AsMut<[Vec2; 4]> for Mat4x2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [Vec2; 4] {
        #[expect(unsafe_code, reason = "Mat4x2 consists of 4 contiguous Vec2 columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[Vec2; 4]>())
        }
    }
}

impl Index<usize> for Mat4x2 {
    type Output = Vec2;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Mat4x2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self([self.0[0].add(rhs.0[0]), self.0[1].add(rhs.0[1])])
    }
}

impl Add<f32> for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        let s = SimdF32x4::splat(rhs);
        Self([self.0[0].add(s), self.0[1].add(s)])
    }
}

impl Add<Mat4x2> for f32 {
    type Output = Mat4x2;
    #[inline(always)]
    fn add(self, rhs: Mat4x2) -> Mat4x2 {
        rhs + self
    }
}

impl AddAssign for Mat4x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Mat4x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self([self.0[0].sub(rhs.0[0]), self.0[1].sub(rhs.0[1])])
    }
}

impl Sub<f32> for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        let s = SimdF32x4::splat(rhs);
        Self([self.0[0].sub(s), self.0[1].sub(s)])
    }
}

impl Sub<Mat4x2> for f32 {
    type Output = Mat4x2;
    #[inline(always)]
    fn sub(self, rhs: Mat4x2) -> Mat4x2 {
        let s = SimdF32x4::splat(self);
        Mat4x2([s.sub(rhs.0[0]), s.sub(rhs.0[1])])
    }
}

impl SubAssign for Mat4x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Mat4x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Neg for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self([self.0[0].neg(), self.0[1].neg()])
    }
}

impl Mul<f32> for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        let s = SimdF32x4::splat(rhs);
        Self([self.0[0].mul(s), self.0[1].mul(s)])
    }
}

impl Mul<Mat4x2> for f32 {
    type Output = Mat4x2;
    #[inline(always)]
    fn mul(self, rhs: Mat4x2) -> Mat4x2 {
        rhs * self
    }
}

impl MulAssign<f32> for Mat4x2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Mat4x2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        let s = SimdF32x4::splat(rhs);
        Self([self.0[0].div(s), self.0[1].div(s)])
    }
}

impl Div<Mat4x2> for f32 {
    type Output = Mat4x2;
    #[inline(always)]
    fn div(self, rhs: Mat4x2) -> Mat4x2 {
        let s = SimdF32x4::splat(self);
        Mat4x2([s.div(rhs.0[0]), s.div(rhs.0[1])])
    }
}

impl DivAssign<f32> for Mat4x2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Vec4> for Mat4x2 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, rhs: Vec4) -> Vec2 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(Vec2::splat(r[1]), res);
        res = self.z_axis.mul_add(Vec2::splat(r[2]), res);
        res = self.w_axis.mul_add(Vec2::splat(r[3]), res);
        res
    }
}

impl Mul<Mat4x2> for Vec2 {
    type Output = Vec4;
    #[inline(always)]
    fn mul(self, rhs: Mat4x2) -> Vec4 {
        Vec4::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
            self.dot(rhs.w_axis),
        )
    }
}

impl Mul<Mat2x4> for Mat4x2 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2x4) -> Mat2 {
        Mat2::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<Mat3x4> for Mat4x2 {
    type Output = Mat3x2;
    #[inline]
    fn mul(self, rhs: Mat3x4) -> Mat3x2 {
        Mat3x2::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<Mat4> for Mat4x2 {
    type Output = Mat4x2;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4x2 {
        Mat4x2::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f32; 2]; 4]> for Mat4x2 {
    #[inline(always)]
    fn from(m: [[f32; 2]; 4]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<Mat4x2> for [[f32; 2]; 4] {
    #[inline(always)]
    fn from(m: Mat4x2) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f32; 8]> for Mat4x2 {
    #[inline(always)]
    fn from(m: [f32; 8]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<Mat4x2> for [f32; 8] {
    #[inline(always)]
    fn from(m: Mat4x2) -> Self {
        m.to_cols_array()
    }
}
