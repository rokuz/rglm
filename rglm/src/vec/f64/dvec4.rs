use crate::BVec4;
use crate::math::f64_math;
use crate::simd::SimdF64x2;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::Neg;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub struct DVec4(pub(crate) [SimdF64x2; 2]);

impl DVec4 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const MIN: Self = Self::splat(f64::MIN);
    pub const MAX: Self = Self::splat(f64::MAX);
    pub const NAN: Self = Self::splat(f64::NAN);
    pub const INFINITY: Self = Self::splat(f64::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);
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
    pub(crate) const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self([SimdF64x2::new(x, y), SimdF64x2::new(z, w)])
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f64) -> Self {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f64; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f64; 4] {
        let a0 = self.0[0].to_array();
        let a1 = self.0[1].to_array();
        [a0[0], a0[1], a1[0], a1[1]]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f64 {
        self.0[0].dot2(rhs.0[0]) + self.0[1].dot2(rhs.0[1])
    }

    #[inline(always)]
    pub(crate) fn min(self, rhs: Self) -> Self {
        Self([self.0[0].min(rhs.0[0]), self.0[1].min(rhs.0[1])])
    }

    #[inline(always)]
    pub(crate) fn max(self, rhs: Self) -> Self {
        Self([self.0[0].max(rhs.0[0]), self.0[1].max(rhs.0[1])])
    }

    #[inline(always)]
    pub(crate) fn clamp(self, min_val: Self, max_val: Self) -> Self {
        self.max(min_val).min(max_val)
    }

    #[inline(always)]
    pub(crate) fn abs(self) -> Self {
        Self([self.0[0].abs(), self.0[1].abs()])
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        Self([self.0[0].sign(), self.0[1].sign()])
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        Self([self.0[0].floor(), self.0[1].floor()])
    }

    #[inline(always)]
    pub(crate) fn ceil(self) -> Self {
        Self([self.0[0].ceil(), self.0[1].ceil()])
    }

    #[inline(always)]
    pub(crate) fn trunc(self) -> Self {
        Self([self.0[0].trunc(), self.0[1].trunc()])
    }

    #[inline(always)]
    pub(crate) fn round(self) -> Self {
        Self([self.0[0].round(), self.0[1].round()])
    }

    #[inline(always)]
    pub(crate) fn round_even(self) -> Self {
        Self([self.0[0].round_even(), self.0[1].round_even()])
    }

    #[inline(always)]
    pub(crate) fn fract(self) -> Self {
        Self([self.0[0].fract(), self.0[1].fract()])
    }

    #[inline(always)]
    pub(crate) fn modulo(self, rhs: Self) -> Self {
        self - (rhs * (self / rhs).floor())
    }

    #[inline(always)]
    pub(crate) fn sqrt(self) -> Self {
        Self([self.0[0].sqrt(), self.0[1].sqrt()])
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(self) -> Self {
        Self::ONE / self.sqrt()
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        Self([
            self.0[0].mul_add(b.0[0], c.0[0]),
            self.0[1].mul_add(b.0[1], c.0[1]),
        ])
    }

    #[inline(always)]
    pub(crate) fn fma_exact(self, b: Self, c: Self) -> Self {
        Self([
            self.0[0].fma_exact(b.0[0], c.0[0]),
            self.0[1].fma_exact(b.0[1], c.0[1]),
        ])
    }

    #[inline(always)]
    pub(crate) fn length(self) -> f64 {
        f64_math::sqrt(self.dot(self))
    }

    #[inline(always)]
    pub(crate) fn normalize(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > 0.0);
        self * f64_math::inversesqrt(len_sq)
    }

    #[inline(always)]
    pub(crate) fn select_mask(mask: BVec4, if_true: Self, if_false: Self) -> Self {
        let m = mask.to_array();
        let t = if_true.to_array();
        let f = if_false.to_array();
        Self::new(
            if m[0] { t[0] } else { f[0] },
            if m[1] { t[1] } else { f[1] },
            if m[2] { t[2] } else { f[2] },
            if m[3] { t[3] } else { f[3] },
        )
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

impl Default for DVec4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for DVec4 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_array() == other.to_array()
    }
}

impl Deref for DVec4 {
    type Target = crate::deref::Vec4<f64>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(unsafe_code, reason = "DVec4 begins with 4 contiguous f64 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for DVec4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(unsafe_code, reason = "DVec4 begins with 4 contiguous f64 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for DVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(DVec4))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for DVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[f64; 4]> for DVec4 {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 4] {
        #[expect(
            unsafe_code,
            reason = "DVec4 starts with 4 contiguous f64 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f64; 4]>())
        }
    }
}

impl AsMut<[f64; 4]> for DVec4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 4] {
        #[expect(
            unsafe_code,
            reason = "DVec4 starts with 4 contiguous f64 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f64; 4]>())
        }
    }
}

impl Index<usize> for DVec4 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DVec4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self([self.0[0].add(rhs.0[0]), self.0[1].add(rhs.0[1])])
    }
}

impl Add<f64> for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<DVec4> for f64 {
    type Output = DVec4;
    #[inline(always)]
    fn add(self, rhs: DVec4) -> DVec4 {
        DVec4::splat(self) + rhs
    }
}

impl AddAssign for DVec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DVec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self([self.0[0].sub(rhs.0[0]), self.0[1].sub(rhs.0[1])])
    }
}

impl Sub<f64> for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<DVec4> for f64 {
    type Output = DVec4;
    #[inline(always)]
    fn sub(self, rhs: DVec4) -> DVec4 {
        DVec4::splat(self) - rhs
    }
}

impl SubAssign for DVec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DVec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Mul for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self([self.0[0].mul(rhs.0[0]), self.0[1].mul(rhs.0[1])])
    }
}

impl Mul<f64> for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<DVec4> for f64 {
    type Output = DVec4;
    #[inline(always)]
    fn mul(self, rhs: DVec4) -> DVec4 {
        DVec4::splat(self) * rhs
    }
}

impl MulAssign for DVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f64> for DVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self([self.0[0].div(rhs.0[0]), self.0[1].div(rhs.0[1])])
    }
}

impl Div<f64> for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<DVec4> for f64 {
    type Output = DVec4;
    #[inline(always)]
    fn div(self, rhs: DVec4) -> DVec4 {
        DVec4::splat(self) / rhs
    }
}

impl DivAssign for DVec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f64> for DVec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Rem for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self([self.0[0].rem(rhs.0[0]), self.0[1].rem(rhs.0[1])])
    }
}

impl Rem<f64> for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f64) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<DVec4> for f64 {
    type Output = DVec4;
    #[inline(always)]
    fn rem(self, rhs: DVec4) -> DVec4 {
        DVec4::splat(self) % rhs
    }
}

impl RemAssign for DVec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f64> for DVec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f64) {
        *self = *self % rhs;
    }
}

impl Neg for DVec4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self([self.0[0].neg(), self.0[1].neg()])
    }
}

impl Sum for DVec4 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for DVec4 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for DVec4 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for DVec4 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f64; 4]> for DVec4 {
    #[inline(always)]
    fn from(a: [f64; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<DVec4> for [f64; 4] {
    #[inline(always)]
    fn from(v: DVec4) -> Self {
        v.to_array()
    }
}

impl From<(f64, f64, f64, f64)> for DVec4 {
    #[inline(always)]
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<DVec4> for (f64, f64, f64, f64) {
    #[inline(always)]
    fn from(v: DVec4) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2], a[3])
    }
}
