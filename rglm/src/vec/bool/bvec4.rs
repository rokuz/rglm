use crate::BVec4A;
use core::fmt;
use core::ops::IndexMut;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BVec4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

impl BVec4 {
    pub const FALSE: Self = Self::splat(false);
    pub const TRUE: Self = Self::splat(true);

    #[inline(always)]
    pub(crate) const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self { x, y, z, w }
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
        [self.x, self.y, self.z, self.w]
    }

    #[inline(always)]
    pub(crate) fn bitmask(self) -> u32 {
        (self.x as u32) | ((self.y as u32) << 1) | ((self.z as u32) << 2) | ((self.w as u32) << 3)
    }

    #[inline(always)]
    pub(crate) fn any(self) -> bool {
        self.x || self.y || self.z || self.w
    }

    #[inline(always)]
    pub(crate) fn all(self) -> bool {
        self.x && self.y && self.z && self.w
    }

    #[inline(always)]
    pub(crate) const fn test(self, idx: usize) -> bool {
        self.to_array()[idx]
    }

    #[inline(always)]
    pub(crate) fn set(&mut self, idx: usize, val: bool) {
        self.as_mut()[idx] = val;
    }
}

impl fmt::Debug for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(BVec4))
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl Index<usize> for BVec4 {
    type Output = bool;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        let slice: &[bool; 4] = self.as_ref();
        &slice[index]
    }
}

impl IndexMut<usize> for BVec4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice: &mut [bool; 4] = self.as_mut();
        &mut slice[index]
    }
}

impl AsRef<[bool; 4]> for BVec4 {
    #[inline(always)]
    fn as_ref(&self) -> &[bool; 4] {
        #[expect(unsafe_code, reason = "BVec4 is repr(C) of 4 contiguous bool fields")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[bool; 4]>())
        }
    }
}

impl AsMut<[bool; 4]> for BVec4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [bool; 4] {
        #[expect(unsafe_code, reason = "BVec4 is repr(C) of 4 contiguous bool fields")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[bool; 4]>())
        }
    }
}

impl BitAnd for BVec4 {
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

impl BitAnd<bool> for BVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: bool) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for BVec4 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for BVec4 {
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

impl BitOr<bool> for BVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: bool) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for BVec4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for BVec4 {
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

impl BitXor<bool> for BVec4 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: bool) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for BVec4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for BVec4 {
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

impl From<[bool; 4]> for BVec4 {
    #[inline(always)]
    fn from(a: [bool; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec4> for [bool; 4] {
    #[inline(always)]
    fn from(v: BVec4) -> Self {
        v.to_array()
    }
}

impl From<(bool, bool, bool, bool)> for BVec4 {
    #[inline(always)]
    fn from(t: (bool, bool, bool, bool)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<BVec4> for (bool, bool, bool, bool) {
    #[inline(always)]
    fn from(v: BVec4) -> Self {
        let a = v.to_array();
        (a[0], a[1], a[2], a[3])
    }
}

impl From<BVec4A> for BVec4 {
    #[inline(always)]
    fn from(v: BVec4A) -> Self {
        let a = v.to_array();
        Self::new(a[0], a[1], a[2], a[3])
    }
}
