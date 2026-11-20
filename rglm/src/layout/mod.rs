#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(1))]
pub struct Align1(pub [u8; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(2))]
pub struct Align2(pub [u8; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(4))]
pub struct Align4(pub [u8; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(8))]
pub struct Align8(pub [u8; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(16))]
pub struct Align16(pub [u8; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, align(32))]
pub struct Align32(pub [u8; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
#[repr(C, align(16))]
pub struct Std140ArrayAlign<A>(pub [A; 0]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
#[repr(C, align(16))]
pub struct Std140Elem<R, A> {
    pub _align: A,
    pub value: R,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
#[repr(C)]
pub struct Std430Elem<R, A> {
    pub _align: A,
    pub value: R,
}

/// Rounds `offset` up to the next multiple of power-of-two `align`.
#[inline(always)]
#[must_use]
pub const fn align_offset(offset: usize, align: usize) -> usize {
    if align <= 1 {
        offset
    } else {
        (offset + align - 1) & !(align - 1)
    }
}

/// Returns the larger of two alignment or size values in `const` contexts.
#[inline(always)]
#[must_use]
pub const fn max_usize(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

pub trait LayoutZero {
    const ZERO_REPR: Self;
}

impl LayoutZero for Align1 {
    const ZERO_REPR: Self = Self([]);
}
impl LayoutZero for Align2 {
    const ZERO_REPR: Self = Self([]);
}
impl LayoutZero for Align4 {
    const ZERO_REPR: Self = Self([]);
}
impl LayoutZero for Align8 {
    const ZERO_REPR: Self = Self([]);
}
impl LayoutZero for Align16 {
    const ZERO_REPR: Self = Self([]);
}
impl LayoutZero for Align32 {
    const ZERO_REPR: Self = Self([]);
}
impl<A> LayoutZero for Std140ArrayAlign<A> {
    const ZERO_REPR: Self = Self([]);
}
impl<R: LayoutZero, A: LayoutZero> LayoutZero for Std140Elem<R, A> {
    const ZERO_REPR: Self = Self {
        _align: A::ZERO_REPR,
        value: R::ZERO_REPR,
    };
}
impl<R: LayoutZero, A: LayoutZero> LayoutZero for Std430Elem<R, A> {
    const ZERO_REPR: Self = Self {
        _align: A::ZERO_REPR,
        value: R::ZERO_REPR,
    };
}
impl<T: LayoutZero + Copy, const N: usize> LayoutZero for [T; N] {
    const ZERO_REPR: Self = [T::ZERO_REPR; N];
}

pub trait Std140: Sized {
    const ALIGN: usize;
    const SIZE: usize;

    type Align: Copy + core::fmt::Debug + LayoutZero;
    type Repr: Copy + core::fmt::Debug + LayoutZero;

    fn to_std140(&self) -> Self::Repr;
    fn write_std140(&self, dst: &mut [u8]);
}

pub trait Std430: Sized {
    const ALIGN: usize;
    const SIZE: usize;

    type Align: Copy + core::fmt::Debug + LayoutZero;
    type Repr: Copy + core::fmt::Debug + LayoutZero;

    fn to_std430(&self) -> Self::Repr;
    fn write_std430(&self, dst: &mut [u8]);
}

pub trait Scalar: Sized {
    const ALIGN: usize;
    const SIZE: usize;

    type Align: Copy + core::fmt::Debug + LayoutZero;
    type Repr: Copy + core::fmt::Debug + LayoutZero;

    fn to_scalar(&self) -> Self::Repr;
    fn write_scalar(&self, dst: &mut [u8]);
}

macro_rules! impl_layout_scalar {
    ($t:ty, $zero:expr, $sz:literal, $align_ty:ty) => {
        impl LayoutZero for $t {
            const ZERO_REPR: Self = $zero;
        }

        impl Std140 for $t {
            const ALIGN: usize = $sz;
            const SIZE: usize = $sz;
            type Align = $align_ty;
            type Repr = $t;

            #[inline(always)]
            fn to_std140(&self) -> Self::Repr {
                *self
            }

            #[inline(always)]
            fn write_std140(&self, dst: &mut [u8]) {
                dst[..$sz].copy_from_slice(&self.to_ne_bytes());
            }
        }

        impl Std430 for $t {
            const ALIGN: usize = $sz;
            const SIZE: usize = $sz;
            type Align = $align_ty;
            type Repr = $t;

            #[inline(always)]
            fn to_std430(&self) -> Self::Repr {
                *self
            }

            #[inline(always)]
            fn write_std430(&self, dst: &mut [u8]) {
                dst[..$sz].copy_from_slice(&self.to_ne_bytes());
            }
        }

        impl Scalar for $t {
            const ALIGN: usize = $sz;
            const SIZE: usize = $sz;
            type Align = $align_ty;
            type Repr = $t;

            #[inline(always)]
            fn to_scalar(&self) -> Self::Repr {
                *self
            }

            #[inline(always)]
            fn write_scalar(&self, dst: &mut [u8]) {
                dst[..$sz].copy_from_slice(&self.to_ne_bytes());
            }
        }
    };
}

impl_layout_scalar!(f32, 0.0, 4, Align4);
#[cfg(feature = "f64")]
impl_layout_scalar!(f64, 0.0, 8, Align8);
impl_layout_scalar!(i8, 0, 1, Align1);
impl_layout_scalar!(u8, 0, 1, Align1);
impl_layout_scalar!(i16, 0, 2, Align2);
impl_layout_scalar!(u16, 0, 2, Align2);
impl_layout_scalar!(i32, 0, 4, Align4);
impl_layout_scalar!(u32, 0, 4, Align4);
impl_layout_scalar!(i64, 0, 8, Align8);
impl_layout_scalar!(u64, 0, 8, Align8);

#[cfg(feature = "f16")]
impl LayoutZero for half::f16 {
    const ZERO_REPR: Self = half::f16::ZERO;
}

#[cfg(feature = "f16")]
impl Std140 for half::f16 {
    const ALIGN: usize = 2;
    const SIZE: usize = 2;
    type Align = Align2;
    type Repr = u16;

    #[inline(always)]
    fn to_std140(&self) -> u16 {
        self.to_bits()
    }

    #[inline(always)]
    fn write_std140(&self, dst: &mut [u8]) {
        dst[..2].copy_from_slice(&self.to_bits().to_ne_bytes());
    }
}

#[cfg(feature = "f16")]
impl Std430 for half::f16 {
    const ALIGN: usize = 2;
    const SIZE: usize = 2;
    type Align = Align2;
    type Repr = u16;

    #[inline(always)]
    fn to_std430(&self) -> u16 {
        self.to_bits()
    }

    #[inline(always)]
    fn write_std430(&self, dst: &mut [u8]) {
        dst[..2].copy_from_slice(&self.to_bits().to_ne_bytes());
    }
}

#[cfg(feature = "f16")]
impl Scalar for half::f16 {
    const ALIGN: usize = 2;
    const SIZE: usize = 2;
    type Align = Align2;
    type Repr = u16;

    #[inline(always)]
    fn to_scalar(&self) -> u16 {
        self.to_bits()
    }

    #[inline(always)]
    fn write_scalar(&self, dst: &mut [u8]) {
        dst[..2].copy_from_slice(&self.to_bits().to_ne_bytes());
    }
}

impl Std140 for bool {
    const ALIGN: usize = 4;
    const SIZE: usize = 4;
    type Align = Align4;
    type Repr = u32;

    #[inline(always)]
    fn to_std140(&self) -> u32 {
        u32::from(*self)
    }

    #[inline(always)]
    fn write_std140(&self, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&u32::from(*self).to_ne_bytes());
    }
}

impl Std430 for bool {
    const ALIGN: usize = 4;
    const SIZE: usize = 4;
    type Align = Align4;
    type Repr = u32;

    #[inline(always)]
    fn to_std430(&self) -> u32 {
        u32::from(*self)
    }

    #[inline(always)]
    fn write_std430(&self, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&u32::from(*self).to_ne_bytes());
    }
}

impl Scalar for bool {
    const ALIGN: usize = 4;
    const SIZE: usize = 4;
    type Align = Align4;
    type Repr = u32;

    #[inline(always)]
    fn to_scalar(&self) -> u32 {
        u32::from(*self)
    }

    #[inline(always)]
    fn write_scalar(&self, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&u32::from(*self).to_ne_bytes());
    }
}

macro_rules! impl_layout_vec {
    (
        $vec_t:ty,
        $elem_repr:ty,
        $dim:literal,
        $elem_sz:literal,
        $vec_align_num:literal,
        $vec_align_ty:ty,
        $scalar_align_ty:ty,
        |$v:ident| $to_arr:expr
    ) => {
        impl Std140 for $vec_t {
            const ALIGN: usize = $vec_align_num;
            const SIZE: usize = $dim * $elem_sz;
            type Align = $vec_align_ty;
            type Repr = [$elem_repr; $dim];

            #[inline(always)]
            fn to_std140(&self) -> Self::Repr {
                let $v = *self;
                $to_arr
            }

            #[inline(always)]
            fn write_std140(&self, dst: &mut [u8]) {
                let arr = self.to_std140();
                let mut i = 0;
                while i < $dim {
                    let start = i * $elem_sz;
                    dst[start..start + $elem_sz].copy_from_slice(&arr[i].to_ne_bytes());
                    i += 1;
                }
            }
        }

        impl Std430 for $vec_t {
            const ALIGN: usize = $vec_align_num;
            const SIZE: usize = $dim * $elem_sz;
            type Align = $vec_align_ty;
            type Repr = [$elem_repr; $dim];

            #[inline(always)]
            fn to_std430(&self) -> Self::Repr {
                let $v = *self;
                $to_arr
            }

            #[inline(always)]
            fn write_std430(&self, dst: &mut [u8]) {
                self.write_std140(dst);
            }
        }

        impl Scalar for $vec_t {
            const ALIGN: usize = $elem_sz;
            const SIZE: usize = $dim * $elem_sz;
            type Align = $scalar_align_ty;
            type Repr = [$elem_repr; $dim];

            #[inline(always)]
            fn to_scalar(&self) -> Self::Repr {
                let $v = *self;
                $to_arr
            }

            #[inline(always)]
            fn write_scalar(&self, dst: &mut [u8]) {
                self.write_std140(dst);
            }
        }
    };
}

impl_layout_vec!(crate::Vec2, f32, 2, 4, 8, Align8, Align4, |v| v.to_array());
impl_layout_vec!(crate::Vec3, f32, 3, 4, 16, Align16, Align4, |v| v
    .to_array());
impl_layout_vec!(crate::Vec3A, f32, 3, 4, 16, Align16, Align4, |v| v
    .to_array());
impl_layout_vec!(crate::Vec4, f32, 4, 4, 16, Align16, Align4, |v| v
    .to_array());
impl_layout_vec!(crate::Quat, f32, 4, 4, 16, Align16, Align4, |v| v
    .to_array());

impl_layout_vec!(crate::BVec2, u32, 2, 4, 8, Align8, Align4, |v| v
    .to_array()
    .map(u32::from));
impl_layout_vec!(crate::BVec3, u32, 3, 4, 16, Align16, Align4, |v| v
    .to_array()
    .map(u32::from));
impl_layout_vec!(crate::BVec3A, u32, 3, 4, 16, Align16, Align4, |v| v
    .to_array()
    .map(u32::from));
impl_layout_vec!(crate::BVec4, u32, 4, 4, 16, Align16, Align4, |v| v
    .to_array()
    .map(u32::from));
impl_layout_vec!(crate::BVec4A, u32, 4, 4, 16, Align16, Align4, |v| v
    .to_array()
    .map(u32::from));

#[cfg(feature = "f64")]
impl_layout_vec!(crate::DVec2, f64, 2, 8, 16, Align16, Align8, |v| v
    .to_array());
#[cfg(feature = "f64")]
impl_layout_vec!(crate::DVec3, f64, 3, 8, 32, Align32, Align8, |v| v
    .to_array());
#[cfg(feature = "f64")]
impl_layout_vec!(crate::DVec4, f64, 4, 8, 32, Align32, Align8, |v| v
    .to_array());
#[cfg(feature = "f64")]
impl_layout_vec!(crate::DQuat, f64, 4, 8, 32, Align32, Align8, |v| v
    .to_array());

#[cfg(feature = "f16")]
impl_layout_vec!(crate::F16Vec2, u16, 2, 2, 4, Align4, Align2, |v| v
    .to_array()
    .map(half::f16::to_bits));
#[cfg(feature = "f16")]
impl_layout_vec!(crate::F16Vec3, u16, 3, 2, 8, Align8, Align2, |v| v
    .to_array()
    .map(half::f16::to_bits));
#[cfg(feature = "f16")]
impl_layout_vec!(crate::F16Vec4, u16, 4, 2, 8, Align8, Align2, |v| v
    .to_array()
    .map(half::f16::to_bits));
#[cfg(feature = "f16")]
impl_layout_vec!(crate::F16Quat, u16, 4, 2, 8, Align8, Align2, |v| v
    .to_array()
    .map(half::f16::to_bits));

#[cfg(feature = "i8")]
impl_layout_vec!(crate::I8Vec2, i8, 2, 1, 2, Align2, Align1, |v| v.to_array());
#[cfg(feature = "i8")]
impl_layout_vec!(crate::I8Vec3, i8, 3, 1, 4, Align4, Align1, |v| v.to_array());
#[cfg(feature = "i8")]
impl_layout_vec!(crate::I8Vec4, i8, 4, 1, 4, Align4, Align1, |v| v.to_array());

#[cfg(feature = "u8")]
impl_layout_vec!(crate::U8Vec2, u8, 2, 1, 2, Align2, Align1, |v| v.to_array());
#[cfg(feature = "u8")]
impl_layout_vec!(crate::U8Vec3, u8, 3, 1, 4, Align4, Align1, |v| v.to_array());
#[cfg(feature = "u8")]
impl_layout_vec!(crate::U8Vec4, u8, 4, 1, 4, Align4, Align1, |v| v.to_array());

#[cfg(feature = "i16")]
impl_layout_vec!(crate::I16Vec2, i16, 2, 2, 4, Align4, Align2, |v| v
    .to_array());
#[cfg(feature = "i16")]
impl_layout_vec!(crate::I16Vec3, i16, 3, 2, 8, Align8, Align2, |v| v
    .to_array());
#[cfg(feature = "i16")]
impl_layout_vec!(crate::I16Vec4, i16, 4, 2, 8, Align8, Align2, |v| v
    .to_array());

#[cfg(feature = "u16")]
impl_layout_vec!(crate::U16Vec2, u16, 2, 2, 4, Align4, Align2, |v| v
    .to_array());
#[cfg(feature = "u16")]
impl_layout_vec!(crate::U16Vec3, u16, 3, 2, 8, Align8, Align2, |v| v
    .to_array());
#[cfg(feature = "u16")]
impl_layout_vec!(crate::U16Vec4, u16, 4, 2, 8, Align8, Align2, |v| v
    .to_array());

#[cfg(feature = "i32")]
impl_layout_vec!(crate::IVec2, i32, 2, 4, 8, Align8, Align4, |v| v.to_array());
#[cfg(feature = "i32")]
impl_layout_vec!(crate::IVec3, i32, 3, 4, 16, Align16, Align4, |v| v
    .to_array());
#[cfg(feature = "i32")]
impl_layout_vec!(crate::IVec4, i32, 4, 4, 16, Align16, Align4, |v| v
    .to_array());

#[cfg(feature = "u32")]
impl_layout_vec!(crate::UVec2, u32, 2, 4, 8, Align8, Align4, |v| v.to_array());
#[cfg(feature = "u32")]
impl_layout_vec!(crate::UVec3, u32, 3, 4, 16, Align16, Align4, |v| v
    .to_array());
#[cfg(feature = "u32")]
impl_layout_vec!(crate::UVec4, u32, 4, 4, 16, Align16, Align4, |v| v
    .to_array());

#[cfg(feature = "i64")]
impl_layout_vec!(crate::I64Vec2, i64, 2, 8, 16, Align16, Align8, |v| v
    .to_array());
#[cfg(feature = "i64")]
impl_layout_vec!(crate::I64Vec3, i64, 3, 8, 32, Align32, Align8, |v| v
    .to_array());
#[cfg(feature = "i64")]
impl_layout_vec!(crate::I64Vec4, i64, 4, 8, 32, Align32, Align8, |v| v
    .to_array());

#[cfg(feature = "u64")]
impl_layout_vec!(crate::U64Vec2, u64, 2, 8, 16, Align16, Align8, |v| v
    .to_array());
#[cfg(feature = "u64")]
impl_layout_vec!(crate::U64Vec3, u64, 3, 8, 32, Align32, Align8, |v| v
    .to_array());
#[cfg(feature = "u64")]
impl_layout_vec!(crate::U64Vec4, u64, 4, 8, 32, Align32, Align8, |v| v
    .to_array());

macro_rules! impl_layout_mat {
    ($mat_t:ty, $col_t:ty, $cols:literal; $($axis:ident : $idx:literal),+) => {
        impl Std140 for $mat_t {
            const ALIGN: usize = max_usize(<$col_t as Std140>::ALIGN, 16);
            const SIZE: usize = $cols * align_offset(<$col_t as Std140>::SIZE, <Self as Std140>::ALIGN);
            type Align = Std140ArrayAlign<<$col_t as Std140>::Align>;
            type Repr = [Std140Elem<<$col_t as Std140>::Repr, <$col_t as Std140>::Align>; $cols];

            #[inline(always)]
            fn to_std140(&self) -> Self::Repr {
                [
                    $(
                        Std140Elem {
                            _align: <<$col_t as Std140>::Align as LayoutZero>::ZERO_REPR,
                            value: self.$axis.to_std140(),
                        }
                    ),+
                ]
            }

            #[inline]
            fn write_std140(&self, dst: &mut [u8]) {
                let stride = align_offset(<$col_t as Std140>::SIZE, <Self as Std140>::ALIGN);
                dst[..<Self as Std140>::SIZE].fill(0);
                $(
                    self.$axis.write_std140(&mut dst[$idx * stride .. $idx * stride + <$col_t as Std140>::SIZE]);
                )+
            }
        }

        impl Std430 for $mat_t {
            const ALIGN: usize = <$col_t as Std430>::ALIGN;
            const SIZE: usize = $cols * align_offset(<$col_t as Std430>::SIZE, <Self as Std430>::ALIGN);
            type Align = <$col_t as Std430>::Align;
            type Repr = [Std430Elem<<$col_t as Std430>::Repr, <$col_t as Std430>::Align>; $cols];

            #[inline(always)]
            fn to_std430(&self) -> Self::Repr {
                [
                    $(
                        Std430Elem {
                            _align: <<$col_t as Std430>::Align as LayoutZero>::ZERO_REPR,
                            value: self.$axis.to_std430(),
                        }
                    ),+
                ]
            }

            #[inline]
            fn write_std430(&self, dst: &mut [u8]) {
                let stride = align_offset(<$col_t as Std430>::SIZE, <Self as Std430>::ALIGN);
                dst[..<Self as Std430>::SIZE].fill(0);
                $(
                    self.$axis.write_std430(&mut dst[$idx * stride .. $idx * stride + <$col_t as Std430>::SIZE]);
                )+
            }
        }

        impl Scalar for $mat_t {
            const ALIGN: usize = <$col_t as Scalar>::ALIGN;
            const SIZE: usize = $cols * <$col_t as Scalar>::SIZE;
            type Align = <$col_t as Scalar>::Align;
            type Repr = [<$col_t as Scalar>::Repr; $cols];

            #[inline(always)]
            fn to_scalar(&self) -> Self::Repr {
                [$(self.$axis.to_scalar()),+]
            }

            #[inline]
            fn write_scalar(&self, dst: &mut [u8]) {
                let stride = <$col_t as Scalar>::SIZE;
                $(
                    self.$axis.write_scalar(&mut dst[$idx * stride .. ($idx + 1) * stride]);
                )+
            }
        }
    };
}

impl_layout_mat!(crate::Mat2, crate::Vec2, 2; x_axis: 0, y_axis: 1);
impl_layout_mat!(crate::Mat2x3, crate::Vec3, 2; x_axis: 0, y_axis: 1);
impl_layout_mat!(crate::Mat2x3A, crate::Vec3A, 2; x_axis: 0, y_axis: 1);
impl_layout_mat!(crate::Mat2x4, crate::Vec4, 2; x_axis: 0, y_axis: 1);
impl_layout_mat!(crate::Mat3x2, crate::Vec2, 3; x_axis: 0, y_axis: 1, z_axis: 2);
impl_layout_mat!(crate::Mat3, crate::Vec3, 3; x_axis: 0, y_axis: 1, z_axis: 2);
impl_layout_mat!(crate::Mat3A, crate::Vec3A, 3; x_axis: 0, y_axis: 1, z_axis: 2);
impl_layout_mat!(crate::Mat3x4, crate::Vec4, 3; x_axis: 0, y_axis: 1, z_axis: 2);
impl_layout_mat!(crate::Mat4x2, crate::Vec2, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
impl_layout_mat!(crate::Mat4x3, crate::Vec3, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
impl_layout_mat!(crate::Mat4x3A, crate::Vec3A, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
impl_layout_mat!(crate::Mat4, crate::Vec4, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);

#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat2, crate::DVec2, 2; x_axis: 0, y_axis: 1);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat2x3, crate::DVec3, 2; x_axis: 0, y_axis: 1);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat2x4, crate::DVec4, 2; x_axis: 0, y_axis: 1);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat3x2, crate::DVec2, 3; x_axis: 0, y_axis: 1, z_axis: 2);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat3, crate::DVec3, 3; x_axis: 0, y_axis: 1, z_axis: 2);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat3x4, crate::DVec4, 3; x_axis: 0, y_axis: 1, z_axis: 2);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat4x2, crate::DVec2, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat4x3, crate::DVec3, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
#[cfg(feature = "f64")]
impl_layout_mat!(crate::DMat4, crate::DVec4, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);

#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat2, crate::F16Vec2, 2; x_axis: 0, y_axis: 1);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat2x3, crate::F16Vec3, 2; x_axis: 0, y_axis: 1);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat2x4, crate::F16Vec4, 2; x_axis: 0, y_axis: 1);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat3x2, crate::F16Vec2, 3; x_axis: 0, y_axis: 1, z_axis: 2);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat3, crate::F16Vec3, 3; x_axis: 0, y_axis: 1, z_axis: 2);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat3x4, crate::F16Vec4, 3; x_axis: 0, y_axis: 1, z_axis: 2);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat4x2, crate::F16Vec2, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat4x3, crate::F16Vec3, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);
#[cfg(feature = "f16")]
impl_layout_mat!(crate::F16Mat4, crate::F16Vec4, 4; x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3);

impl<T: Std140, const N: usize> Std140 for [T; N] {
    const ALIGN: usize = max_usize(T::ALIGN, 16);
    const SIZE: usize = N * align_offset(T::SIZE, <Self as Std140>::ALIGN);
    type Align = Std140ArrayAlign<T::Align>;
    type Repr = [Std140Elem<T::Repr, T::Align>; N];

    #[inline]
    fn to_std140(&self) -> Self::Repr {
        let mut out = [Std140Elem::<T::Repr, T::Align>::ZERO_REPR; N];
        let mut i = 0;
        while i < N {
            out[i] = Std140Elem {
                _align: T::Align::ZERO_REPR,
                value: self[i].to_std140(),
            };
            i += 1;
        }
        out
    }

    #[inline]
    fn write_std140(&self, dst: &mut [u8]) {
        let stride = align_offset(T::SIZE, <Self as Std140>::ALIGN);
        dst[..<Self as Std140>::SIZE].fill(0);
        let mut i = 0;
        while i < N {
            let start = i * stride;
            self[i].write_std140(&mut dst[start..start + T::SIZE]);
            i += 1;
        }
    }
}

impl<T: Std430, const N: usize> Std430 for [T; N] {
    const ALIGN: usize = T::ALIGN;
    const SIZE: usize = N * align_offset(T::SIZE, T::ALIGN);
    type Align = T::Align;
    type Repr = [Std430Elem<T::Repr, T::Align>; N];

    #[inline]
    fn to_std430(&self) -> Self::Repr {
        let mut out = [Std430Elem::<T::Repr, T::Align>::ZERO_REPR; N];
        let mut i = 0;
        while i < N {
            out[i] = Std430Elem {
                _align: T::Align::ZERO_REPR,
                value: self[i].to_std430(),
            };
            i += 1;
        }
        out
    }

    #[inline]
    fn write_std430(&self, dst: &mut [u8]) {
        let stride = align_offset(T::SIZE, T::ALIGN);
        dst[..<Self as Std430>::SIZE].fill(0);
        let mut i = 0;
        while i < N {
            let start = i * stride;
            self[i].write_std430(&mut dst[start..start + T::SIZE]);
            i += 1;
        }
    }
}

impl<T: Scalar, const N: usize> Scalar for [T; N] {
    const ALIGN: usize = T::ALIGN;
    const SIZE: usize = N * align_offset(T::SIZE, T::ALIGN);
    type Align = T::Align;
    type Repr = [Std430Elem<T::Repr, T::Align>; N];

    #[inline]
    fn to_scalar(&self) -> Self::Repr {
        let mut out = [Std430Elem::<T::Repr, T::Align>::ZERO_REPR; N];
        let mut i = 0;
        while i < N {
            out[i] = Std430Elem {
                _align: T::Align::ZERO_REPR,
                value: self[i].to_scalar(),
            };
            i += 1;
        }
        out
    }

    #[inline]
    fn write_scalar(&self, dst: &mut [u8]) {
        let stride = align_offset(T::SIZE, T::ALIGN);
        dst[..<Self as Scalar>::SIZE].fill(0);
        let mut i = 0;
        while i < N {
            let start = i * stride;
            self[i].write_scalar(&mut dst[start..start + T::SIZE]);
            i += 1;
        }
    }
}

/// Converts a value into its Vulkan `std140` GPU representation.
#[inline]
#[must_use]
pub fn to_std140<T: Std140>(val: &T) -> T::Repr {
    val.to_std140()
}

/// Converts a value into its Vulkan `std430` GPU representation.
#[inline]
#[must_use]
pub fn to_std430<T: Std430>(val: &T) -> T::Repr {
    val.to_std430()
}

/// Converts a value into its Vulkan `VK_EXT_scalar_block_layout` GPU representation.
#[inline]
#[must_use]
pub fn to_scalar<T: Scalar>(val: &T) -> T::Repr {
    val.to_scalar()
}

/// Writes the zero-padded Vulkan `std140` representation of `val` into `dst[..T::SIZE]`.
#[inline]
pub fn write_std140<T: Std140>(val: &T, dst: &mut [u8]) {
    val.write_std140(dst);
}

/// Writes the zero-padded Vulkan `std430` representation of `val` into `dst[..T::SIZE]`.
#[inline]
pub fn write_std430<T: Std430>(val: &T, dst: &mut [u8]) {
    val.write_std430(dst);
}

/// Writes the zero-padded Vulkan `VK_EXT_scalar_block_layout` representation of `val` into `dst[..T::SIZE]`.
#[inline]
pub fn write_scalar<T: Scalar>(val: &T, dst: &mut [u8]) {
    val.write_scalar(dst);
}
