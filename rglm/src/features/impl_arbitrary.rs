use arbitrary::{Arbitrary, Result, Unstructured};

macro_rules! impl_arbitrary_via_array {
    ($($ty:ty => [$elem:ty; $n:expr]),+ $(,)?) => {
        $(
            impl<'a> Arbitrary<'a> for $ty {
                #[inline]
                fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                    let arr = <[$elem; $n]>::arbitrary(u)?;
                    Ok(Self::from(arr))
                }
            }
        )+
    };
}

impl_arbitrary_via_array!(
    crate::Vec2 => [f32; 2],
    crate::Vec3 => [f32; 3],
    crate::Vec3A => [f32; 3],
    crate::Vec4 => [f32; 4],
    crate::BVec2 => [bool; 2],
    crate::BVec3 => [bool; 3],
    crate::BVec3A => [bool; 3],
    crate::BVec4 => [bool; 4],
    crate::BVec4A => [bool; 4],
    crate::Quat => [f32; 4],
    crate::Mat2 => [f32; 4],
    crate::Mat2x3 => [f32; 6],
    crate::Mat2x3A => [f32; 6],
    crate::Mat2x4 => [f32; 8],
    crate::Mat3x2 => [f32; 6],
    crate::Mat3 => [f32; 9],
    crate::Mat3A => [f32; 9],
    crate::Mat3x4 => [f32; 12],
    crate::Mat4x2 => [f32; 8],
    crate::Mat4x3 => [f32; 12],
    crate::Mat4x3A => [f32; 12],
    crate::Mat4 => [f32; 16],
);

#[cfg(feature = "f64")]
impl_arbitrary_via_array!(
    crate::DVec2 => [f64; 2],
    crate::DVec3 => [f64; 3],
    crate::DVec4 => [f64; 4],
    crate::DQuat => [f64; 4],
    crate::DMat2 => [f64; 4],
    crate::DMat2x3 => [f64; 6],
    crate::DMat2x4 => [f64; 8],
    crate::DMat3x2 => [f64; 6],
    crate::DMat3 => [f64; 9],
    crate::DMat3x4 => [f64; 12],
    crate::DMat4x2 => [f64; 8],
    crate::DMat4x3 => [f64; 12],
    crate::DMat4 => [f64; 16],
);

#[cfg(feature = "f16")]
macro_rules! impl_arbitrary_f16 {
    ($($ty:ty => $n:expr),+ $(,)?) => {
        $(
            impl<'a> Arbitrary<'a> for $ty {
                #[inline]
                fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                    let bits = <[u16; $n]>::arbitrary(u)?;
                    Ok(Self::from(bits.map(half::f16::from_bits)))
                }
            }
        )+
    };
}

#[cfg(feature = "f16")]
impl_arbitrary_f16!(
    crate::F16Vec2 => 2,
    crate::F16Vec3 => 3,
    crate::F16Vec4 => 4,
    crate::F16Quat => 4,
    crate::F16Mat2 => 4,
    crate::F16Mat2x3 => 6,
    crate::F16Mat2x4 => 8,
    crate::F16Mat3x2 => 6,
    crate::F16Mat3 => 9,
    crate::F16Mat3x4 => 12,
    crate::F16Mat4x2 => 8,
    crate::F16Mat4x3 => 12,
    crate::F16Mat4 => 16,
);

#[cfg(feature = "i8")]
impl_arbitrary_via_array!(
    crate::I8Vec2 => [i8; 2],
    crate::I8Vec3 => [i8; 3],
    crate::I8Vec4 => [i8; 4],
);

#[cfg(feature = "u8")]
impl_arbitrary_via_array!(
    crate::U8Vec2 => [u8; 2],
    crate::U8Vec3 => [u8; 3],
    crate::U8Vec4 => [u8; 4],
);

#[cfg(feature = "i16")]
impl_arbitrary_via_array!(
    crate::I16Vec2 => [i16; 2],
    crate::I16Vec3 => [i16; 3],
    crate::I16Vec4 => [i16; 4],
);

#[cfg(feature = "u16")]
impl_arbitrary_via_array!(
    crate::U16Vec2 => [u16; 2],
    crate::U16Vec3 => [u16; 3],
    crate::U16Vec4 => [u16; 4],
);

#[cfg(feature = "i32")]
impl_arbitrary_via_array!(
    crate::IVec2 => [i32; 2],
    crate::IVec3 => [i32; 3],
    crate::IVec4 => [i32; 4],
);

#[cfg(feature = "u32")]
impl_arbitrary_via_array!(
    crate::UVec2 => [u32; 2],
    crate::UVec3 => [u32; 3],
    crate::UVec4 => [u32; 4],
);

#[cfg(feature = "i64")]
impl_arbitrary_via_array!(
    crate::I64Vec2 => [i64; 2],
    crate::I64Vec3 => [i64; 3],
    crate::I64Vec4 => [i64; 4],
);

#[cfg(feature = "u64")]
impl_arbitrary_via_array!(
    crate::U64Vec2 => [u64; 2],
    crate::U64Vec3 => [u64; 3],
    crate::U64Vec4 => [u64; 4],
);
