#[doc(hidden)]
pub mod mask;

use core::fmt;
use core::ops::{Deref, DerefMut, IndexMut};

mod private {
    pub trait Sealed {}
}

trait SetLane<S> {
    fn set_lane(&mut self, i: usize, v: S);
}

impl<S, T: IndexMut<usize, Output = S>> SetLane<S> for T {
    #[inline(always)]
    fn set_lane(&mut self, i: usize, v: S) {
        self[i] = v;
    }
}

macro_rules! impl_set_lane_via_array {
    ($($t:ty),+) => {
        $(
            impl SetLane<bool> for $t {
                #[inline(always)]
                fn set_lane(&mut self, i: usize, v: bool) {
                    let mut a = self.to_array();
                    a[i] = v;
                    *self = Self::from_array(a);
                }
            }
        )+
    };
}

impl_set_lane_via_array!(crate::BVec3A, crate::BVec4A);

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M1<const X: usize>;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M2<const X: usize, const Y: usize>;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M3<const X: usize, const Y: usize, const Z: usize>;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4<const X: usize, const Y: usize, const Z: usize, const W: usize>;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lane<const I: usize>;

#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "swizzle component lane is out of bounds for this vector dimension"
)]
pub trait InBounds<const DIM: usize> {}

impl InBounds<2> for Lane<0> {}
impl InBounds<2> for Lane<1> {}

impl InBounds<3> for Lane<0> {}
impl InBounds<3> for Lane<1> {}
impl InBounds<3> for Lane<2> {}

impl InBounds<4> for Lane<0> {}
impl InBounds<4> for Lane<1> {}
impl InBounds<4> for Lane<2> {}
impl InBounds<4> for Lane<3> {}

#[diagnostic::on_unimplemented(
    message = "swizzle mask contains components outside this vector's dimension"
)]
pub trait ValidForDim<const DIM: usize>: Copy {}

impl<const X: usize, const DIM: usize> ValidForDim<DIM> for M1<X> where Lane<X>: InBounds<DIM> {}

impl<const X: usize, const Y: usize, const DIM: usize> ValidForDim<DIM> for M2<X, Y>
where
    Lane<X>: InBounds<DIM>,
    Lane<Y>: InBounds<DIM>,
{
}

impl<const X: usize, const Y: usize, const Z: usize, const DIM: usize> ValidForDim<DIM>
    for M3<X, Y, Z>
where
    Lane<X>: InBounds<DIM>,
    Lane<Y>: InBounds<DIM>,
    Lane<Z>: InBounds<DIM>,
{
}

impl<const X: usize, const Y: usize, const Z: usize, const W: usize, const DIM: usize>
    ValidForDim<DIM> for M4<X, Y, Z, W>
where
    Lane<X>: InBounds<DIM>,
    Lane<Y>: InBounds<DIM>,
    Lane<Z>: InBounds<DIM>,
    Lane<W>: InBounds<DIM>,
{
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pair<const A: usize, const B: usize>;

#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "write swizzle mask cannot contain duplicate component lanes"
)]
pub trait Distinct {}

impl Distinct for Pair<0, 1> {}
impl Distinct for Pair<0, 2> {}
impl Distinct for Pair<0, 3> {}
impl Distinct for Pair<1, 0> {}
impl Distinct for Pair<1, 2> {}
impl Distinct for Pair<1, 3> {}
impl Distinct for Pair<2, 0> {}
impl Distinct for Pair<2, 1> {}
impl Distinct for Pair<2, 3> {}
impl Distinct for Pair<3, 0> {}
impl Distinct for Pair<3, 1> {}
impl Distinct for Pair<3, 2> {}

#[diagnostic::on_unimplemented(
    message = "write swizzle mask cannot contain duplicate component lanes"
)]
pub trait UniqueLanes: Copy {}

impl<const X: usize> UniqueLanes for M1<X> {}

impl<const X: usize, const Y: usize> UniqueLanes for M2<X, Y> where Pair<X, Y>: Distinct {}

impl<const X: usize, const Y: usize, const Z: usize> UniqueLanes for M3<X, Y, Z>
where
    Pair<X, Y>: Distinct,
    Pair<X, Z>: Distinct,
    Pair<Y, Z>: Distinct,
{
}

