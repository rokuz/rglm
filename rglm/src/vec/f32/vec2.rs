use crate::BVec2;
use crate::math::f32_math;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::Neg;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const MIN: Self = Self::splat(f32::MIN);
    pub const MAX: Self = Self::splat(f32::MAX);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const AXES: [Self; 2] = [Self::X, Self::Y];
    pub const NEG_X: Self = Self::new(-1.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0);
    #[inline(always)]
    pub(crate) const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f32) -> Self {
        Self::new(v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    #[inline(always)]
    pub(crate) fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    #[inline(always)]
    pub(crate) fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }

    #[inline(always)]
    pub(crate) fn clamp(self, min_val: Self, max_val: Self) -> Self {
        self.max(min_val).min(max_val)
    }

    #[inline(always)]
    pub(crate) fn abs(self) -> Self {
        Self::new(f32_math::abs(self.x), f32_math::abs(self.y))
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        Self::new(f32_math::sign(self.x), f32_math::sign(self.y))
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        Self::new(f32_math::floor(self.x), f32_math::floor(self.y))
    }

    #[inline(always)]
    pub(crate) fn ceil(self) -> Self {
        Self::new(f32_math::ceil(self.x), f32_math::ceil(self.y))
    }

    #[inline(always)]
    pub(crate) fn trunc(self) -> Self {
        Self::new(f32_math::trunc(self.x), f32_math::trunc(self.y))
    }

    #[inline(always)]
    pub(crate) fn round(self) -> Self {
        Self::new(f32_math::round(self.x), f32_math::round(self.y))
    }

    #[inline(always)]
    pub(crate) fn round_even(self) -> Self {
        Self::new(f32_math::round_even(self.x), f32_math::round_even(self.y))
    }

    #[inline(always)]
    pub(crate) fn fract(self) -> Self {
        self - self.floor()
    }

    #[inline(always)]
    pub(crate) fn modulo(self, rhs: Self) -> Self {
        self - (rhs * (self / rhs).floor())
    }

    #[inline(always)]
    pub(crate) fn sqrt(self) -> Self {
        Self::new(f32_math::sqrt(self.x), f32_math::sqrt(self.y))
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(self) -> Self {
        Self::ONE / self.sqrt()
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        Self::new(
            f32_math::mul_add(self.x, b.x, c.x),
            f32_math::mul_add(self.y, b.y, c.y),
        )
    }

    #[inline(always)]
    pub(crate) fn fma_exact(self, b: Self, c: Self) -> Self {
        let x = self.to_array();
        let y = b.to_array();
        let z = c.to_array();
        Self::new(
            f32_math::mul_add(x[0], y[0], z[0]),
            f32_math::mul_add(x[1], y[1], z[1]),
        )
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

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(Vec2))
            .field(&a[0])
            .field(&a[1])
            .finish()
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}]", a[0], a[1])
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[f32; 2] {
        #[expect(
            unsafe_code,
            reason = "Vec2 starts with 2 contiguous f32 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f32; 2]>())
        }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        #[expect(
            unsafe_code,
            reason = "Vec2 starts with 2 contiguous f32 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f32; 2]>())
        }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::splat(self) + rhs
    }
}

impl AddAssign for Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::splat(self) - rhs
    }
}

impl SubAssign for Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::splat(self) * rhs
    }
}

impl MulAssign for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2::splat(self) / rhs
    }
}

impl DivAssign for Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Rem for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Rem<f32> for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f32) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<Vec2> for f32 {
    type Output = Vec2;
    #[inline(always)]
    fn rem(self, rhs: Vec2) -> Vec2 {
        Vec2::splat(self) % rhs
    }
}

impl RemAssign for Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f32> for Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f32) {
        *self = *self % rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sum for Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for Vec2 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for Vec2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline(always)]
    fn from(a: [f32; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline(always)]
    fn from(v: Vec2) -> Self {
        v.to_array()
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline(always)]
    fn from(t: (f32, f32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline(always)]
    fn from(v: Vec2) -> Self {
        let a = v.to_array();
        (a[0], a[1])
    }
}
