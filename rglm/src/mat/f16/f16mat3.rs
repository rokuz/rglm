use crate::F16Mat2x3;
use crate::F16Mat4x3;
use crate::F16Vec3;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use half::f16;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct F16Mat3 {
    pub x_axis: F16Vec3,
    pub y_axis: F16Vec3,
    pub z_axis: F16Vec3,
}

impl F16Mat3 {
    pub const ZERO: Self = Self::from_cols(F16Vec3::ZERO, F16Vec3::ZERO, F16Vec3::ZERO);
    pub const NAN: Self = Self::from_cols(F16Vec3::NAN, F16Vec3::NAN, F16Vec3::NAN);
    pub const IDENTITY: Self = Self::diagonal(f16::ONE);
    #[inline(always)]
    pub(crate) const fn from_cols(x_axis: F16Vec3, y_axis: F16Vec3, z_axis: F16Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn diagonal(s: f16) -> Self {
        Self::from_cols(
            F16Vec3::new(s, f16::ZERO, f16::ZERO),
            F16Vec3::new(f16::ZERO, s, f16::ZERO),
            F16Vec3::new(f16::ZERO, f16::ZERO, s),
        )
    }

    #[inline(always)]
    pub(crate) const fn col(self, idx: usize) -> F16Vec3 {
        match idx {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => self.z_axis,
        }
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array(m: [f16; 9]) -> Self {
        Self::from_cols(
            F16Vec3::new(m[0], m[1], m[2]),
            F16Vec3::new(m[3], m[4], m[5]),
            F16Vec3::new(m[6], m[7], m[8]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array(self) -> [f16; 9] {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        [
            c0[0], c0[1], c0[2], c1[0], c1[1], c1[2], c2[0], c2[1], c2[2],
        ]
    }

    #[inline(always)]
    pub(crate) const fn from_cols_array_2d(m: [[f16; 3]; 3]) -> Self {
        Self::from_cols(
            F16Vec3::from_array(m[0]),
            F16Vec3::from_array(m[1]),
            F16Vec3::from_array(m[2]),
        )
    }

    #[inline(always)]
    pub(crate) const fn to_cols_array_2d(self) -> [[f16; 3]; 3] {
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
    pub(crate) fn transpose(self) -> F16Mat3 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        F16Mat3::from_cols(
            F16Vec3::new(c0[0], c1[0], c2[0]),
            F16Vec3::new(c0[1], c1[1], c2[1]),
            F16Vec3::new(c0[2], c1[2], c2[2]),
        )
    }

    #[inline]
    pub(crate) fn determinant(self) -> f16 {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let m00 = c0[0].to_f32();
        let m01 = c0[1].to_f32();
        let m02 = c0[2].to_f32();
        let m10 = c1[0].to_f32();
        let m11 = c1[1].to_f32();
        let m12 = c1[2].to_f32();
        let m20 = c2[0].to_f32();
        let m21 = c2[1].to_f32();
        let m22 = c2[2].to_f32();
        f16::from_f32(
            m00 * (m11 * m22 - m12 * m21)
                + m01 * (m12 * m20 - m10 * m22)
                + m02 * (m10 * m21 - m11 * m20),
        )
    }

    #[inline]
    pub(crate) fn inverse(self) -> Self {
        let c0 = self.col(0).to_array();
        let c1 = self.col(1).to_array();
        let c2 = self.col(2).to_array();
        let m = crate::Mat3::from_cols(
            crate::Vec3::new(c0[0].to_f32(), c0[1].to_f32(), c0[2].to_f32()),
            crate::Vec3::new(c1[0].to_f32(), c1[1].to_f32(), c1[2].to_f32()),
            crate::Vec3::new(c2[0].to_f32(), c2[1].to_f32(), c2[2].to_f32()),
        )
        .inverse();
        Self::from_cols(
            F16Vec3::new(
                f16::from_f32(m.x_axis.x),
                f16::from_f32(m.x_axis.y),
                f16::from_f32(m.x_axis.z),
            ),
            F16Vec3::new(
                f16::from_f32(m.y_axis.x),
                f16::from_f32(m.y_axis.y),
                f16::from_f32(m.y_axis.z),
            ),
            F16Vec3::new(
                f16::from_f32(m.z_axis.x),
                f16::from_f32(m.z_axis.y),
                f16::from_f32(m.z_axis.z),
            ),
        )
    }
}

impl Default for F16Mat3 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl fmt::Debug for F16Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(F16Mat3))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for F16Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl AsRef<[F16Vec3; 3]> for F16Mat3 {
    #[inline(always)]
    fn as_ref(&self) -> &[F16Vec3; 3] {
        #[expect(
            unsafe_code,
            reason = "F16Mat3 consists of 3 contiguous F16Vec3 columns"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[F16Vec3; 3]>())
        }
    }
}

impl AsMut<[F16Vec3; 3]> for F16Mat3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [F16Vec3; 3] {
        #[expect(
            unsafe_code,
            reason = "F16Mat3 consists of 3 contiguous F16Vec3 columns"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[F16Vec3; 3]>())
        }
    }
}