impl<const X: usize, const Y: usize, const Z: usize, const W: usize> UniqueLanes for M4<X, Y, Z, W>
where
    Pair<X, Y>: Distinct,
    Pair<X, Z>: Distinct,
    Pair<X, W>: Distinct,
    Pair<Y, Z>: Distinct,
    Pair<Y, W>: Distinct,
    Pair<Z, W>: Distinct,
{
}

#[doc(hidden)]
pub trait SwzRead<M>: private::Sealed + Copy {
    type Out: Copy;
    fn swz_read(self, mask: M) -> Self::Out;
}

#[doc(hidden)]
pub trait SwzWrite<M: UniqueLanes>: SwzRead<M> {
    fn swz_write(&mut self, mask: M, val: Self::Out);
}

#[doc(hidden)]
pub struct SwzGuard<'a, V: SwzWrite<M>, M: UniqueLanes> {
    target: &'a mut V,
    mask: M,
    sub: V::Out,
}

impl<V: SwzWrite<M>, M: UniqueLanes> fmt::Debug for SwzGuard<'_, V, M>
where
    V::Out: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SwzGuard").field(&self.sub).finish()
    }
}

impl<V: SwzWrite<M>, M: UniqueLanes> Deref for SwzGuard<'_, V, M> {
    type Target = V::Out;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.sub
    }
}

impl<V: SwzWrite<M>, M: UniqueLanes> DerefMut for SwzGuard<'_, V, M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sub
    }
}

impl<V: SwzWrite<M>, M: UniqueLanes> Drop for SwzGuard<'_, V, M> {
    #[inline(always)]
    fn drop(&mut self) {
        self.target.swz_write(self.mask, self.sub);
    }
}

/// Evaluates a read-only vector swizzle mask; prefer invoking [`crate::rswz!`].
#[doc(hidden)]
#[inline(always)]
#[must_use]
pub fn read<V: SwzRead<M>, M>(v: V, mask: M) -> V::Out {
    v.swz_read(mask)
}

/// Creates a mutable swizzle write guard; prefer invoking [`crate::swz!`].
#[doc(hidden)]
#[inline(always)]
#[must_use]
pub fn write<V: SwzWrite<M>, M: UniqueLanes>(target: &mut V, mask: M) -> SwzGuard<'_, V, M> {
    let sub = target.swz_read(mask);
    SwzGuard { target, mask, sub }
}

