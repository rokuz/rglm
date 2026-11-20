use crate::Mat2x3A;
use crate::Mat4x3A;
use crate::Vec3A;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Mat3A {
    pub x_axis: Vec3A,
    pub y_axis: Vec3A,
    pub z_axis: Vec3A,
}

impl Mat3A {
    pub const ZERO: Self = Self::from_cols(Vec3A::ZERO, Vec3A::ZERO, Vec3A::ZERO);
    pub const NAN: Self = Self::from_cols(Vec3A::NAN, Vec3A::NAN, Vec3A::NAN);
    pub const IDENTITY: Self = Self::diagonal(1.0);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f32) -> Self {
        Self::from_cols(
            Vec3A::new(s, 0.0, 0.0),
            Vec3A::new(0.0, s, 0.0),
            Vec3A::new(0.0, 0.0, s),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> Vec3A {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => self.z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f32; 9]) -> Self {
        Self::from_cols(
            Vec3A::new(m[0], m[1], m[2]),
            Vec3A::new(m[3], m[4], m[5]),
            Vec3A::new(m[6], m[7], m[8]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f32; 9] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        [
            c0[0], c0[1], c0[2], c1[0], c1[1], c1[2], c2[0], c2[1], c2[2],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f32; 3]; 3]) -> Self {
        Self::from_cols(
            Vec3A::from_array(m[0]),
            Vec3A::from_array(m[1]),
            Vec3A::from_array(m[2]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f32; 3]; 3] {
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
    pub(crate) fn transpose(self) -> Mat3A {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        Mat3A::from_cols(
            Vec3A::new(c0[0], c1[0], c2[0]),
            Vec3A::new(c0[1], c1[1], c2[1]),
            Vec3A::new(c0[2], c1[2], c2[2]),
        )
    }

    #[inline]
    pub(crate) fn determinant(self) -> f32 {
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

impl Default for Mat3A {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl fmt::Debug for Mat3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat3A))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for Mat3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl AsRef<[Vec3A; 3]> for Mat3A {
    #[inline(always)]
    fn as_ref(&self) -> &[Vec3A; 3] {
        #[expect(unsafe_code, reason = "Mat3A consists of 3 contiguous Vec3A columns")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[Vec3A; 3]>())
        }
    }
}

impl AsMut<[Vec3A; 3]> for Mat3A {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [Vec3A; 3] {
        #[expect(unsafe_code, reason = "Mat3A consists of 3 contiguous Vec3A columns")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[Vec3A; 3]>())
        }
    }
}

impl Index<usize> for Mat3A {
    type Output = Vec3A;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Mat3A {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Mat3A {
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

impl Add<f32> for Mat3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs, self.z_axis + rhs)
    }
}

impl Add<Mat3A> for f32 {
    type Output = Mat3A;
    #[inline(always)]
    fn add(self, rhs: Mat3A) -> Mat3A {
        rhs + self
    }
}

impl AddAssign for Mat3A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Mat3A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Mat3A {
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

impl Sub<f32> for Mat3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs, self.z_axis - rhs)
    }
}

impl Sub<Mat3A> for f32 {
    type Output = Mat3A;
    #[inline(always)]
    fn sub(self, rhs: Mat3A) -> Mat3A {
        Mat3A::from_cols(self - rhs.x_axis, self - rhs.y_axis, self - rhs.z_axis)
    }
}

impl SubAssign for Mat3A {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Mat3A {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Neg for Mat3A {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis)
    }
}

impl Mul<f32> for Mat3A {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs, self.z_axis * rhs)
    }
}

impl Mul<Mat3A> for f32 {
    type Output = Mat3A;
    #[inline(always)]
    fn mul(self, rhs: Mat3A) -> Mat3A {
        rhs * self
    }
}

impl MulAssign<f32> for Mat3A {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Mat3A {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs, self.z_axis / rhs)
    }
}

impl Div<Mat3A> for f32 {
    type Output = Mat3A;
    #[inline(always)]
    fn div(self, rhs: Mat3A) -> Mat3A {
        Mat3A::from_cols(self / rhs.x_axis, self / rhs.y_axis, self / rhs.z_axis)
    }
}

impl DivAssign<f32> for Mat3A {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Vec3A> for Mat3A {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        let mut res = self.x_axis.0.mul(rhs.0.swizzle::<0, 0, 0, 0>());
        res = self.y_axis.0.mul_add(rhs.0.swizzle::<1, 1, 1, 1>(), res);
        res = self.z_axis.0.mul_add(rhs.0.swizzle::<2, 2, 2, 2>(), res);
        Vec3A(res)
    }
}

impl Mul<Mat3A> for Vec3A {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, rhs: Mat3A) -> Vec3A {
        Vec3A::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
        )
    }
}

impl Mul<crate::Vec3> for Mat3A {
    type Output = crate::Vec3;
    #[inline(always)]
    fn mul(self, rhs: crate::Vec3) -> crate::Vec3 {
        crate::Vec3::from(self * crate::Vec3A::from(rhs))
    }
}

impl Mul<Mat3A> for crate::Vec3 {
    type Output = crate::Vec3;
    #[inline(always)]
    fn mul(self, rhs: Mat3A) -> crate::Vec3 {
        crate::Vec3::from(crate::Vec3A::from(self) * rhs)
    }
}

impl MulAssign<Mat3A> for Vec3A {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Mat3A) {
        *self = *self * rhs;
    }
}

impl Mul<Mat2x3A> for Mat3A {
    type Output = Mat2x3A;
    #[inline]
    fn mul(self, rhs: Mat2x3A) -> Mat2x3A {
        Mat2x3A::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<Mat3A> for Mat3A {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: Mat3A) -> Mat3A {
        Mat3A::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<Mat4x3A> for Mat3A {
    type Output = Mat4x3A;
    #[inline]
    fn mul(self, rhs: Mat4x3A) -> Mat4x3A {
        Mat4x3A::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl MulAssign for Mat3A {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<[[f32; 3]; 3]> for Mat3A {
    #[inline(always)]
    fn from(m: [[f32; 3]; 3]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<Mat3A> for [[f32; 3]; 3] {
    #[inline(always)]
    fn from(m: Mat3A) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f32; 9]> for Mat3A {
    #[inline(always)]
    fn from(m: [f32; 9]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<Mat3A> for [f32; 9] {
    #[inline(always)]
    fn from(m: Mat3A) -> Self {
        m.to_cols_array()
    }
}
