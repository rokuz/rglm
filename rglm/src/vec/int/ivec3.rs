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
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IVec3 {
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const NEG_ONE: Self = Self::splat(-1);
    pub const MIN: Self = Self::splat(i32::MIN);
    pub const MAX: Self = Self::splat(i32::MAX);
    pub const X: Self = Self::new(1, 0, 0);
    pub const Y: Self = Self::new(0, 1, 0);
    pub const Z: Self = Self::new(0, 0, 1);
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    pub const NEG_Y: Self = Self::new(0, -1, 0);
    pub const NEG_Z: Self = Self::new(0, 0, -1);
    #[inline(always)]
    pub(crate) const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: i32) -> Self {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [i32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> i32 {
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

impl fmt::Debug for IVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(IVec3))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .finish()
    }
}

impl fmt::Display for IVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}]", a[0], a[1], a[2])
    }
}

impl AsRef<[i32; 3]> for IVec3 {
    #[inline(always)]
    fn as_ref(&self) -> &[i32; 3] {
        #[expect(
            unsafe_code,
            reason = "IVec3 starts with 3 contiguous i32 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[i32; 3]>())
        }
    }
}

impl AsMut<[i32; 3]> for IVec3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [i32; 3] {
        #[expect(
            unsafe_code,
            reason = "IVec3 starts with 3 contiguous i32 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[i32; 3]>())
        }
    }
}

impl Index<usize> for IVec3 {
    type Output = i32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for IVec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for IVec3 {
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

impl Add<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: i32) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<IVec3> for i32 {
    type Output = IVec3;
    #[inline(always)]
    fn add(self, rhs: IVec3) -> IVec3 {
        IVec3::splat(self) + rhs
    }
}

impl AddAssign for IVec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i32> for IVec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub for IVec3 {
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

impl Sub<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: i32) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<IVec3> for i32 {
    type Output = IVec3;
    #[inline(always)]
    fn sub(self, rhs: IVec3) -> IVec3 {
        IVec3::splat(self) - rhs
    }
}

impl SubAssign for IVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i32> for IVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl Mul for IVec3 {
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

impl Mul<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: i32) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<IVec3> for i32 {
    type Output = IVec3;
    #[inline(always)]
    fn mul(self, rhs: IVec3) -> IVec3 {
        IVec3::splat(self) * rhs
    }
}

impl MulAssign for IVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i32> for IVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl Div for IVec3 {
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

impl Div<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i32) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<IVec3> for i32 {
    type Output = IVec3;
    #[inline(always)]
    fn div(self, rhs: IVec3) -> IVec3 {
        IVec3::splat(self) / rhs
    }
}

impl DivAssign for IVec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i32> for IVec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl Rem for IVec3 {
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

impl Rem<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: i32) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<IVec3> for i32 {
    type Output = IVec3;
    #[inline(always)]
    fn rem(self, rhs: IVec3) -> IVec3 {
        IVec3::splat(self) % rhs
    }
}

impl RemAssign for IVec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<i32> for IVec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: i32) {
        *self = *self % rhs;
    }
}

impl Neg for IVec3 {
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

impl BitAnd for IVec3 {
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

impl BitAnd<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: i32) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for IVec3 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for IVec3 {
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

impl BitOr<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: i32) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for IVec3 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for IVec3 {
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

impl BitXor<i32> for IVec3 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: i32) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for IVec3 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for IVec3 {
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

impl Shl<Self> for IVec3 {
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

impl Shl<u32> for IVec3 {
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

impl ShlAssign<Self> for IVec3 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl ShlAssign<u32> for IVec3 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<Self> for IVec3 {
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

impl Shr<u32> for IVec3 {
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

impl ShrAssign<Self> for IVec3 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl ShrAssign<u32> for IVec3 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl Sum for IVec3 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for IVec3 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for IVec3 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for IVec3 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[i32; 3]> for IVec3 {
    #[inline(always)]
    fn from(a: [i32; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<IVec3> for [i32; 3] {
    #[inline(always)]
    fn from(v: IVec3) -> Self {
        v.to_array()
    }
}

impl From<(i32, i32, i32)> for IVec3 {
    #[inline(always)]
    fn from(t: (i32, i32, i32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<IVec3> for (i32, i32, i32) {
    #[inline(always)]
    fn from(v: IVec3) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2])
    }
}
