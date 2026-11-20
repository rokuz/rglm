#![expect(
    dead_code,
    reason = "internal vector kernels are wired to public free functions in M2-M4"
)]

pub(crate) mod bool {
    pub mod bvec2;
    pub mod bvec3;
    pub mod bvec3a;
    pub mod bvec4;
    pub mod bvec4a;
}

pub(crate) mod f32 {
    pub mod vec2;
    pub mod vec3;
    pub mod vec3a;
    pub mod vec4;
}

#[cfg(feature = "f64")]
pub(crate) mod f64 {
    pub mod dvec2;
    pub mod dvec3;
    pub mod dvec4;
}

#[cfg(feature = "f16")]
pub(crate) mod f16 {
    pub mod f16vec2;
    pub mod f16vec3;
    pub mod f16vec4;
}

pub(crate) mod int {
    #[cfg(feature = "i8")]
    pub mod i8vec2;
    #[cfg(feature = "i8")]
    pub mod i8vec3;
    #[cfg(feature = "i8")]
    pub mod i8vec4;

    #[cfg(feature = "u8")]
    pub mod u8vec2;
    #[cfg(feature = "u8")]
    pub mod u8vec3;
    #[cfg(feature = "u8")]
    pub mod u8vec4;

    #[cfg(feature = "i16")]
    pub mod i16vec2;
    #[cfg(feature = "i16")]
    pub mod i16vec3;
    #[cfg(feature = "i16")]
    pub mod i16vec4;

    #[cfg(feature = "u16")]
    pub mod u16vec2;
    #[cfg(feature = "u16")]
    pub mod u16vec3;
    #[cfg(feature = "u16")]
    pub mod u16vec4;

    #[cfg(feature = "i32")]
    pub mod ivec2;
    #[cfg(feature = "i32")]
    pub mod ivec3;
    #[cfg(feature = "i32")]
    pub mod ivec4;

    #[cfg(feature = "u32")]
    pub mod uvec2;
    #[cfg(feature = "u32")]
    pub mod uvec3;
    #[cfg(feature = "u32")]
    pub mod uvec4;

    #[cfg(feature = "i64")]
    pub mod i64vec2;
    #[cfg(feature = "i64")]
    pub mod i64vec3;
    #[cfg(feature = "i64")]
    pub mod i64vec4;

    #[cfg(feature = "u64")]
    pub mod u64vec2;
    #[cfg(feature = "u64")]
    pub mod u64vec3;
    #[cfg(feature = "u64")]
    pub mod u64vec4;
}

pub use self::bool::bvec2::BVec2;
pub use self::bool::bvec3::BVec3;
pub use self::bool::bvec3a::BVec3A;
pub use self::bool::bvec4::BVec4;
pub use self::bool::bvec4a::BVec4A;

pub use self::f32::vec2::Vec2;
pub use self::f32::vec3::Vec3;
pub use self::f32::vec3a::Vec3A;
pub use self::f32::vec4::Vec4;

#[cfg(feature = "f64")]
pub use self::f64::dvec2::DVec2;
#[cfg(feature = "f64")]
pub use self::f64::dvec3::DVec3;
#[cfg(feature = "f64")]
pub use self::f64::dvec4::DVec4;

#[cfg(feature = "f16")]
pub use self::f16::f16vec2::F16Vec2;
#[cfg(feature = "f16")]
pub use self::f16::f16vec3::F16Vec3;
#[cfg(feature = "f16")]
pub use self::f16::f16vec4::F16Vec4;

#[cfg(feature = "i8")]
pub use self::int::{i8vec2::I8Vec2, i8vec3::I8Vec3, i8vec4::I8Vec4};
#[cfg(feature = "i16")]
pub use self::int::{i16vec2::I16Vec2, i16vec3::I16Vec3, i16vec4::I16Vec4};
#[cfg(feature = "i64")]
pub use self::int::{i64vec2::I64Vec2, i64vec3::I64Vec3, i64vec4::I64Vec4};
#[cfg(feature = "i32")]
pub use self::int::{ivec2::IVec2, ivec3::IVec3, ivec4::IVec4};
#[cfg(feature = "u8")]
pub use self::int::{u8vec2::U8Vec2, u8vec3::U8Vec3, u8vec4::U8Vec4};
#[cfg(feature = "u16")]
pub use self::int::{u16vec2::U16Vec2, u16vec3::U16Vec3, u16vec4::U16Vec4};
#[cfg(feature = "u64")]
pub use self::int::{u64vec2::U64Vec2, u64vec3::U64Vec3, u64vec4::U64Vec4};
#[cfg(feature = "u32")]
pub use self::int::{uvec2::UVec2, uvec3::UVec3, uvec4::UVec4};

const_assert_eq!(8, core::mem::size_of::<Vec2>());
const_assert_eq!(4, core::mem::align_of::<Vec2>());
const_assert_eq!(12, core::mem::size_of::<Vec3>());
const_assert_eq!(4, core::mem::align_of::<Vec3>());
const_assert_eq!(16, core::mem::size_of::<Vec3A>());
const_assert_eq!(16, core::mem::align_of::<Vec3A>());
const_assert_eq!(16, core::mem::size_of::<Vec4>());
const_assert_eq!(16, core::mem::align_of::<Vec4>());
const_assert_eq!(2, core::mem::size_of::<BVec2>());
const_assert_eq!(3, core::mem::size_of::<BVec3>());
const_assert_eq!(4, core::mem::size_of::<BVec4>());
const_assert_eq!(16, core::mem::size_of::<BVec3A>());
const_assert_eq!(16, core::mem::align_of::<BVec3A>());
const_assert_eq!(16, core::mem::size_of::<BVec4A>());
const_assert_eq!(16, core::mem::align_of::<BVec4A>());

#[cfg(feature = "f64")]
const_assert_eq!(16, core::mem::size_of::<DVec2>());
#[cfg(feature = "f64")]
const_assert_eq!(24, core::mem::size_of::<DVec3>());
#[cfg(feature = "f64")]
const_assert_eq!(32, core::mem::size_of::<DVec4>());

#[cfg(feature = "f16")]
const_assert_eq!(4, core::mem::size_of::<F16Vec2>());
#[cfg(feature = "f16")]
const_assert_eq!(6, core::mem::size_of::<F16Vec3>());
#[cfg(feature = "f16")]
const_assert_eq!(8, core::mem::size_of::<F16Vec4>());
