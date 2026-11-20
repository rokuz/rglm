use crate::BVec3;
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
pub struct U8Vec3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl U8Vec3 {
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const MIN: Self = Self::splat(u8::MIN);
    pub const MAX: Self = Self::splat(u8::MAX);
    pub const X: Self = Self::new(1, 0, 0);
    pub const Y: Self = Self::new(0, 1, 0);
    pub const Z: Self = Self::new(0, 0, 1);
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    #[inline(always)]
    pub(crate) const fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: u8) -> Self {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [u8; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [u8; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> u8 {
        self.x
            .wrapping_mul(rhs.x)
            .wrapping_add(self.y.wrapping_mul(rhs.y))
            .wrapping_add(self.z.wrapping_mul(rhs.z))
    }

    #[inline(always)]
    pub(crate) fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    #[inline(always)]
    pub(crate) fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    #[inline(always)]
    pub(crate) fn clamp(self, min_val: Self, max_val: Self) -> Self {
        self.max(min_val).min(max_val)
    }

    #[inline(always)]
    pub(crate) fn select_mask(mask: BVec3, if_true: Self, if_false: Self) -> Self {
        let m = mask.to_array();
        let t = if_true.to_array();
        let f = if_false.to_array();
        Self::new(
            if m[0] { t[0] } else { f[0] },
            if m[1] { t[1] } else { f[1] },
            if m[2] { t[2] } else { f[2] },
        )
    }

    #[inline(always)]
    pub(crate) fn cmp_eq(self, rhs: Self) -> BVec3 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec3::new(a[0] == b[0], a[1] == b[1], a[2] == b[2])
    }

    #[inline(always)]
    pub(crate) fn cmp_ne(self, rhs: Self) -> BVec3 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec3::new(a[0] != b[0], a[1] != b[1], a[2] != b[2])
    }

    #[inline(always)]
    pub(crate) fn cmp_lt(self, rhs: Self) -> BVec3 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec3::new(a[0] < b[0], a[1] < b[1], a[2] < b[2])
    }

    #[inline(always)]
    pub(crate) fn cmp_le(self, rhs: Self) -> BVec3 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec3::new(a[0] <= b[0], a[1] <= b[1], a[2] <= b[2])
    }

    #[inline(always)]
    pub(crate) fn cmp_gt(self, rhs: Self) -> BVec3 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec3::new(a[0] > b[0], a[1] > b[1], a[2] > b[2])
    }

    #[inline(always)]
    pub(crate) fn cmp_ge(self, rhs: Self) -> BVec3 {
        let a = self.to_array();
        let b = rhs.to_array();
        BVec3::new(a[0] >= b[0], a[1] >= b[1], a[2] >= b[2])
    }
}

impl fmt::Debug for U8Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(U8Vec3))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .finish()
    }
}

impl fmt::Display for U8Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}]", a[0], a[1], a[2])
    }
}

impl AsRef<[u8; 3]> for U8Vec3 {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; 3] {
        #[expect(
            unsafe_code,
            reason = "U8Vec3 starts with 3 contiguous u8 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[u8; 3]>())
        }
    }
}

impl AsMut<[u8; 3]> for U8Vec3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8; 3] {
        #[expect(
            unsafe_code,
            reason = "U8Vec3 starts with 3 contiguous u8 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[u8; 3]>())
        }
    }
}

impl Index<usize> for U8Vec3 {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for U8Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
        }
    }
}

impl Add<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: u8) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<U8Vec3> for u8 {
    type Output = U8Vec3;
    #[inline(always)]
    fn add(self, rhs: U8Vec3) -> U8Vec3 {
        U8Vec3::splat(self) + rhs
    }
}

impl AddAssign for U8Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<u8> for U8Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
        }
    }
}

impl Sub<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: u8) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<U8Vec3> for u8 {
    type Output = U8Vec3;
    #[inline(always)]
    fn sub(self, rhs: U8Vec3) -> U8Vec3 {
        U8Vec3::splat(self) - rhs
    }
}

impl SubAssign for U8Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<u8> for U8Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl Mul for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
            z: self.z.wrapping_mul(rhs.z),
        }
    }
}

impl Mul<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: u8) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<U8Vec3> for u8 {
    type Output = U8Vec3;
    #[inline(always)]
    fn mul(self, rhs: U8Vec3) -> U8Vec3 {
        U8Vec3::splat(self) * rhs
    }
}

impl MulAssign for U8Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<u8> for U8Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u8) {
        *self = *self * rhs;
    }
}

impl Div for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 { 0 } else { self.x / rhs.x },
            y: if rhs.y == 0 { 0 } else { self.y / rhs.y },
            z: if rhs.z == 0 { 0 } else { self.z / rhs.z },
        }
    }
}

impl Div<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: u8) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<U8Vec3> for u8 {
    type Output = U8Vec3;
    #[inline(always)]
    fn div(self, rhs: U8Vec3) -> U8Vec3 {
        U8Vec3::splat(self) / rhs
    }
}

impl DivAssign for U8Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<u8> for U8Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: u8) {
        *self = *self / rhs;
    }
}

impl Rem for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 { 0 } else { self.x % rhs.x },
            y: if rhs.y == 0 { 0 } else { self.y % rhs.y },
            z: if rhs.z == 0 { 0 } else { self.z % rhs.z },
        }
    }
}

impl Rem<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: u8) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<U8Vec3> for u8 {
    type Output = U8Vec3;
    #[inline(always)]
    fn rem(self, rhs: U8Vec3) -> U8Vec3 {
        U8Vec3::splat(self) % rhs
    }
}

impl RemAssign for U8Vec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<u8> for U8Vec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: u8) {
        *self = *self % rhs;
    }
}

impl BitAnd for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
        }
    }
}

impl BitAnd<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: u8) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for U8Vec3 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }
}

impl BitOr<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: u8) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for U8Vec3 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }
}

impl BitXor<u8> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: u8) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for U8Vec3 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

impl Shl<Self> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs.x as u32),
            y: self.y.wrapping_shl(rhs.y as u32),
            z: self.z.wrapping_shl(rhs.z as u32),
        }
    }
}

impl Shl<u32> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs),
            y: self.y.wrapping_shl(rhs),
            z: self.z.wrapping_shl(rhs),
        }
    }
}

impl ShlAssign<Self> for U8Vec3 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl ShlAssign<u32> for U8Vec3 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<Self> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs.x as u32),
            y: self.y.wrapping_shr(rhs.y as u32),
            z: self.z.wrapping_shr(rhs.z as u32),
        }
    }
}

impl Shr<u32> for U8Vec3 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs),
            y: self.y.wrapping_shr(rhs),
            z: self.z.wrapping_shr(rhs),
        }
    }
}

impl ShrAssign<Self> for U8Vec3 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl ShrAssign<u32> for U8Vec3 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl Sum for U8Vec3 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for U8Vec3 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for U8Vec3 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for U8Vec3 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[u8; 3]> for U8Vec3 {
    #[inline(always)]
    fn from(a: [u8; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<U8Vec3> for [u8; 3] {
    #[inline(always)]
    fn from(v: U8Vec3) -> Self {
        v.to_array()
    }
}

impl From<(u8, u8, u8)> for U8Vec3 {
    #[inline(always)]
    fn from(t: (u8, u8, u8)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<U8Vec3> for (u8, u8, u8) {
    #[inline(always)]
    fn from(v: U8Vec3) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2])
    }
}