macro_rules! impl_swizzle_family {
    ($vec_t:ty, $dim:literal, $s:ty, $v2:ty, $v3:ty, $v4:ty) => {
        impl private::Sealed for $vec_t {}

        impl<const X: usize> SwzRead<M1<X>> for $vec_t
        where
            M1<X>: ValidForDim<$dim>,
        {
            type Out = $s;

            #[inline(always)]
            fn swz_read(self, _: M1<X>) -> $s {
                self.to_array()[X]
            }
        }

        impl<const X: usize> SwzWrite<M1<X>> for $vec_t
        where
            M1<X>: ValidForDim<$dim> + UniqueLanes,
        {
            #[inline(always)]
            fn swz_write(&mut self, _: M1<X>, val: $s) {
                self.set_lane(X, val);
            }
        }

        impl<const X: usize, const Y: usize> SwzRead<M2<X, Y>> for $vec_t
        where
            M2<X, Y>: ValidForDim<$dim>,
        {
            type Out = $v2;

            #[inline(always)]
            fn swz_read(self, _: M2<X, Y>) -> $v2 {
                let a = self.to_array();
                <$v2>::from_array([a[X], a[Y]])
            }
        }

        impl<const X: usize, const Y: usize> SwzWrite<M2<X, Y>> for $vec_t
        where
            M2<X, Y>: ValidForDim<$dim> + UniqueLanes,
        {
            #[inline(always)]
            fn swz_write(&mut self, _: M2<X, Y>, val: $v2) {
                let s = val.to_array();
                self.set_lane(X, s[0]);
                self.set_lane(Y, s[1]);
            }
        }

        impl<const X: usize, const Y: usize, const Z: usize> SwzRead<M3<X, Y, Z>> for $vec_t
        where
            M3<X, Y, Z>: ValidForDim<$dim>,
        {
            type Out = $v3;

            #[inline(always)]
            fn swz_read(self, _: M3<X, Y, Z>) -> $v3 {
                let a = self.to_array();
                <$v3>::from_array([a[X], a[Y], a[Z]])
            }
        }

        impl<const X: usize, const Y: usize, const Z: usize> SwzWrite<M3<X, Y, Z>> for $vec_t
        where
            M3<X, Y, Z>: ValidForDim<$dim> + UniqueLanes,
        {
            #[inline(always)]
            fn swz_write(&mut self, _: M3<X, Y, Z>, val: $v3) {
                let s = val.to_array();
                self.set_lane(X, s[0]);
                self.set_lane(Y, s[1]);
                self.set_lane(Z, s[2]);
            }
        }

        impl<const X: usize, const Y: usize, const Z: usize, const W: usize> SwzRead<M4<X, Y, Z, W>>
            for $vec_t
        where
            M4<X, Y, Z, W>: ValidForDim<$dim>,
        {
            type Out = $v4;

            #[inline(always)]
            fn swz_read(self, _: M4<X, Y, Z, W>) -> $v4 {
                let a = self.to_array();
                <$v4>::from_array([a[X], a[Y], a[Z], a[W]])
            }
        }

        impl<const X: usize, const Y: usize, const Z: usize, const W: usize>
            SwzWrite<M4<X, Y, Z, W>> for $vec_t
        where
            M4<X, Y, Z, W>: ValidForDim<$dim> + UniqueLanes,
        {
            #[inline(always)]
            fn swz_write(&mut self, _: M4<X, Y, Z, W>, val: $v4) {
                let s = val.to_array();
                self.set_lane(X, s[0]);
                self.set_lane(Y, s[1]);
                self.set_lane(Z, s[2]);
                self.set_lane(W, s[3]);
            }
        }
    };
}

impl_swizzle_family!(crate::Vec2, 2, f32, crate::Vec2, crate::Vec3, crate::Vec4);
impl_swizzle_family!(crate::Vec3, 3, f32, crate::Vec2, crate::Vec3, crate::Vec4);
impl_swizzle_family!(crate::Vec3A, 3, f32, crate::Vec2, crate::Vec3A, crate::Vec4);
impl_swizzle_family!(crate::Vec4, 4, f32, crate::Vec2, crate::Vec3, crate::Vec4);

impl_swizzle_family!(
    crate::BVec2,
    2,
    bool,
    crate::BVec2,
    crate::BVec3,
    crate::BVec4
);
impl_swizzle_family!(
    crate::BVec3,
    3,
    bool,
    crate::BVec2,
    crate::BVec3,
    crate::BVec4
);
impl_swizzle_family!(
    crate::BVec3A,
    3,
    bool,
    crate::BVec2,
    crate::BVec3A,
    crate::BVec4A
);
impl_swizzle_family!(
    crate::BVec4,
    4,
    bool,
    crate::BVec2,
    crate::BVec3,
    crate::BVec4
);
impl_swizzle_family!(
    crate::BVec4A,
    4,
    bool,
    crate::BVec2,
    crate::BVec3A,
    crate::BVec4A
);

#[cfg(feature = "f64")]
impl_swizzle_family!(
    crate::DVec2,
    2,
    f64,
    crate::DVec2,
    crate::DVec3,
    crate::DVec4
);
#[cfg(feature = "f64")]
impl_swizzle_family!(
    crate::DVec3,
    3,
    f64,
    crate::DVec2,
    crate::DVec3,
    crate::DVec4
);
#[cfg(feature = "f64")]
impl_swizzle_family!(
    crate::DVec4,
    4,
    f64,
    crate::DVec2,
    crate::DVec3,
    crate::DVec4
);

#[cfg(feature = "f16")]
impl_swizzle_family!(
    crate::F16Vec2,
    2,
    half::f16,
    crate::F16Vec2,
    crate::F16Vec3,
    crate::F16Vec4
);
#[cfg(feature = "f16")]
impl_swizzle_family!(
    crate::F16Vec3,
    3,
    half::f16,
    crate::F16Vec2,
    crate::F16Vec3,
    crate::F16Vec4
);
#[cfg(feature = "f16")]
impl_swizzle_family!(
    crate::F16Vec4,
    4,
    half::f16,
    crate::F16Vec2,
    crate::F16Vec3,
    crate::F16Vec4
);

