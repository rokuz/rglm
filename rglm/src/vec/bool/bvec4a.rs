use crate::BVec4;
use crate::simd::SimdMask32x4;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct BVec4A(pub(crate) SimdMask32x4);

impl BVec4A {
    pub const FALSE: Self = Self::splat(false);
    pub const TRUE: Self = Self::splat(true);

    #[inline(always)]
    pub(crate) const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self(SimdMask32x4::new(x, y, z, w))
    }

    #[inline(always)]
    pub(crate) const fn splat(v: bool) -> Self {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [bool; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [bool; 4] {
        let a = self.0.to_bool_array();
        [a[0], a[1], a[2], a[3]]
    }

    #[inline(always)]
    pub(crate) fn bitmask(self) -> u32 {
        self.0.bitmask4()
    }

    #[inline(always)]
    pub(crate) fn any(self) -> bool {
        self.0.any4()
    }

    #[inline(always)]
    pub(crate) fn all(self) -> bool {
        self.0.all4()
    }

    #[inline(always)]
    pub(crate) const fn test(self, idx: usize) -> bool {
        self.to_array()[idx]
    }

    #[inline(always)]
    pub(crate) fn set(&mut self, idx: usize, val: bool) {
        self.0.set(idx, val);
    }
}

impl Default for BVec4A {
    #[inline(always)]
    fn default() -> Self {
        Self::FALSE
    }
}

impl PartialEq for BVec4A {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.bitmask() == other.bitmask()
    }
}

impl Eq for BVec4A {}

impl Hash for BVec4A {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_array().hash(state);
    }
}

impl fmt::Debug for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(BVec4A))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl Index<usize> for BVec4A {
    type Output = bool;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        static TRUE_VAL: bool = true;
        static FALSE_VAL: bool = false;
        if self.to_array()[index] {
            &TRUE_VAL
        } else {
            &FALSE_VAL
        }
    }
}

impl BitAnd for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0.bitand(rhs.0))
    }
}

impl BitAnd<bool> for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: bool) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for BVec4A {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0.bitor(rhs.0))
    }
}

impl BitOr<bool> for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: bool) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for BVec4A {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0.bitxor(rhs.0))
    }
}

impl BitXor<bool> for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: bool) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for BVec4A {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for BVec4A {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self(self.0.not())
    }
}

impl From<[bool; 4]> for BVec4A {
    #[inline(always)]
    fn from(a: [bool; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec4A> for [bool; 4] {
    #[inline(always)]
    fn from(v: BVec4A) -> Self {
        v.to_array()
    }
}

impl From<(bool, bool, bool, bool)> for BVec4A {
    #[inline(always)]
    fn from(t: (bool, bool, bool, bool)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<BVec4A> for (bool, bool, bool, bool) {
    #[inline(always)]
    fn from(v: BVec4A) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2], a[3])
    }
}

impl From<BVec4> for BVec4A {
    #[inline(always)]
    fn from(v: BVec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }
}
