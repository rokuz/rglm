use crate::BVec4;
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
pub struct UVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

impl UVec4 {
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const MIN: Self = Self::splat(u32::MIN);
    pub const MAX: Self = Self::splat(u32::MAX);
    pub const X: Self = Self::new(1, 0, 0, 0);
    pub const Y: Self = Self::new(0, 1, 0, 0);
    pub const Z: Self = Self::new(0, 0, 1, 0);
    pub const W: Self = Self::new(0, 0, 0, 1);
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];
    #[inline(always)]
    pub(crate) const fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: u32) -> Self {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [u32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [u32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> u32 {
        self.x
            .wrapping_mul(rhs.x)
            .wrapping_add(self.y.wrapping_mul(rhs.y))
            .wrapping_add(self.z.wrapping_mul(rhs.z))
            .wrapping_add(self.w.wrapping_mul(rhs.w))
    }

    #[inline(always)]
    pub(crate) fn min(self, rhs: Self) -> Self {
        Self::new(
            self.x.min(rhs.x),
            self.y.min(rhs.y),
            self.z.min(rhs.z),
            self.w.min(rhs.w),
        )
    }

    #[inline(always)]
    pub(crate) fn max(self, rhs: Self) -> Self {
        Self::new(
            self.x.max(rhs.x),
            self.y.max(rhs.y),
            self.z.max(rhs.z),
            self.w.max(rhs.w),
        )
    }

    #[inline(always)]
    pub(crate) fn clamp(self, min_val: Self, max_val: Self) -> Self {
        self.max(min_val).min(max_val)
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

impl fmt::Debug for UVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(UVec4))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for UVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[u32; 4]> for UVec4 {
    #[inline(always)]
    fn as_ref(&self) -> &[u32; 4] {
        #[expect(
            unsafe_code,
            reason = "UVec4 starts with 4 contiguous u32 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[u32; 4]>())
        }
    }
}

impl AsMut<[u32; 4]> for UVec4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u32; 4] {
        #[expect(
            unsafe_code,
            reason = "UVec4 starts with 4 contiguous u32 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[u32; 4]>())
        }
    }
}

impl Index<usize> for UVec4 {
    type Output = u32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for UVec4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
            w: self.w.wrapping_add(rhs.w),
        }
    }
}

impl Add<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: u32) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn add(self, rhs: UVec4) -> UVec4 {
        UVec4::splat(self) + rhs
    }
}

impl AddAssign for UVec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<u32> for UVec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl Sub for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
            w: self.w.wrapping_sub(rhs.w),
        }
    }
}

impl Sub<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: u32) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn sub(self, rhs: UVec4) -> UVec4 {
        UVec4::splat(self) - rhs
    }
}

impl SubAssign for UVec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<u32> for UVec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}

impl Mul for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
            z: self.z.wrapping_mul(rhs.z),
            w: self.w.wrapping_mul(rhs.w),
        }
    }
}

impl Mul<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: u32) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn mul(self, rhs: UVec4) -> UVec4 {
        UVec4::splat(self) * rhs
    }
}

impl MulAssign for UVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<u32> for UVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 { 0 } else { self.x / rhs.x },
            y: if rhs.y == 0 { 0 } else { self.y / rhs.y },
            z: if rhs.z == 0 { 0 } else { self.z / rhs.z },
            w: if rhs.w == 0 { 0 } else { self.w / rhs.w },
        }
    }
}

impl Div<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: u32) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn div(self, rhs: UVec4) -> UVec4 {
        UVec4::splat(self) / rhs
    }
}

impl DivAssign for UVec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<u32> for UVec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Rem for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: if rhs.x == 0 { 0 } else { self.x % rhs.x },
            y: if rhs.y == 0 { 0 } else { self.y % rhs.y },
            z: if rhs.z == 0 { 0 } else { self.z % rhs.z },
            w: if rhs.w == 0 { 0 } else { self.w % rhs.w },
        }
    }
}

impl Rem<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: u32) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn rem(self, rhs: UVec4) -> UVec4 {
        UVec4::splat(self) % rhs
    }
}

impl RemAssign for UVec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<u32> for UVec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: u32) {
        *self = *self % rhs;
    }
}

impl BitAnd for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
            w: self.w & rhs.w,
        }
    }
}

impl BitAnd<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: u32) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for UVec4 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
            w: self.w | rhs.w,
        }
    }
}

impl BitOr<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: u32) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for UVec4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
            w: self.w ^ rhs.w,
        }
    }
}

impl BitXor<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: u32) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for UVec4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }
}

impl Shl<Self> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs.x),
            y: self.y.wrapping_shl(rhs.y),
            z: self.z.wrapping_shl(rhs.z),
            w: self.w.wrapping_shl(rhs.w),
        }
    }
}

impl Shl<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs),
            y: self.y.wrapping_shl(rhs),
            z: self.z.wrapping_shl(rhs),
            w: self.w.wrapping_shl(rhs),
        }
    }
}

impl ShlAssign<Self> for UVec4 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl ShlAssign<u32> for UVec4 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<Self> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs.x),
            y: self.y.wrapping_shr(rhs.y),
            z: self.z.wrapping_shr(rhs.z),
            w: self.w.wrapping_shr(rhs.w),
        }
    }
}

impl Shr<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs),
            y: self.y.wrapping_shr(rhs),
            z: self.z.wrapping_shr(rhs),
            w: self.w.wrapping_shr(rhs),
        }
    }
}

impl ShrAssign<Self> for UVec4 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl ShrAssign<u32> for UVec4 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl Sum for UVec4 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for UVec4 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for UVec4 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for UVec4 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[u32; 4]> for UVec4 {
    #[inline(always)]
    fn from(a: [u32; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<UVec4> for [u32; 4] {
    #[inline(always)]
    fn from(v: UVec4) -> Self {
        v.to_array()
    }
}

impl From<(u32, u32, u32, u32)> for UVec4 {
    #[inline(always)]
    fn from(t: (u32, u32, u32, u32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<UVec4> for (u32, u32, u32, u32) {
    #[inline(always)]
    fn from(v: UVec4) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2], a[3])
    }
}