#[cfg(feature = "i8")]
impl_swizzle_family!(
    crate::I8Vec2,
    2,
    i8,
    crate::I8Vec2,
    crate::I8Vec3,
    crate::I8Vec4
);
#[cfg(feature = "i8")]
impl_swizzle_family!(
    crate::I8Vec3,
    3,
    i8,
    crate::I8Vec2,
    crate::I8Vec3,
    crate::I8Vec4
);
#[cfg(feature = "i8")]
impl_swizzle_family!(
    crate::I8Vec4,
    4,
    i8,
    crate::I8Vec2,
    crate::I8Vec3,
    crate::I8Vec4
);

#[cfg(feature = "u8")]
impl_swizzle_family!(
    crate::U8Vec2,
    2,
    u8,
    crate::U8Vec2,
    crate::U8Vec3,
    crate::U8Vec4
);
#[cfg(feature = "u8")]
impl_swizzle_family!(
    crate::U8Vec3,
    3,
    u8,
    crate::U8Vec2,
    crate::U8Vec3,
    crate::U8Vec4
);
#[cfg(feature = "u8")]
impl_swizzle_family!(
    crate::U8Vec4,
    4,
    u8,
    crate::U8Vec2,
    crate::U8Vec3,
    crate::U8Vec4
);

#[cfg(feature = "i16")]
impl_swizzle_family!(
    crate::I16Vec2,
    2,
    i16,
    crate::I16Vec2,
    crate::I16Vec3,
    crate::I16Vec4
);
#[cfg(feature = "i16")]
impl_swizzle_family!(
    crate::I16Vec3,
    3,
    i16,
    crate::I16Vec2,
    crate::I16Vec3,
    crate::I16Vec4
);
#[cfg(feature = "i16")]
impl_swizzle_family!(
    crate::I16Vec4,
    4,
    i16,
    crate::I16Vec2,
    crate::I16Vec3,
    crate::I16Vec4
);

#[cfg(feature = "u16")]
impl_swizzle_family!(
    crate::U16Vec2,
    2,
    u16,
    crate::U16Vec2,
    crate::U16Vec3,
    crate::U16Vec4
);
#[cfg(feature = "u16")]
impl_swizzle_family!(
    crate::U16Vec3,
    3,
    u16,
    crate::U16Vec2,
    crate::U16Vec3,
    crate::U16Vec4
);
#[cfg(feature = "u16")]
impl_swizzle_family!(
    crate::U16Vec4,
    4,
    u16,
    crate::U16Vec2,
    crate::U16Vec3,
    crate::U16Vec4
);

#[cfg(feature = "i32")]
impl_swizzle_family!(
    crate::IVec2,
    2,
    i32,
    crate::IVec2,
    crate::IVec3,
    crate::IVec4
);
#[cfg(feature = "i32")]
impl_swizzle_family!(
    crate::IVec3,
    3,
    i32,
    crate::IVec2,
    crate::IVec3,
    crate::IVec4
);
#[cfg(feature = "i32")]
impl_swizzle_family!(
    crate::IVec4,
    4,
    i32,
    crate::IVec2,
    crate::IVec3,
    crate::IVec4
);

#[cfg(feature = "u32")]
impl_swizzle_family!(
    crate::UVec2,
    2,
    u32,
    crate::UVec2,
    crate::UVec3,
    crate::UVec4
);
#[cfg(feature = "u32")]
impl_swizzle_family!(
    crate::UVec3,
    3,
    u32,
    crate::UVec2,
    crate::UVec3,
    crate::UVec4
);
#[cfg(feature = "u32")]
impl_swizzle_family!(
    crate::UVec4,
    4,
    u32,
    crate::UVec2,
    crate::UVec3,
    crate::UVec4
);

