use crate::BVec3;
use crate::math::f32_math;
use crate::simd::SimdF32x4;
use crate::{BVec3A, Vec3};
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
pub struct Vec3A(pub(crate) SimdF32x4);

impl Vec3A {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const MIN: Self = Self::splat(f32::MIN);
    pub const MAX: Self = Self::splat(f32::MAX);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    #[inline(always)]
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(SimdF32x4::new(x, y, z, 0.0))
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f32; 3] {
        let a = self.0.to_array();
        [a[0], a[1], a[2]]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f32 {
        self.0.dot3(rhs.0)
    }

    #[inline(always)]
    pub(crate) fn cross(self, rhs: Self) -> Self {
        Self(self.0.cross3(rhs.0))
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
        Self(self.0.inversesqrt())
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
    pub(crate) fn select_mask(mask: BVec3, if_true: Self, if_false: Self) -> Self {
        Self(SimdF32x4::select(
            BVec3A::from(mask).0,
            if_true.0,
            if_false.0,
        ))
    }

    #[inline(always)]
    pub(crate) fn select_simd(mask: BVec3A, if_true: Self, if_false: Self) -> Self {
        Self(SimdF32x4::select(mask.0, if_true.0, if_false.0))
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

impl Default for Vec3A {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Vec3A {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.cmp_eq(other.0).all3()
    }
}

impl Deref for Vec3A {
    type Target = crate::deref::Vec3<f32>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        #[expect(unsafe_code, reason = "Vec3A begins with 3 contiguous f32 lanes")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<Self::Target>())
        }
    }
}

impl DerefMut for Vec3A {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(unsafe_code, reason = "Vec3A begins with 3 contiguous f32 lanes")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<Self::Target>())
        }
    }
}

impl fmt::Debug for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(Vec3A))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .finish()
    }
}

impl fmt::Display for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}]", a[0], a[1], a[2])
    }
}

impl AsRef<[f32; 3]> for Vec3A {
    #[inline(always)]
    fn as_ref(&self) -> &[f32; 3] {
        #[expect(
            unsafe_code,
            reason = "Vec3A starts with 3 contiguous f32 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f32; 3]>())
        }
    }
}

impl AsMut<[f32; 3]> for Vec3A {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        #[expect(
            unsafe_code,
            reason = "Vec3A starts with 3 contiguous f32 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f32; 3]>())
        }
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec3A {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }
}

impl Add<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f32) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn add(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self) + rhs
    }
}

impl AddAssign for Vec3A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Vec3A {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0))
    }
}

impl Sub<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f32) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn sub(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self) - rhs
    }
}

impl SubAssign for Vec3A {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Vec3A {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul(rhs.0))
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self) * rhs
    }
}

impl MulAssign for Vec3A {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        let r = SimdF32x4::new(rhs.x, rhs.y, rhs.z, 1.0);
        Self(self.0.div(r))
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn div(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self) / rhs
    }
}

impl DivAssign for Vec3A {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Rem for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        let r = SimdF32x4::new(rhs.x, rhs.y, rhs.z, 1.0);
        Self(self.0.rem(r))
    }
}

impl Rem<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f32) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn rem(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self) % rhs
    }
}

impl RemAssign for Vec3A {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f32> for Vec3A {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f32) {
        *self = *self % rhs;
    }
}

impl Neg for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        let v = self.0.neg().to_array();
        Self::new(v[0], v[1], v[2])
    }
}

impl Sum for Vec3A {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for Vec3A {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for Vec3A {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for Vec3A {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f32; 3]> for Vec3A {
    #[inline(always)]
    fn from(a: [f32; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<Vec3A> for [f32; 3] {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        v.to_array()
    }
}

impl From<(f32, f32, f32)> for Vec3A {
    #[inline(always)]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3A> for (f32, f32, f32) {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2])
    }
}

impl From<Vec3> for Vec3A {
    #[inline(always)]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
