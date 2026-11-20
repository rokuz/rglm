use crate::F16Mat3x2;
use crate::F16Mat4x2;
use crate::F16Vec2;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use half::f16;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct F16Mat2 {
    pub x_axis: F16Vec2,
    pub y_axis: F16Vec2,
}

impl F16Mat2 {
    pub const ZERO: Self = Self::from_cols(F16Vec2::ZERO, F16Vec2::ZERO);
    pub const NAN: Self = Self::from_cols(F16Vec2::NAN, F16Vec2::NAN);
    pub const IDENTITY: Self = Self::diagonal(f16::ONE);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: F16Vec2, y_axis: F16Vec2) -> Self {
        Self { x_axis, y_axis }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f16) -> Self {
        Self::from_cols(F16Vec2::new(s, f16::ZERO), F16Vec2::new(f16::ZERO, s))
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> F16Vec2 {
        match idx {
            0 => self.x_axis,
            _ => self.y_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f16; 4]) -> Self {
        Self::from_cols(F16Vec2::new(m[0], m[1]), F16Vec2::new(m[2], m[3]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f16; 4] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        [c0[0], c0[1], c1[0], c1[1]]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f16; 2]; 2]) -> Self {
        Self::from_cols(F16Vec2::from_array(m[0]), F16Vec2::from_array(m[1]))
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f16; 2]; 2] {
        [self.col(0).to_array(), self.col(1).to_array()]
    }

    #[inline(always)]
    pub(crate) fn matrix_comp_mult(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis * rhs.x_axis, self.y_axis * rhs.y_axis)
    }

    #[inline(always)]
    pub(crate) fn transpose(self) -> F16Mat2 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        F16Mat2::from_cols(F16Vec2::new(c0[0], c1[0]), F16Vec2::new(c0[1], c1[1]))
    }

    #[inline]
    pub(crate) fn determinant(self) -> f16 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        f16::from_f32(c0[0].to_f32() * c1[1].to_f32() - c0[1].to_f32() * c1[0].to_f32())
    }

    #[inline]
    pub(crate) fn inverse(self) -> Self {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let a = c0[0].to_f32();
        let b = c0[1].to_f32();
        let c = c1[0].to_f32();
        let d = c1[1].to_f32();
        let det = a * d - b * c;
        rglm_debug_assert!(det != 0.0);
        let inv_det = 1.0 / det;
        Self::from_cols(
            F16Vec2::new(f16::from_f32(d * inv_det), f16::from_f32(-b * inv_det)),
            F16Vec2::new(f16::from_f32(-c * inv_det), f16::from_f32(a * inv_det)),
        )
    }
}

impl Default for F16Mat2 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl fmt::Debug for F16Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(F16Mat2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

impl fmt::Display for F16Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x_axis, self.y_axis)
    }
}

impl AsRef<[F16Vec2; 2]> for F16Mat2 {
    #[inline(always)]
    fn as_ref(&self) -> &[F16Vec2; 2] {
        #[expect(
            unsafe_code,
            reason = "F16Mat2 consists of 2 contiguous F16Vec2 columns"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[F16Vec2; 2]>())
        }
    }
}

impl AsMut<[F16Vec2; 2]> for F16Mat2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [F16Vec2; 2] {
        #[expect(
            unsafe_code,
            reason = "F16Mat2 consists of 2 contiguous F16Vec2 columns"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[F16Vec2; 2]>())
        }
    }
}

impl Index<usize> for F16Mat2 {
    type Output = F16Vec2;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for F16Mat2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis + rhs.x_axis, self.y_axis + rhs.y_axis)
    }
}

impl Add<f16> for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs)
    }
}

impl Add<F16Mat2> for f16 {
    type Output = F16Mat2;
    #[inline(always)]
    fn add(self, rhs: F16Mat2) -> F16Mat2 {
        rhs + self
    }
}

impl AddAssign for F16Mat2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f16> for F16Mat2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f16) {
        *self = *self + rhs;
    }
}

impl Sub for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis - rhs.x_axis, self.y_axis - rhs.y_axis)
    }
}

impl Sub<f16> for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs)
    }
}

impl Sub<F16Mat2> for f16 {
    type Output = F16Mat2;
    #[inline(always)]
    fn sub(self, rhs: F16Mat2) -> F16Mat2 {
        F16Mat2::from_cols(self - rhs.x_axis, self - rhs.y_axis)
    }
}

impl SubAssign for F16Mat2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f16> for F16Mat2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f16) {
        *self = *self - rhs;
    }
}

impl Neg for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis)
    }
}

impl Mul<f16> for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs)
    }
}

impl Mul<F16Mat2> for f16 {
    type Output = F16Mat2;
    #[inline(always)]
    fn mul(self, rhs: F16Mat2) -> F16Mat2 {
        rhs * self
    }
}

impl MulAssign<f16> for F16Mat2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f16) {
        *self = *self * rhs;
    }
}

impl Div<f16> for F16Mat2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs)
    }
}

impl Div<F16Mat2> for f16 {
    type Output = F16Mat2;
    #[inline(always)]
    fn div(self, rhs: F16Mat2) -> F16Mat2 {
        F16Mat2::from_cols(self / rhs.x_axis, self / rhs.y_axis)
    }
}

impl DivAssign<f16> for F16Mat2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f16) {
        *self = *self / rhs;
    }
}

impl Mul<F16Vec2> for F16Mat2 {
    type Output = F16Vec2;
    #[inline(always)]
    fn mul(self, rhs: F16Vec2) -> F16Vec2 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(F16Vec2::splat(r[1]), res);
        res
    }
}

impl Mul<F16Mat2> for F16Vec2 {
    type Output = F16Vec2;
    #[inline(always)]
    fn mul(self, rhs: F16Mat2) -> F16Vec2 {
        F16Vec2::new(self.dot(rhs.x_axis), self.dot(rhs.y_axis))
    }
}

impl MulAssign<F16Mat2> for F16Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F16Mat2) {
        *self = *self * rhs;
    }
}

impl Mul<F16Mat2> for F16Mat2 {
    type Output = F16Mat2;
    #[inline]
    fn mul(self, rhs: F16Mat2) -> F16Mat2 {
        F16Mat2::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<F16Mat3x2> for F16Mat2 {
    type Output = F16Mat3x2;
    #[inline]
    fn mul(self, rhs: F16Mat3x2) -> F16Mat3x2 {
        F16Mat3x2::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<F16Mat4x2> for F16Mat2 {
    type Output = F16Mat4x2;
    #[inline]
    fn mul(self, rhs: F16Mat4x2) -> F16Mat4x2 {
        F16Mat4x2::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl MulAssign for F16Mat2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<[[f16; 2]; 2]> for F16Mat2 {
    #[inline(always)]
    fn from(m: [[f16; 2]; 2]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<F16Mat2> for [[f16; 2]; 2] {
    #[inline(always)]
    fn from(m: F16Mat2) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f16; 4]> for F16Mat2 {
    #[inline(always)]
    fn from(m: [f16; 4]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<F16Mat2> for [f16; 4] {
    #[inline(always)]
    fn from(m: F16Mat2) -> Self {
        m.to_cols_array()
    }
}