#[cfg(feature = "i64")]
impl_swizzle_family!(
    crate::I64Vec2,
    2,
    i64,
    crate::I64Vec2,
    crate::I64Vec3,
    crate::I64Vec4
);
#[cfg(feature = "i64")]
impl_swizzle_family!(
    crate::I64Vec3,
    3,
    i64,
    crate::I64Vec2,
    crate::I64Vec3,
    crate::I64Vec4
);
#[cfg(feature = "i64")]
impl_swizzle_family!(
    crate::I64Vec4,
    4,
    i64,
    crate::I64Vec2,
    crate::I64Vec3,
    crate::I64Vec4
);

#[cfg(feature = "u64")]
impl_swizzle_family!(
    crate::U64Vec2,
    2,
    u64,
    crate::U64Vec2,
    crate::U64Vec3,
    crate::U64Vec4
);
#[cfg(feature = "u64")]
impl_swizzle_family!(
    crate::U64Vec3,
    3,
    u64,
    crate::U64Vec2,
    crate::U64Vec3,
    crate::U64Vec4
);
#[cfg(feature = "u64")]
impl_swizzle_family!(
    crate::U64Vec4,
    4,
    u64,
    crate::U64Vec2,
    crate::U64Vec3,
    crate::U64Vec4
);

/// Reads a 1-to-4 component GLSL swizzle (`xyzw`, `rgba`, or `stpq`) from a vector expression.
///
/// Out-of-bounds component lanes fail to compile:
///
/// ```compile_fail
/// use rglm::*;
/// let v = Vec2::from([1.0, 2.0]);
/// let _ = rswz!(v.z);
/// ```
#[macro_export]
macro_rules! rswz {
    ($base:ident . $mask:ident) => {
        $crate::swizzle::read($base, $crate::swizzle::mask::$mask)
    };
    (($base:expr) . $mask:ident) => {
        $crate::swizzle::read($base, $crate::swizzle::mask::$mask)
    };
    ($base:ident $($rest:tt)+) => {
        $crate::rswz!(@peel [$base] $($rest)+)
    };
    (($base:expr) $($rest:tt)+) => {
        $crate::rswz!(@peel [($base)] $($rest)+)
    };
    (@peel [$($acc:tt)+] . $mask:ident) => {
        $crate::swizzle::read($($acc)+, $crate::swizzle::mask::$mask)
    };
    (@peel [$($acc:tt)+] . $field:ident $($rest:tt)+) => {
        $crate::rswz!(@peel [$($acc)+ . $field] $($rest)+)
    };
    (@peel [$($acc:tt)+] [$idx:expr] $($rest:tt)+) => {
        $crate::rswz!(@peel [$($acc)+ [$idx]] $($rest)+)
    };
}

/// Writes or mutates a 1-to-4 component GLSL swizzle (`xyzw`, `rgba`, or `stpq`) on a vector place.
///
/// Duplicate component lanes in write masks fail to compile:
///
/// ```compile_fail
/// use rglm::*;
/// let mut v = Vec4::from([1.0, 2.0, 3.0, 4.0]);
/// swz!(v.xx) = Vec2::from([5.0, 6.0]);
/// ```
#[macro_export]
macro_rules! swz {
    ($base:ident . $mask:ident) => {
        *($crate::swizzle::write(&mut $base, $crate::swizzle::mask::$mask))
    };
    (($base:expr) . $mask:ident) => {
        *($crate::swizzle::write(&mut ($base), $crate::swizzle::mask::$mask))
    };
    ($base:ident $($rest:tt)+) => {
        $crate::swz!(@peel [$base] $($rest)+)
    };
    (($base:expr) $($rest:tt)+) => {
        $crate::swz!(@peel [($base)] $($rest)+)
    };
    (@peel [$($acc:tt)+] . $mask:ident) => {
        *($crate::swizzle::write(&mut $($acc)+, $crate::swizzle::mask::$mask))
    };
    (@peel [$($acc:tt)+] . $field:ident $($rest:tt)+) => {
        $crate::swz!(@peel [$($acc)+ . $field] $($rest)+)
    };
    (@peel [$($acc:tt)+] [$idx:expr] $($rest:tt)+) => {
        $crate::swz!(@peel [$($acc)+ [$idx]] $($rest)+)
    };
}
