use crate::{BVec2, BVec3, BVec3A, BVec4, BVec4A};

mod private {
    pub trait Sealed {}
}

pub trait RelationalEq: private::Sealed + Copy {
    type Mask;

    fn equal_op(self, rhs: Self) -> Self::Mask;
    fn not_equal_op(self, rhs: Self) -> Self::Mask;
}

pub trait RelationalOrd: RelationalEq {
    fn less_than_op(self, rhs: Self) -> Self::Mask;
    fn less_than_equal_op(self, rhs: Self) -> Self::Mask;
    fn greater_than_op(self, rhs: Self) -> Self::Mask;
    fn greater_than_equal_op(self, rhs: Self) -> Self::Mask;
}

pub trait BoolVecOp: RelationalEq {
    fn any_op(self) -> bool;
    fn all_op(self) -> bool;
    fn not_op(self) -> Self;
}

macro_rules! impl_numeric_rel {
    ($($v:ty => $m:ty),+ $(,)?) => {
        $(
            impl private::Sealed for $v {}

            impl RelationalEq for $v {
                type Mask = $m;

                #[inline(always)]
                fn equal_op(self, rhs: Self) -> $m {
                    self.cmp_eq(rhs)
                }

                #[inline(always)]
                fn not_equal_op(self, rhs: Self) -> $m {
                    self.cmp_ne(rhs)
                }
            }

            impl RelationalOrd for $v {
                #[inline(always)]
                fn less_than_op(self, rhs: Self) -> $m {
                    self.cmp_lt(rhs)
                }

                #[inline(always)]
                fn less_than_equal_op(self, rhs: Self) -> $m {
                    self.cmp_le(rhs)
                }

                #[inline(always)]
                fn greater_than_op(self, rhs: Self) -> $m {
                    self.cmp_gt(rhs)
                }

                #[inline(always)]
                fn greater_than_equal_op(self, rhs: Self) -> $m {
                    self.cmp_ge(rhs)
                }
            }
        )+
    };
}

impl_numeric_rel!(
    crate::Vec2 => BVec2,
    crate::Vec3 => BVec3,
    crate::Vec3A => BVec3,
    crate::Vec4 => BVec4,
);

#[cfg(feature = "f64")]
impl_numeric_rel!(
    crate::DVec2 => BVec2,
    crate::DVec3 => BVec3,
    crate::DVec4 => BVec4,
);

#[cfg(feature = "f16")]
impl_numeric_rel!(
    crate::F16Vec2 => BVec2,
    crate::F16Vec3 => BVec3,
    crate::F16Vec4 => BVec4,
);

#[cfg(feature = "i8")]
impl_numeric_rel!(
    crate::I8Vec2 => BVec2,
    crate::I8Vec3 => BVec3,
    crate::I8Vec4 => BVec4,
);

#[cfg(feature = "u8")]
impl_numeric_rel!(
    crate::U8Vec2 => BVec2,
    crate::U8Vec3 => BVec3,
    crate::U8Vec4 => BVec4,
);

#[cfg(feature = "i16")]
impl_numeric_rel!(
    crate::I16Vec2 => BVec2,
    crate::I16Vec3 => BVec3,
    crate::I16Vec4 => BVec4,
);

#[cfg(feature = "u16")]
impl_numeric_rel!(
    crate::U16Vec2 => BVec2,
    crate::U16Vec3 => BVec3,
    crate::U16Vec4 => BVec4,
);

#[cfg(feature = "i32")]
impl_numeric_rel!(
    crate::IVec2 => BVec2,
    crate::IVec3 => BVec3,
    crate::IVec4 => BVec4,
);

#[cfg(feature = "u32")]
impl_numeric_rel!(
    crate::UVec2 => BVec2,
    crate::UVec3 => BVec3,
    crate::UVec4 => BVec4,
);

#[cfg(feature = "i64")]
impl_numeric_rel!(
    crate::I64Vec2 => BVec2,
    crate::I64Vec3 => BVec3,
    crate::I64Vec4 => BVec4,
);

#[cfg(feature = "u64")]
impl_numeric_rel!(
    crate::U64Vec2 => BVec2,
    crate::U64Vec3 => BVec3,
    crate::U64Vec4 => BVec4,
);

