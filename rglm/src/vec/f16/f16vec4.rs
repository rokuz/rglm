use crate::BVec4;
use crate::math::f16_math;
use crate::simd::SimdF16x4;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::Neg;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use half::f16;

#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct F16Vec4 {
    pub x: f16,
    pub y: f16,
    pub z: f16,
    pub w: f16,
}

impl F16Vec4 {
    pub const ZERO: Self = Self::splat(f16::ZERO);
    pub const ONE: Self = Self::splat(f16::ONE);
    pub const NEG_ONE: Self = Self::splat(f16::NEG_ONE);
    pub const MIN: Self = Self::splat(f16::MIN);
    pub const MAX: Self = Self::splat(f16::MAX);
    pub const NAN: Self = Self::splat(f16::NAN);
    pub const INFINITY: Self = Self::splat(f16::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f16::NEG_INFINITY);
    pub const X: Self = Self::new(f16::ONE, f16::ZERO, f16::ZERO, f16::ZERO);
    pub const Y: Self = Self::new(f16::ZERO, f16::ONE, f16::ZERO, f16::ZERO);
    pub const Z: Self = Self::new(f16::ZERO, f16::ZERO, f16::ONE, f16::ZERO);
    pub const W: Self = Self::new(f16::ZERO, f16::ZERO, f16::ZERO, f16::ONE);
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];
    pub const NEG_X: Self = Self::new(f16::NEG_ONE, f16::ZERO, f16::ZERO, f16::ZERO);
    pub const NEG_Y: Self = Self::new(f16::ZERO, f16::NEG_ONE, f16::ZERO, f16::ZERO);
    pub const NEG_Z: Self = Self::new(f16::ZERO, f16::ZERO, f16::NEG_ONE, f16::ZERO);
    pub const NEG_W: Self = Self::new(f16::ZERO, f16::ZERO, f16::ZERO, f16::NEG_ONE);
    #[inline(always)]
    pub(crate) const fn new(x: f16, y: f16, z: f16, w: f16) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: f16) -> Self {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [f16; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [f16; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f16 {
        let p = (self * rhs).to_array();
        (p[0] + p[2]) + (p[1] + p[3])
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
    pub(crate) fn abs(self) -> Self {
        Self::new(
            f16_math::abs(self.x),
            f16_math::abs(self.y),
            f16_math::abs(self.z),
            f16_math::abs(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn sign(self) -> Self {
        Self::new(
            f16_math::sign(self.x),
            f16_math::sign(self.y),
            f16_math::sign(self.z),
            f16_math::sign(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn floor(self) -> Self {
        Self::new(
            f16_math::floor(self.x),
            f16_math::floor(self.y),
            f16_math::floor(self.z),
            f16_math::floor(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn ceil(self) -> Self {
        Self::new(
            f16_math::ceil(self.x),
            f16_math::ceil(self.y),
            f16_math::ceil(self.z),
            f16_math::ceil(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn trunc(self) -> Self {
        Self::new(
            f16_math::trunc(self.x),
            f16_math::trunc(self.y),
            f16_math::trunc(self.z),
            f16_math::trunc(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn round(self) -> Self {
        Self::new(
            f16_math::round(self.x),
            f16_math::round(self.y),
            f16_math::round(self.z),
            f16_math::round(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn round_even(self) -> Self {
        Self::new(
            f16_math::round_even(self.x),
            f16_math::round_even(self.y),
            f16_math::round_even(self.z),
            f16_math::round_even(self.w),
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
            f16_math::sqrt(self.x),
            f16_math::sqrt(self.y),
            f16_math::sqrt(self.z),
            f16_math::sqrt(self.w),
        )
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(self) -> Self {
        Self::ONE / self.sqrt()
    }

    #[inline(always)]
    pub(crate) fn mul_add(self, b: Self, c: Self) -> Self {
        let r = SimdF16x4([self.x, self.y, self.z, self.w])
            .mul_add(
                SimdF16x4([b.x, b.y, b.z, b.w]),
                SimdF16x4([c.x, c.y, c.z, c.w]),
            )
            .to_array();
        Self::new(r[0], r[1], r[2], r[3])
    }

    #[inline(always)]
    pub(crate) fn fma_exact(self, b: Self, c: Self) -> Self {
        let x = self.to_array();
        let y = b.to_array();
        let z = c.to_array();
        Self::new(
            f16_math::mul_add(x[0], y[0], z[0]),
            f16_math::mul_add(x[1], y[1], z[1]),
            f16_math::mul_add(x[2], y[2], z[2]),
            f16_math::mul_add(x[3], y[3], z[3]),
        )
    }

    #[inline(always)]
    pub(crate) fn length(self) -> f16 {
        f16_math::sqrt(self.dot(self))
    }

    #[inline(always)]
    pub(crate) fn normalize(self) -> Self {
        let len_sq = self.dot(self);
        rglm_debug_assert!(len_sq > f16::ZERO);
        self * f16_math::inversesqrt(len_sq)
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

impl fmt::Debug for F16Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(F16Vec4))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for F16Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl AsRef<[f16; 4]> for F16Vec4 {
    #[inline(always)]
    fn as_ref(&self) -> &[f16; 4] {
        #[expect(
            unsafe_code,
            reason = "F16Vec4 starts with 4 contiguous f16 lanes with sufficient alignment"
        )]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[f16; 4]>())
        }
    }
}

impl AsMut<[f16; 4]> for F16Vec4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f16; 4] {
        #[expect(
            unsafe_code,
            reason = "F16Vec4 starts with 4 contiguous f16 lanes with sufficient alignment"
        )]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[f16; 4]>())
        }
    }
}

impl Index<usize> for F16Vec4 {
    type Output = f16;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for F16Vec4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl Add for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let r = SimdF16x4([self.x, self.y, self.z, self.w])
            .add(SimdF16x4([rhs.x, rhs.y, rhs.z, rhs.w]))
            .to_array();
        Self::new(r[0], r[1], r[2], r[3])
    }
}

impl Add<f16> for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: f16) -> Self {
        self + Self::splat(rhs)
    }
}

impl Add<F16Vec4> for f16 {
    type Output = F16Vec4;
    #[inline(always)]
    fn add(self, rhs: F16Vec4) -> F16Vec4 {
        F16Vec4::splat(self) + rhs
    }
}

impl AddAssign for F16Vec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f16> for F16Vec4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f16) {
        *self = *self + rhs;
    }
}

impl Sub for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        let r = SimdF16x4([self.x, self.y, self.z, self.w])
            .sub(SimdF16x4([rhs.x, rhs.y, rhs.z, rhs.w]))
            .to_array();
        Self::new(r[0], r[1], r[2], r[3])
    }
}

impl Sub<f16> for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: f16) -> Self {
        self - Self::splat(rhs)
    }
}

impl Sub<F16Vec4> for f16 {
    type Output = F16Vec4;
    #[inline(always)]
    fn sub(self, rhs: F16Vec4) -> F16Vec4 {
        F16Vec4::splat(self) - rhs
    }
}

impl SubAssign for F16Vec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f16> for F16Vec4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f16) {
        *self = *self - rhs;
    }
}

impl Mul for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        let r = SimdF16x4([self.x, self.y, self.z, self.w])
            .mul(SimdF16x4([rhs.x, rhs.y, rhs.z, rhs.w]))
            .to_array();
        Self::new(r[0], r[1], r[2], r[3])
    }
}

