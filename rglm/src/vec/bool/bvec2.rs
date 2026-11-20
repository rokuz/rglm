use core::fmt;
use core::ops::IndexMut;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}

impl BVec2 {
    pub const FALSE: Self = Self::splat(false);
    pub const TRUE: Self = Self::splat(true);

    #[inline(always)]
    pub(crate) const fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub(crate) const fn splat(v: bool) -> Self {
        Self::new(v, v)
    }

    #[inline(always)]
    pub(crate) const fn from_array(a: [bool; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline(always)]
    pub(crate) const fn to_array(self) -> [bool; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    pub(crate) fn bitmask(self) -> u32 {
        (self.x as u32) | ((self.y as u32) << 1)
    }

    #[inline(always)]
    pub(crate) fn any(self) -> bool {
        self.x || self.y
    }

    #[inline(always)]
    pub(crate) fn all(self) -> bool {
        self.x && self.y
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

impl fmt::Debug for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        f.debug_tuple(stringify!(BVec2))
            .field(&a[0])
            .field(&a[1])
            .finish()
    }
}

impl fmt::Display for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_array();
        write!(f, "[{}, {}]", a[0], a[1])
    }
}

impl Index<usize> for BVec2 {
    type Output = bool;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        let slice: &[bool; 2] = self.as_ref();
        &slice[index]
    }
}

impl IndexMut<usize> for BVec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice: &mut [bool; 2] = self.as_mut();
        &mut slice[index]
    }
}

impl AsRef<[bool; 2]> for BVec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[bool; 2] {
        #[expect(unsafe_code, reason = "BVec2 is repr(C) of 2 contiguous bool fields")]
        unsafe {
            &*(core::ptr::from_ref(self).cast::<[bool; 2]>())
        }
    }
}

impl AsMut<[bool; 2]> for BVec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [bool; 2] {
        #[expect(unsafe_code, reason = "BVec2 is repr(C) of 2 contiguous bool fields")]
        unsafe {
            &mut *(core::ptr::from_mut(self).cast::<[bool; 2]>())
        }
    }
}

impl BitAnd for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }
}

impl BitAnd<bool> for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: bool) -> Self {
        self & Self::splat(rhs)
    }
}

impl BitAndAssign for BVec2 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }
}

impl BitOr<bool> for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: bool) -> Self {
        self | Self::splat(rhs)
    }
}

impl BitOrAssign for BVec2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}

impl BitXor<bool> for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: bool) -> Self {
        self ^ Self::splat(rhs)
    }
}

impl BitXorAssign for BVec2 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for BVec2 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl From<[bool; 2]> for BVec2 {
    #[inline(always)]
    fn from(a: [bool; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec2> for [bool; 2] {
    #[inline(always)]
    fn from(v: BVec2) -> Self {
        v.to_array()
    }
}

impl From<(bool, bool)> for BVec2 {
    #[inline(always)]
    fn from(t: (bool, bool)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<BVec2> for (bool, bool) {
    #[inline(always)]
    fn from(v: BVec2) -> Self {
        let a = v.to_array();
        (a[0], a[1])
    }
}
