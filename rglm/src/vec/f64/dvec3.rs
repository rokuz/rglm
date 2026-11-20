use crate::BVec3;
use crate::math::f64_math;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::Neg;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const MIN: Self = Self::splat(f64::MIN);
    pub const MAX: Self = Self::splat(f64::MAX);
    pub const NAN: Self = Self::splat(f64::NAN);
    pub const INFINITY: Self = Self::splat(f64::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    #[inline(always)]
    pub(crate) const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f64) -> Self {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f64; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    #[inline(always)]
    pub(crate) fn cross(self, rhs: Self) -> Self {
        Self::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
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
            f64_math::abs(self.x),
            f64_math::abs(self.y),
            f64_math::abs(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        Self::new(
            f64_math::sign(self.x),
            f64_math::sign(self.y),
            f64_math::sign(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        Self::new(
            f64_math::floor(self.x),
            f64_math::floor(self.y),
            f64_math::floor(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn ceil(self) -> Self {
        Self::new(
            f64_math::ceil(self.x),
            f64_math::ceil(self.y),
            f64_math::ceil(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn trunc(self) -> Self {
        Self::new(
            f64_math::trunc(self.x),
            f64_math::trunc(self.y),
            f64_math::trunc(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn round(self) -> Self {
        Self::new(
            f64_math::round(self.x),
            f64_math::round(self.y),
            f64_math::round(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn round_even(self) -> Self {
        Self::new(
            f64_math::round_even(self.x),
            f64_math::round_even(self.y),
            f64_math::round_even(self.z),
        )
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
        Self::new(
            f64_math::sqrt(self.x),
            f64_math::sqrt(self.y),
            f64_math::sqrt(self.z),
        )
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(self) -> Self {
        Self::ONE / self.sqrt()
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        Self::new(
            f64_math::mul_add(self.x, b.x, c.x),
            f64_math::mul_add(self.y, b.y, c.y),
            f64_math::mul_add(self.z, b.z, c.z),
        )
    }

    #[inline(always)]
    pub(crate) fn fma_exact(self, b: Self, c: Self) -> Self {
        let x = self.to_array();
        let y = b.to_array();
        let z = c.to_array();
        Self::new(
            f64_math::mul_add(x[0], y[0], z[0]),
            f64_math::mul_add(x[1], y[1], z[1]),
            f64_math::mul_add(x[2], y[2], z[2]),
        )
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

impl fmt::Debug for DVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(DVec3))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .finish()
    }
}

impl fmt::Display for DVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}]", a[0], a[1], a[2])
    }
}

impl AsRef<[f64; 3]> for DVec3 {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 3] {
        #[expect(
            unsafe_code,
            reason = "DVec3 starts with 3 contiguous f64 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f64; 3]>())
        }
    }
}

impl AsMut<[f64; 3]> for DVec3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 3] {
        #[expect(
            unsafe_code,
            reason = "DVec3 starts with 3 contiguous f64 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f64; 3]>())
        }
    }
}

impl Index<usize> for DVec3 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for DVec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f64) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<DVec3> for f64 {
    type Output = DVec3;
    #[inline(always)]
    fn add(self, rhs: DVec3) -> DVec3 {
        DVec3::splat(self) + rhs
    }
}

impl AddAssign for DVec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for DVec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f64) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<DVec3> for f64 {
    type Output = DVec3;
    #[inline(always)]
    fn sub(self, rhs: DVec3) -> DVec3 {
        DVec3::splat(self) - rhs
    }
}

impl SubAssign for DVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f64> for DVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl Mul for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<DVec3> for f64 {
    type Output = DVec3;
    #[inline(always)]
    fn mul(self, rhs: DVec3) -> DVec3 {
        DVec3::splat(self) * rhs
    }
}

impl MulAssign for DVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f64> for DVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<DVec3> for f64 {
    type Output = DVec3;
    #[inline(always)]
    fn div(self, rhs: DVec3) -> DVec3 {
        DVec3::splat(self) / rhs
    }
}

impl DivAssign for DVec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f64> for DVec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Rem for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }
}

impl Rem<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f64) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<DVec3> for f64 {
    type Output = DVec3;
    #[inline(always)]
    fn rem(self, rhs: DVec3) -> DVec3 {
        DVec3::splat(self) % rhs
    }
}

impl RemAssign for DVec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f64> for DVec3 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f64) {
        *self = *self % rhs;
    }
}

impl Neg for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sum for DVec3 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for DVec3 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for DVec3 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for DVec3 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f64; 3]> for DVec3 {
    #[inline(always)]
    fn from(a: [f64; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<DVec3> for [f64; 3] {
    #[inline(always)]
    fn from(v: DVec3) -> Self {
        v.to_array()
    }
}

impl From<(f64, f64, f64)> for DVec3 {
    #[inline(always)]
    fn from(t: (f64, f64, f64)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<DVec3> for (f64, f64, f64) {
    #[inline(always)]
    fn from(v: DVec3) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2])
    }
}