macro_rules! impl_quat_rel {
    ($($q:ty),+ $(,)?) => {
        $(
            impl private::Sealed for $q {}

            impl RelationalEq for $q {
                type Mask = BVec4;

                #[inline(always)]
                fn equal_op(self, rhs: Self) -> BVec4 {
                    let a = self.to_array();
                    let b = rhs.to_array();
                    BVec4::new(a[0] == b[0], a[1] == b[1], a[2] == b[2], a[3] == b[3])
                }

                #[inline(always)]
                fn not_equal_op(self, rhs: Self) -> BVec4 {
                    let a = self.to_array();
                    let b = rhs.to_array();
                    BVec4::new(a[0] != b[0], a[1] != b[1], a[2] != b[2], a[3] != b[3])
                }
            }
        )+
    };
}

impl_quat_rel!(crate::Quat);
#[cfg(feature = "f64")]
impl_quat_rel!(crate::DQuat);
#[cfg(feature = "f16")]
impl_quat_rel!(crate::F16Quat);

macro_rules! impl_bool_vec_rel {
    ($($bv:ty => $dim:literal),+ $(,)?) => {
        $(
            impl private::Sealed for $bv {}

            impl RelationalEq for $bv {
                type Mask = $bv;

                #[inline(always)]
                fn equal_op(self, rhs: Self) -> $bv {
                    !(self ^ rhs)
                }

                #[inline(always)]
                fn not_equal_op(self, rhs: Self) -> $bv {
                    self ^ rhs
                }
            }

            impl BoolVecOp for $bv {
                #[inline(always)]
                fn any_op(self) -> bool {
                    self.any()
                }

                #[inline(always)]
                fn all_op(self) -> bool {
                    self.all()
                }

                #[inline(always)]
                fn not_op(self) -> Self {
                    !self
                }
            }
        )+
    };
}

impl_bool_vec_rel!(
    BVec2 => 2,
    BVec3 => 3,
    BVec3A => 3,
    BVec4 => 4,
    BVec4A => 4,
);

/// Returns the component-wise `<` comparison of `x` and `y` (`lessThan`).
#[inline(always)]
#[must_use]
pub fn less_than<V: RelationalOrd>(x: V, y: V) -> V::Mask {
    x.less_than_op(y)
}

/// Returns the component-wise `<=` comparison of `x` and `y` (`lessThanEqual`).
#[inline(always)]
#[must_use]
pub fn less_than_equal<V: RelationalOrd>(x: V, y: V) -> V::Mask {
    x.less_than_equal_op(y)
}

/// Returns the component-wise `>` comparison of `x` and `y` (`greaterThan`).
#[inline(always)]
#[must_use]
pub fn greater_than<V: RelationalOrd>(x: V, y: V) -> V::Mask {
    x.greater_than_op(y)
}

/// Returns the component-wise `>=` comparison of `x` and `y` (`greaterThanEqual`).
#[inline(always)]
#[must_use]
pub fn greater_than_equal<V: RelationalOrd>(x: V, y: V) -> V::Mask {
    x.greater_than_equal_op(y)
}

/// Returns the component-wise `==` comparison of `x` and `y` (`equal`).
#[inline(always)]
#[must_use]
pub fn equal<V: RelationalEq>(x: V, y: V) -> V::Mask {
    x.equal_op(y)
}

/// Returns the component-wise `!=` comparison of `x` and `y` (`notEqual`).
#[inline(always)]
#[must_use]
pub fn not_equal<V: RelationalEq>(x: V, y: V) -> V::Mask {
    x.not_equal_op(y)
}

/// Returns `true` if any component of boolean vector `x` is `true` (`any`).
#[inline(always)]
#[must_use]
pub fn any<B: BoolVecOp>(x: B) -> bool {
    x.any_op()
}

/// Returns `true` if all components of boolean vector `x` are `true` (`all`).
#[inline(always)]
#[must_use]
pub fn all<B: BoolVecOp>(x: B) -> bool {
    x.all_op()
}

/// Returns the component-wise logical negation of boolean vector `x` (`not`).
#[inline(always)]
#[must_use]
pub fn not<B: BoolVecOp>(x: B) -> B {
    x.not_op()
}
