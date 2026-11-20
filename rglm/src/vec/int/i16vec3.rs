use crate::BVec3;
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
pub struct I16Vec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl I16Vec3 {
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const NEG_ONE: Self = Self::splat(-1);
    pub const MIN: Self = Self::splat(i16::MIN);
    pub const MAX: Self = Self::splat(i16::MAX);
    pub const X: Self = Self::new(1, 0, 0);
    pub const Y: Self = Self::new(0, 1, 0);
    pub const Z: Self = Self::new(0, 0, 1);
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    pub const NEG_Y: Self = Self::new(0, -1, 0);
    pub const NEG_Z: Self = Self::new(0, 0, -1);
    #[inline(always)]
    pub(crate) const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: i16) -> Self {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [i16; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [i16; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> i16 {
        self.x
            .wrapping_mul(rhs.x)
            .wrapping_add(self.y.wrapping_mul(rhs.y))
            .wrapping_add(self.z.wrapping_mul(rhs.z))
    }

    #[inline(always)]
    pub(crate) fn cross(self, rhs: Self) -> Self {
        Self {
            x: self
                .y
                .wrapping_mul(rhs.z)
                .wrapping_sub(self.z.wrapping_mul(rhs.y)),
            y: self
                .z
                .wrapping_mul(rhs.x)
                .wrapping_sub(self.x.wrapping_mul(rhs.z)),
            z: self
                .x
                .wrapping_mul(rhs.y)
                .wrapping_sub(self.y.wrapping_mul(rhs.x)),
        }
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
    pub(crate) fn abs(self) -> Self {
        Self::new(
            self.x.wrapping_abs(),
            self.y.wrapping_abs(),
            self.z.wrapping_abs(),
        )
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
            if self.z > 0 {
                1
            } else if self.z < 0 {
                -1
            } else {
                0
            },
        )
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

impl fmt::Debug for I16Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(I16Vec3))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .finish()
    }
}

impl fmt::Display for I16Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}]", a[0], a[1], a[2])
    }
}

impl AsRef<[i16; 3]> for I16Vec3 {
    #[inline(always)]
    fn as_ref(&self) -> &[i16; 3] {
        #[expect(
            unsafe_code,
            reason = "I16Vec3 starts with 3 contiguous i16 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[i16; 3]>())
        }
    }
}

impl AsMut<[i16; 3]> for I16Vec3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [i16; 3] {
        #[expect(
            unsafe_code,
            reason = "I16Vec3 starts with 3 contiguous i16 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[i16; 3]>())
        }
    }
}

impl Index<usize> for I16Vec3 {
    type Output = i16;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for I16Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for I16Vec3 {
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

impl Add<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: i16) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<I16Vec3> for i16 {
    type Output = I16Vec3;
    #[inline(always)]
    fn add(self, rhs: I16Vec3) -> I16Vec3 {
        I16Vec3::splat(self) + rhs
    }
}

impl AddAssign for I16Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i16> for I16Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: i16) {
        *self = *self + rhs;
    }
}

impl Sub for I16Vec3 {
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

impl Sub<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: i16) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<I16Vec3> for i16 {
    type Output = I16Vec3;
    #[inline(always)]
    fn sub(self, rhs: I16Vec3) -> I16Vec3 {
        I16Vec3::splat(self) - rhs
    }
}

impl SubAssign for I16Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i16> for I16Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: i16) {
        *self = *self - rhs;
    }
}

impl Mul for I16Vec3 {
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

impl Mul<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: i16) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<I16Vec3> for i16 {
    type Output = I16Vec3;
    #[inline(always)]
    fn mul(self, rhs: I16Vec3) -> I16Vec3 {
        I16Vec3::splat(self) * rhs
    }
}

impl MulAssign for I16Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i16> for I16Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: i16) {
        *self = *self * rhs;
    }
}

impl Div for I16Vec3 {
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
            z: if rhs.z == 0 {
                0
            } else {
                self.z.wrapping_div(rhs.z)
            },
        }
    }
}

impl Div<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i16) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<I16Vec3> for i16 {
    type Output = I16Vec3;
    #[inline(always)]
    fn div(self, rhs: I16Vec3) -> I16Vec3 {
        I16Vec3::splat(self) / rhs
    }
}

impl DivAssign for I16Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i16> for I16Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: i16) {
        *self = *self / rhs;
    }
}

impl Rem for I16Vec3 {
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
            z: if rhs.z == 0 {
                0
            } else {
                self.z.wrapping_rem(rhs.z)
            },
        }
    }
}

impl Rem<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: i16) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<I16Vec3> for i16 {
    type Output = I16Vec3;
    #[inline(always)]
    fn rem(self, rhs: I16Vec3) -> I16Vec3 {
        I16Vec3::splat(self) % rhs
    }
}

impl RemAssign for I16Vec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<i16> for I16Vec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: i16) {
        *self = *self % rhs;
    }
}

impl Neg for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: self.x.wrapping_neg(),
            y: self.y.wrapping_neg(),
            z: self.z.wrapping_neg(),
        }
    }
}

impl BitAnd for I16Vec3 {
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

impl BitAnd<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: i16) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for I16Vec3 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for I16Vec3 {
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

impl BitOr<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: i16) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for I16Vec3 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for I16Vec3 {
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

impl BitXor<i16> for I16Vec3 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: i16) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for I16Vec3 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for I16Vec3 {
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

impl Shl<Self> for I16Vec3 {
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

impl Shl<u32> for I16Vec3 {
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

impl ShlAssign<Self> for I16Vec3 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl ShlAssign<u32> for I16Vec3 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<Self> for I16Vec3 {
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

impl Shr<u32> for I16Vec3 {
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

impl ShrAssign<Self> for I16Vec3 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl ShrAssign<u32> for I16Vec3 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl Sum for I16Vec3 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for I16Vec3 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for I16Vec3 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for I16Vec3 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[i16; 3]> for I16Vec3 {
    #[inline(always)]
    fn from(a: [i16; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<I16Vec3> for [i16; 3] {
    #[inline(always)]
    fn from(v: I16Vec3) -> Self {
        v.to_array()
    }
}

impl From<(i16, i16, i16)> for I16Vec3 {
    #[inline(always)]
    fn from(t: (i16, i16, i16)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<I16Vec3> for (i16, i16, i16) {
    #[inline(always)]
    fn from(v: I16Vec3) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2])
    }
}