impl Index<usize> for F16Mat3 {
    type Output = F16Vec3;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for F16Mat3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for F16Mat3 {
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

impl Add<f16> for F16Mat3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis + rhs, self.y_axis + rhs, self.z_axis + rhs)
    }
}

impl Add<F16Mat3> for f16 {
    type Output = F16Mat3;
    #[inline(always)]
    fn add(self, rhs: F16Mat3) -> F16Mat3 {
        rhs + self
    }
}

impl AddAssign for F16Mat3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f16> for F16Mat3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f16) {
        *self = *self + rhs;
    }
}

impl Sub for F16Mat3 {
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

impl Sub<f16> for F16Mat3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis - rhs, self.y_axis - rhs, self.z_axis - rhs)
    }
}

impl Sub<F16Mat3> for f16 {
    type Output = F16Mat3;
    #[inline(always)]
    fn sub(self, rhs: F16Mat3) -> F16Mat3 {
        F16Mat3::from_cols(self - rhs.x_axis, self - rhs.y_axis, self - rhs.z_axis)
    }
}

impl SubAssign for F16Mat3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f16> for F16Mat3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f16) {
        *self = *self - rhs;
    }
}

impl Neg for F16Mat3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::from_cols(-self.x_axis, -self.y_axis, -self.z_axis)
    }
}

impl Mul<f16> for F16Mat3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis * rhs, self.y_axis * rhs, self.z_axis * rhs)
    }
}

impl Mul<F16Mat3> for f16 {
    type Output = F16Mat3;
    #[inline(always)]
    fn mul(self, rhs: F16Mat3) -> F16Mat3 {
        rhs * self
    }
}

impl MulAssign<f16> for F16Mat3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f16) {
        *self = *self * rhs;
    }
}

impl Div<f16> for F16Mat3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f16) -> Self {
        Self::from_cols(self.x_axis / rhs, self.y_axis / rhs, self.z_axis / rhs)
    }
}

impl Div<F16Mat3> for f16 {
    type Output = F16Mat3;
    #[inline(always)]
    fn div(self, rhs: F16Mat3) -> F16Mat3 {
        F16Mat3::from_cols(self / rhs.x_axis, self / rhs.y_axis, self / rhs.z_axis)
    }
}

impl DivAssign<f16> for F16Mat3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f16) {
        *self = *self / rhs;
    }
}

impl Mul<F16Vec3> for F16Mat3 {
    type Output = F16Vec3;
    #[inline(always)]
    fn mul(self, rhs: F16Vec3) -> F16Vec3 {
        let r = rhs.to_array();
        let mut res = self.x_axis * r[0];
        res = self.y_axis.mul_add(F16Vec3::splat(r[1]), res);
        res = self.z_axis.mul_add(F16Vec3::splat(r[2]), res);
        res
    }
}

impl Mul<F16Mat3> for F16Vec3 {
    type Output = F16Vec3;
    #[inline(always)]
    fn mul(self, rhs: F16Mat3) -> F16Vec3 {
        F16Vec3::new(
            self.dot(rhs.x_axis),
            self.dot(rhs.y_axis),
            self.dot(rhs.z_axis),
        )
    }
}

impl MulAssign<F16Mat3> for F16Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F16Mat3) {
        *self = *self * rhs;
    }
}

impl Mul<F16Mat2x3> for F16Mat3 {
    type Output = F16Mat2x3;
    #[inline]
    fn mul(self, rhs: F16Mat2x3) -> F16Mat2x3 {
        F16Mat2x3::from_cols(self * rhs.x_axis, self * rhs.y_axis)
    }
}

impl Mul<F16Mat3> for F16Mat3 {
    type Output = F16Mat3;
    #[inline]
    fn mul(self, rhs: F16Mat3) -> F16Mat3 {
        F16Mat3::from_cols(self * rhs.x_axis, self * rhs.y_axis, self * rhs.z_axis)
    }
}

impl Mul<F16Mat4x3> for F16Mat3 {
    type Output = F16Mat4x3;
    #[inline]
    fn mul(self, rhs: F16Mat4x3) -> F16Mat4x3 {
        F16Mat4x3::from_cols(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl MulAssign for F16Mat3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<[[f16; 3]; 3]> for F16Mat3 {
    #[inline(always)]
    fn from(m: [[f16; 3]; 3]) -> Self {
        Self::from_cols_array_2d(m)
    }
}

impl From<F16Mat3> for [[f16; 3]; 3] {
    #[inline(always)]
    fn from(m: F16Mat3) -> Self {
        m.to_cols_array_2d()
    }
}

impl From<[f16; 9]> for F16Mat3 {
    #[inline(always)]
    fn from(m: [f16; 9]) -> Self {
        Self::from_cols_array(m)
    }
}

impl From<F16Mat3> for [f16; 9] {
    #[inline(always)]
    fn from(m: F16Mat3) -> Self {
        m.to_cols_array()
    }
}
