use crate::F16Mat2x3;
use crate::F16Mat2x4;
use crate::F16Mat3;
use crate::F16Mat4;
use crate::F16Mat4x3;
use crate::F16Vec3;
use crate::F16Vec4;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use half::f16;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct F16Mat3x4 {
    pub x_axis: F16Vec4,
    pub y_axis: F16Vec4,
    pub z_axis: F16Vec4,
}

impl F16Mat3x4 {
    pub const ZERO: Self = Self::from_cols(F16Vec4::ZERO, F16Vec4::ZERO, F16Vec4::ZERO);
    pub const NAN: Self = Self::from_cols(F16Vec4::NAN, F16Vec4::NAN, F16Vec4::NAN);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: F16Vec4, y_axis: F16Vec4, z_axis: F16Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f16) -> Self {
        Self::from_cols(
            F16Vec4::new(s, f16::ZERO, f16::ZERO, f16::ZERO),
            F16Vec4::new(f16::ZERO, s, f16::ZERO, f16::ZERO),
            F16Vec4::new(f16::ZERO, f16::ZERO, s, f16::ZERO),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> F16Vec4 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => self.z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f16; 12]) -> Self {
        Self::from_cols(
            F16Vec4::new(m[0], m[1], m[2], m[3]),
            F16Vec4::new(m[4], m[5], m[6], m[7]),
            F16Vec4::new(m[8], m[9], m[10], m[11]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f16; 12] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        [
            c0[0], c0[1], c0[2], c0[3], c1[0], c1[1], c1[2], c1[3], c2[0], c2[1], c2[2], c2[3],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f16; 4]; 3]) -> Self {
        Self::from_cols(
            F16Vec4::from_array(m[0]),
            F16Vec4::from_array(m[1]),
            F16Vec4::from_array(m[2]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f16; 4]; 3] {
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
    pub(crate) fn transpose(self) -> F16Mat4x3 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        F16Mat4x3::from_cols(
            F16Vec3::new(c0[0], c1[0], c2[0]),
            F16Vec3::new(c0[1], c1[1], c2[1]),
            F16Vec3::new(c0[2], c1[2], c2[2]),
            F16Vec3::new(c0[3], c1[3], c2[3]),
        )
    }
}

impl Default for F16Mat3x4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for F16Mat3x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(F16Mat3x4))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for F16Mat3x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl AsRef<[F16Vec4; 3]> for F16Mat3x4 {
    #[inline(always)]
    fn as_ref(&self) -> &[F16Vec4; 3] {
        #[expect(
            unsafe_code,
            reason = "F16Mat3x4 consists of 3 contiguous F16Vec4 columns"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[F16Vec4; 3]>())
        }
    }
}

impl AsMut<[F16Vec4; 3]> for F16Mat3x4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [F16Vec4; 3] {
        #[expect(
            unsafe_code,
            reason = "F16Mat3x4 consists of 3 contiguous F16Vec4 columns"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[F16Vec4; 3]>())
        }
    }
}

impl Index<usize> for F16Mat3x4 {
    type Output = F16Vec4;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for F16Mat3x4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for F16Mat3x4 {
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

impl Add<f16> for F16Mat3x4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs, self.z_axis + rhs)
    }
}

impl Add<F16Mat3x4> for f16 {
    type Output = F16Mat3x4;
    #[inline(always)]
    fn add(self, rhs: F16Mat3x4) -> F16Mat3x4 {
        rhs + self
    }
}

impl AddAssign for F16Mat3x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f16> for F16Mat3x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f16) {
        *self = *self + rhs;
    }
}

impl Sub for F16Mat3x4 {
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

impl Sub<f16> for F16Mat3x4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs, self.z_axis - rhs)
    }
}

impl Sub<F16Mat3x4> for f16 {
    type Output = F16Mat3x4;
    #[inline(always)]
    fn sub(self, rhs: F16Mat3x4) -> F16Mat3x4 {
        F16Mat3x4::from_cols(self - rhs.x_axis, self - rhs.y_axis, self - rhs.z_axis)
    }
}

impl SubAssign for F16Mat3x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f16> for F16Mat3x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f16) {
        *self = *self - rhs;
    }
}

impl Neg for F16Mat3x4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis)
    }
}

impl Mul<f16> for F16Mat3x4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs, self.z_axis * rhs)
    }
}

impl Mul<F16Mat3x4> for f16 {
    type Output = F16Mat3x4;
    #[inline(always)]
    fn mul(self, rhs: F16Mat3x4) -> F16Mat3x4 {
        rhs * self
    }
}

impl MulAssign<f16> for F16Mat3x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f16) {
        *self = *self * rhs;
    }
}

impl Div<f16> for F16Mat3x4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs, self.z_axis / rhs)
    }
}

impl Div<F16Mat3x4> for f16 {
    type Output = F16Mat3x4;
    #[inline(always)]
    fn div(self, rhs: F16Mat3x4) -> F16Mat3x4 {
        F16Mat3x4::from_cols(self / rhs.x_axis, self / rhs.y_axis, self / rhs.z_axis)
    }
}

impl DivAssign<f16> for F16Mat3x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f16) {
        *self = *self / rhs;
    }
}

impl Mul<F16Vec3> for F16Mat3x4 {
    type Output = F16Vec4;
    #[inline(always)]
    fn mul(self, rhs: F16Vec3) -> F16Vec4 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(F16Vec4::splat(r[1]), res);
        res = self.z_axis.mul_add(F16Vec4::splat(r[2]), res);
        res
    }
}

impl Mul<F16Mat3x4> for F16Vec4 {
    type Output = F16Vec3;
    #[inline(always)]
    fn mul(self, rhs: F16Mat3x4) -> F16Vec3 {
        F16Vec3::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
        )
    }
}

impl Mul<F16Mat2x3> for F16Mat3x4 {
    type Output = F16Mat2x4;
    #[inline]
    fn mul(self, rhs: F16Mat2x3) -> F16Mat2x4 {
        F16Mat2x4::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<F16Mat3> for F16Mat3x4 {
    type Output = F16Mat3x4;
    #[inline]
    fn mul(self, rhs: F16Mat3) -> F16Mat3x4 {
        F16Mat3x4::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<F16Mat4x3> for F16Mat3x4 {
    type Output = F16Mat4;
    #[inline]
    fn mul(self, rhs: F16Mat4x3) -> F16Mat4 {
        F16Mat4::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl From<[[f16; 4]; 3]> for F16Mat3x4 {
    #[inline(always)]
    fn from(m: [[f16; 4]; 3]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<F16Mat3x4> for [[f16; 4]; 3] {
    #[inline(always)]
    fn from(m: F16Mat3x4) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f16; 12]> for F16Mat3x4 {
    #[inline(always)]
    fn from(m: [f16; 12]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<F16Mat3x4> for [f16; 12] {
    #[inline(always)]
    fn from(m: F16Mat3x4) -> Self {
        m.to_cols_array()
    }
}