impl Mul<f16> for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f16) -> Self {
        self * Self::splat(rhs)
    }
}

impl Mul<F16Vec4> for f16 {
    type Output = F16Vec4;
    #[inline(always)]
    fn mul(self, rhs: F16Vec4) -> F16Vec4 {
        F16Vec4::splat(self) * rhs
    }
}

impl MulAssign for F16Vec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f16> for F16Vec4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f16) {
        *self = *self * rhs;
    }
}

impl Div for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        let r = SimdF16x4([self.x, self.y, self.z, self.w])
            .div(SimdF16x4([rhs.x, rhs.y, rhs.z, rhs.w]))
            .to_array();
        Self::new(r[0], r[1], r[2], r[3])
    }
}

impl Div<f16> for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f16) -> Self {
        self / Self::splat(rhs)
    }
}

impl Div<F16Vec4> for f16 {
    type Output = F16Vec4;
    #[inline(always)]
    fn div(self, rhs: F16Vec4) -> F16Vec4 {
        F16Vec4::splat(self) / rhs
    }
}

impl DivAssign for F16Vec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f16> for F16Vec4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f16) {
        *self = *self / rhs;
    }
}

impl Rem for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
            w: self.w % rhs.w,
        }
    }
}

impl Rem<f16> for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: f16) -> Self {
        self % Self::splat(rhs)
    }
}

impl Rem<F16Vec4> for f16 {
    type Output = F16Vec4;
    #[inline(always)]
    fn rem(self, rhs: F16Vec4) -> F16Vec4 {
        F16Vec4::splat(self) % rhs
    }
}

impl RemAssign for F16Vec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<f16> for F16Vec4 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: f16) {
        *self = *self % rhs;
    }
}

impl Neg for F16Vec4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Sum for F16Vec4 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for F16Vec4 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &x| acc + x)
    }
}

impl Product for F16Vec4 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for F16Vec4 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, &x| acc * x)
    }
}

impl From<[f16; 4]> for F16Vec4 {
    #[inline(always)]
    fn from(a: [f16; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<F16Vec4> for [f16; 4] {
    #[inline(always)]
    fn from(v: F16Vec4) -> Self {
        v.to_array()
    }
}

impl From<(f16, f16, f16, f16)> for F16Vec4 {
    #[inline(always)]
    fn from(t: (f16, f16, f16, f16)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<F16Vec4> for (f16, f16, f16, f16) {
    #[inline(always)]
    fn from(v: F16Vec4) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2], a[3])
    }
}
