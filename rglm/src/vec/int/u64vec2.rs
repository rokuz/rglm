use crate::BVec2;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct U64Vec2 {
    pub x: u64,
    pub y: u64,
}

impl U64Vec2 {
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const MIN: Self = Self::splat(u64::MIN);
    pub const MAX: Self = Self::splat(u64::MAX);
    pub const X: Self = Self::new(1, 0);
    pub const Y: Self = Self::new(0, 1);
    pub const AXES: [Self; 2] = [Self::X, Self::Y];
    #[inline(always)]
    pub(crate) const fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: u64) -> Self {
        Self::new(v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [u64; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [u64; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> u64 {
        self.x
            .wrapping_mul(rhs.x)
            .wrapping_add(self.y.wrapping_mul(rhs.y))
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

impl fmt::Debug for U64Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(U64Vec2))
            .field(&a[0])
            .field(&a[1])
            .finish()
    }
}

impl fmt::Display for U64Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}]", a[0], a[1])
    }
}

impl AsRef<[u64; 2]> for U64Vec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[u64; 2] {
        #[expect(
            unsafe_code,
            reason = "U64Vec2 starts with 2 contiguous u64 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[u64; 2]>())
        }
    }
}

impl AsMut<[u64; 2]> for U64Vec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u64; 2] {
        #[expect(
            unsafe_code,
            reason = "U64Vec2 starts with 2 contiguous u64 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[u64; 2]>())
        }
    }
}

impl Index<usize> for U64Vec2 {
    type Output = u64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for U64Vec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl Add<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: u64) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<U64Vec2> for u64 {
    type Output = U64Vec2;
    #[inline(always)]
    fn add(self, rhs: U64Vec2) -> U64Vec2 {
        U64Vec2::splat(self) + rhs
    }
}

impl AddAssign for U64Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<u64> for U64Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

impl Sub for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }
}

impl Sub<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: u64) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<U64Vec2> for u64 {
    type Output = U64Vec2;
    #[inline(always)]
    fn sub(self, rhs: U64Vec2) -> U64Vec2 {
        U64Vec2::splat(self) - rhs
    }
}

impl SubAssign for U64Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<u64> for U64Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

impl Mul for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
        }
    }
}

impl Mul<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: u64) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<U64Vec2> for u64 {
    type Output = U64Vec2;
    #[inline(always)]
    fn mul(self, rhs: U64Vec2) -> U64Vec2 {
        U64Vec2::splat(self) * rhs
    }
}

impl MulAssign for U64Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<u64> for U64Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u64) {
        *self = *self * rhs;
    }
}

impl Div for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 { 0 } else { self.x / rhs.x },
            y: if rhs.y == 0 { 0 } else { self.y / rhs.y },
        }
    }
}

impl Div<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: u64) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<U64Vec2> for u64 {
    type Output = U64Vec2;
    #[inline(always)]
    fn div(self, rhs: U64Vec2) -> U64Vec2 {
        U64Vec2::splat(self) / rhs
    }
}

impl DivAssign for U64Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<u64> for U64Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: u64) {
        *self = *self / rhs;
    }
}

impl Rem for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 { 0 } else { self.x % rhs.x },
            y: if rhs.y == 0 { 0 } else { self.y % rhs.y },
        }
    }
}

impl Rem<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: u64) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<U64Vec2> for u64 {
    type Output = U64Vec2;
    #[inline(always)]
    fn rem(self, rhs: U64Vec2) -> U64Vec2 {
        U64Vec2::splat(self) % rhs
    }
}

impl RemAssign for U64Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<u64> for U64Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: u64) {
        *self = *self % rhs;
    }
}

impl BitAnd for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }
}

impl BitAnd<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: u64) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for U64Vec2 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }
}

impl BitOr<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: u64) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for U64Vec2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}

impl BitXor<u64> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: u64) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for U64Vec2 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl Shl<Self> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs.x as u32),
            y: self.y.wrapping_shl(rhs.y as u32),
        }
    }
}

impl Shl<u32> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs),
            y: self.y.wrapping_shl(rhs),
        }
    }
}

impl ShlAssign<Self> for U64Vec2 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl ShlAssign<u32> for U64Vec2 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<Self> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs.x as u32),
            y: self.y.wrapping_shr(rhs.y as u32),
        }
    }
}

impl Shr<u32> for U64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs),
            y: self.y.wrapping_shr(rhs),
        }
    }
}

impl ShrAssign<Self> for U64Vec2 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl ShrAssign<u32> for U64Vec2 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl Sum for U64Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for U64Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for U64Vec2 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for U64Vec2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[u64; 2]> for U64Vec2 {
    #[inline(always)]
    fn from(a: [u64; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<U64Vec2> for [u64; 2] {
    #[inline(always)]
    fn from(v: U64Vec2) -> Self {
        v.to_array()
    }
}

impl From<(u64, u64)> for U64Vec2 {
    #[inline(always)]
    fn from(t: (u64, u64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<U64Vec2> for (u64, u64) {
    #[inline(always)]
    fn from(v: U64Vec2) -> Self {
        let a = v.to_array();
        (a[0], a[1])
    }
}
