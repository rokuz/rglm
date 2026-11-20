use crate::BVec2;
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
#[repr(transparent)]
pub struct DVec2(pub(crate) SimdF64x2);

impl DVec2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const MIN: Self = Self::splat(f64::MIN);
    pub const MAX: Self = Self::splat(f64::MAX);
    pub const NAN: Self = Self::splat(f64::NAN);
    pub const INFINITY: Self = Self::splat(f64::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const AXES: [Self; 2] = [Self::X, Self::Y];
    pub const NEG_X: Self = Self::new(-1.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0);
    #[inline(always)]
    pub(crate) const fn new(x: f64, y: f64) -> Self {
        Self(SimdF64x2::new(x, y))
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f64) -> Self {
        Self::new(v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f64; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f64; 2] {
        self.0.to_array()
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f64 {
        self.0.dot2(rhs.0)
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
        Self::ONE / self.sqrt()
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
    pub(crate) fn select_mask(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        let m = mask.to_array();
        let t = if_true.to_array();
        let f = if_false.to_array();
        Self::new(
            if m[0] { t[0] } else { f[0] },
            if m[1] { t[1] } else { f[1] },
        )
    }

    #[inline(always)]
    pub(crate) fn cmp_eq(self, rhs: Self) -> BVec2 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec2::new(a[0] == b[0], a[1] == b[1])
    }

    #[inline(always)]
    pub(crate) fn cmp_ne(self, rhs: Self) -> BVec2 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec2::new(a[0] != b[0], a[1] != b[1])
    }

    #[inline(always)]
    pub(crate) fn cmp_lt(self, rhs: Self) -> BVec2 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec2::new(a[0] < b[0], a[1] < b[1])
    }

    #[inline(always)]
    pub(crate) fn cmp_le(self, rhs: Self) -> BVec2 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec2::new(a[0] <= b[0], a[1] <= b[1])
    }

    #[inline(always)]
    pub(crate) fn cmp_gt(self, rhs: Self) -> BVec2 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec2::new(a[0] > b[0], a[1] > b[1])
    }

    #[inline(always)]
    pub(crate) fn cmp_ge(self, rhs: Self) -> BVec2 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec2::new(a[0] >= b[0], a[1] >= b[1])
    }
}

impl Default for DVec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for DVec2 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_array() == other.to_array()
    }
}

impl Deref for DVec2 {
    type Target = crate::deref::Vec2<f64>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(unsafe_code, reason = "DVec2 begins with 2 contiguous f64 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for DVec2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(unsafe_code, reason = "DVec2 begins with 2 contiguous f64 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for DVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(DVec2))
            .field(&a[0])
            .field(&a[1])
            .finish()
    }
}

impl fmt::Display for DVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}]", a[0], a[1])
    }
}

impl AsRef<[f64; 2]> for DVec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 2] {
        #[expect(
            unsafe_code,
            reason = "DVec2 starts with 2 contiguous f64 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f64; 2]>())
        }
    }
}

impl AsMut<[f64; 2]> for DVec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 2] {
        #[expect(
            unsafe_code,
            reason = "DVec2 starts with 2 contiguous f64 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f64; 2]>())
        }
    }
}

impl Index<usize> for DVec2 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DVec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }
}

impl Add<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<DVec2> for f64 {
    type Output = DVec2;
    #[inline(always)]
    fn add(self, rhs: DVec2) -> DVec2 {
        DVec2::splat(self) + rhs
    }
}

impl AddAssign for DVec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DVec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0))
    }
}

impl Sub<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<DVec2> for f64 {
    type Output = DVec2;
    #[inline(always)]
    fn sub(self, rhs: DVec2) -> DVec2 {
        DVec2::splat(self) - rhs
    }
}

impl SubAssign for DVec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DVec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Mul for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul(rhs.0))
    }
}

impl Mul<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<DVec2> for f64 {
    type Output = DVec2;
    #[inline(always)]
    fn mul(self, rhs: DVec2) -> DVec2 {
        DVec2::splat(self) * rhs
    }
}

impl MulAssign for DVec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f64> for DVec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self(self.0.div(rhs.0))
    }
}

impl Div<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<DVec2> for f64 {
    type Output = DVec2;
    #[inline(always)]
    fn div(self, rhs: DVec2) -> DVec2 {
        DVec2::splat(self) / rhs
    }
}

impl DivAssign for DVec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f64> for DVec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Rem for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0.rem(rhs.0))
    }
}

impl Rem<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f64) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<DVec2> for f64 {
    type Output = DVec2;
    #[inline(always)]
    fn rem(self, rhs: DVec2) -> DVec2 {
        DVec2::splat(self) % rhs
    }
}

impl RemAssign for DVec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f64> for DVec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f64) {
        *self = *self % rhs;
    }
}

impl Neg for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Sum for DVec2 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for DVec2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for DVec2 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for DVec2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f64; 2]> for DVec2 {
    #[inline(always)]
    fn from(a: [f64; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<DVec2> for [f64; 2] {
    #[inline(always)]
    fn from(v: DVec2) -> Self {
        v.to_array()
    }
}

impl From<(f64, f64)> for DVec2 {
    #[inline(always)]
    fn from(t: (f64, f64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<DVec2> for (f64, f64) {
    #[inline(always)]
    fn from(v: DVec2) -> Self {
        let a = v.to_array();
        (a[0], a[1])
    }
}
