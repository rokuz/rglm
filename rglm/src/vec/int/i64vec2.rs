use crate::BVec2;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::Neg;
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
pub struct I64Vec2 {
    pub x: i64,
    pub y: i64,
}

impl I64Vec2 {
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const NEG_ONE: Self = Self::splat(-1);
    pub const MIN: Self = Self::splat(i64::MIN);
    pub const MAX: Self = Self::splat(i64::MAX);
    pub const X: Self = Self::new(1, 0);
    pub const Y: Self = Self::new(0, 1);
    pub const AXES: [Self; 2] = [Self::X, Self::Y];
    pub const NEG_X: Self = Self::new(-1, 0);
    pub const NEG_Y: Self = Self::new(0, -1);
    #[inline(always)]
    pub(crate) const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: i64) -> Self {
        Self::new(v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [i64; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [i64; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> i64 {
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
    pub(crate) fn abs(self) -> Self {
        Self::new(self.x.wrapping_abs(), self.y.wrapping_abs())
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        Self::new(
            if self.x > 0 {
                1
            } else if self.x < 0 {
                -1
            } else {
                0
            },
            if self.y > 0 {
                1
            } else if self.y < 0 {
                -1
            } else {
                0
            },
        )
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

impl fmt::Debug for I64Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(I64Vec2))
            .field(&a[0])
            .field(&a[1])
            .finish()
    }
}

impl fmt::Display for I64Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}]", a[0], a[1])
    }
}

impl AsRef<[i64; 2]> for I64Vec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[i64; 2] {
        #[expect(
            unsafe_code,
            reason = "I64Vec2 starts with 2 contiguous i64 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[i64; 2]>())
        }
    }
}

impl AsMut<[i64; 2]> for I64Vec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [i64; 2] {
        #[expect(
            unsafe_code,
            reason = "I64Vec2 starts with 2 contiguous i64 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[i64; 2]>())
        }
    }
}

impl Index<usize> for I64Vec2 {
    type Output = i64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for I64Vec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl Add<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: i64) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<I64Vec2> for i64 {
    type Output = I64Vec2;
    #[inline(always)]
    fn add(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2::splat(self) + rhs
    }
}

impl AddAssign for I64Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i64> for I64Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl Sub for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }
}

impl Sub<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: i64) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<I64Vec2> for i64 {
    type Output = I64Vec2;
    #[inline(always)]
    fn sub(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2::splat(self) - rhs
    }
}

impl SubAssign for I64Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i64> for I64Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl Mul for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
        }
    }
}

impl Mul<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: i64) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<I64Vec2> for i64 {
    type Output = I64Vec2;
    #[inline(always)]
    fn mul(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2::splat(self) * rhs
    }
}

impl MulAssign for I64Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i64> for I64Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl Div for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 {
                0
            } else {
                self.x.wrapping_div(rhs.x)
            },
            y: if rhs.y == 0 {
                0
            } else {
                self.y.wrapping_div(rhs.y)
            },
        }
    }
}

impl Div<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i64) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<I64Vec2> for i64 {
    type Output = I64Vec2;
    #[inline(always)]
    fn div(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2::splat(self) / rhs
    }
}

impl DivAssign for I64Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i64> for I64Vec2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl Rem for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 {
                0
            } else {
                self.x.wrapping_rem(rhs.x)
            },
            y: if rhs.y == 0 {
                0
            } else {
                self.y.wrapping_rem(rhs.y)
            },
        }
    }
}

impl Rem<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: i64) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<I64Vec2> for i64 {
    type Output = I64Vec2;
    #[inline(always)]
    fn rem(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2::splat(self) % rhs
    }
}

impl RemAssign for I64Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<i64> for I64Vec2 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: i64) {
        *self = *self % rhs;
    }
}

impl Neg for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: self.x.wrapping_neg(),
            y: self.y.wrapping_neg(),
        }
    }
}

impl BitAnd for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }
}

impl BitAnd<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: i64) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for I64Vec2 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }
}

impl BitOr<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: i64) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for I64Vec2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}

impl BitXor<i64> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: i64) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for I64Vec2 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl Shl<Self> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs.x as u32),
            y: self.y.wrapping_shl(rhs.y as u32),
        }
    }
}

impl Shl<u32> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs),
            y: self.y.wrapping_shl(rhs),
        }
    }
}

impl ShlAssign<Self> for I64Vec2 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl ShlAssign<u32> for I64Vec2 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<Self> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs.x as u32),
            y: self.y.wrapping_shr(rhs.y as u32),
        }
    }
}

impl Shr<u32> for I64Vec2 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs),
            y: self.y.wrapping_shr(rhs),
        }
    }
}

impl ShrAssign<Self> for I64Vec2 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl ShrAssign<u32> for I64Vec2 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl Sum for I64Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for I64Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for I64Vec2 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for I64Vec2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[i64; 2]> for I64Vec2 {
    #[inline(always)]
    fn from(a: [i64; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<I64Vec2> for [i64; 2] {
    #[inline(always)]
    fn from(v: I64Vec2) -> Self {
        v.to_array()
    }
}

impl From<(i64, i64)> for I64Vec2 {
    #[inline(always)]
    fn from(t: (i64, i64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<I64Vec2> for (i64, i64) {
    #[inline(always)]
    fn from(v: I64Vec2) -> Self {
        let a = v.to_array();
        (a[0], a[1])
    }
}
