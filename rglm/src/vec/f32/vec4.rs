use crate::BVec4;
use crate::BVec4A;
use crate::math::f32_math;
use crate::simd::SimdF32x4;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::Neg;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct Vec4(pub(crate) SimdF32x4);

impl Vec4 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const MIN: Self = Self::splat(f32::MIN);
    pub const MAX: Self = Self::splat(f32::MAX);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);
    #[inline(always)]
    pub(crate) const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(SimdF32x4::new(x, y, z, w))
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f32; 4] {
        let a = self.0.to_array();
        [a[0], a[1], a[2], a[3]]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f32 {
        self.0.dot4(rhs.0)
    }

    #[inline(always)]
    pub(crate) fn min(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }

    #[inline(always)]
    pub(crate) fn max(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }

    #[inline(always)]
    pub(crate) fn clamp(self, min_val: Self, max_val: Self) -> Self {
        self.max(min_val).min(max_val)
    }

    #[inline(always)]
    pub(crate) fn abs(self) -> Self {
        Self(self.0.abs())
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        Self(self.0.sign())
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        Self(self.0.floor())
    }

    #[inline(always)]
    pub(crate) fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    #[inline(always)]
    pub(crate) fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    #[inline(always)]
    pub(crate) fn round(self) -> Self {
        Self(self.0.round())
    }

    #[inline(always)]
    pub(crate) fn round_even(self) -> Self {
        Self(self.0.round_even())
    }

    #[inline(always)]
    pub(crate) fn fract(self) -> Self {
        Self(self.0.fract())
    }

    #[inline(always)]
    pub(crate) fn modulo(self, rhs: Self) -> Self {
        self - (rhs * (self / rhs).floor())
    }

    #[inline(always)]
    pub(crate) fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(self) -> Self {
        Self(self.0.inversesqrt())
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        Self(self.0.mul_add(b.0, c.0))
    }

    #[inline(always)]
    pub(crate) fn fma_exact(self, b: Self, c: Self) -> Self {
        Self(self.0.fma_exact(b.0, c.0))
    }

    #[inline(always)]
    pub(crate) fn length(self) -> f32 {
        f32_math::sqrt(self.dot(self))
    }

    #[inline(always)]
    pub(crate) fn normalize(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > 0.0);
        self * f32_math::inversesqrt(len_sq)
    }

    #[inline(always)]
    pub(crate) fn select_mask(mask: BVec4, if_true: Self, if_false: Self) -> Self {
        Self(SimdF32x4::select(
            BVec4A::from(mask).0,
            if_true.0,
            if_false.0,
        ))
    }

    #[inline(always)]
    pub(crate) fn select_simd(mask: BVec4A, if_true: Self, if_false: Self) -> Self {
        Self(SimdF32x4::select(mask.0, if_true.0, if_false.0))
    }

    #[inline(always)]
    pub(crate) fn cmp_eq(self, rhs: Self) -> BVec4 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec4::new(a[0] == b[0], a[1] == b[1], a[2] == b[2], a[3] == b[3])
    }

    #[inline(always)]
    pub(crate) fn cmp_ne(self, rhs: Self) -> BVec4 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec4::new(a[0] != b[0], a[1] != b[1], a[2] != b[2], a[3] != b[3])
    }

    #[inline(always)]
    pub(crate) fn cmp_lt(self, rhs: Self) -> BVec4 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec4::new(a[0] < b[0], a[1] < b[1], a[2] < b[2], a[3] < b[3])
    }

    #[inline(always)]
    pub(crate) fn cmp_le(self, rhs: Self) -> BVec4 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec4::new(a[0] <= b[0], a[1] <= b[1], a[2] <= b[2], a[3] <= b[3])
    }

    #[inline(always)]
    pub(crate) fn cmp_gt(self, rhs: Self) -> BVec4 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec4::new(a[0] > b[0], a[1] > b[1], a[2] > b[2], a[3] > b[3])
    }

    #[inline(always)]
    pub(crate) fn cmp_ge(self, rhs: Self) -> BVec4 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec4::new(a[0] >= b[0], a[1] >= b[1], a[2] >= b[2], a[3] >= b[3])
    }
}

impl Default for Vec4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Vec4 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.cmp_eq(other.0).all4()
    }
}

impl Deref for Vec4 {
    type Target = crate::deref::Vec4<f32>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(unsafe_code, reason = "Vec4 begins with 4 contiguous f32 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for Vec4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(unsafe_code, reason = "Vec4 begins with 4 contiguous f32 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(Vec4))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline(always)]
    fn as_ref(&self) -> &[f32; 4] {
        #[expect(
            unsafe_code,
            reason = "Vec4 starts with 4 contiguous f32 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f32; 4]>())
        }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        #[expect(
            unsafe_code,
            reason = "Vec4 starts with 4 contiguous f32 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f32; 4]>())
        }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<Vec4> for f32 {
    type Output = Vec4;
    #[inline(always)]
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4::splat(self) + rhs
    }
}

impl AddAssign for Vec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Vec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0))
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<Vec4> for f32 {
    type Output = Vec4;
    #[inline(always)]
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4::splat(self) - rhs
    }
}

impl SubAssign for Vec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Vec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul(rhs.0))
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline(always)]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::splat(self) * rhs
    }
}

impl MulAssign for Vec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self(self.0.div(rhs.0))
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;
    #[inline(always)]
    fn div(self, rhs: Vec4) -> Vec4 {
        Vec4::splat(self) / rhs
    }
}

impl DivAssign for Vec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Rem for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0.rem(rhs.0))
    }
}

impl Rem<f32> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f32) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<Vec4> for f32 {
    type Output = Vec4;
    #[inline(always)]
    fn rem(self, rhs: Vec4) -> Vec4 {
        Vec4::splat(self) % rhs
    }
}

impl RemAssign for Vec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f32> for Vec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f32) {
        *self = *self % rhs;
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Sum for Vec4 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for Vec4 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for Vec4 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for Vec4 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline(always)]
    fn from(a: [f32; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline(always)]
    fn from(v: Vec4) -> Self {
        v.to_array()
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline(always)]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline(always)]
    fn from(v: Vec4) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2], a[3])
    }
}
