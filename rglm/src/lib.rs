#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]
#![cfg_attr(
    all(feature = "core-simd", not(feature = "scalar-math")),
    feature(portable_simd)
)]

extern crate self as rglm;

#[cfg(all(
    not(feature = "std"),
    not(feature = "libm"),
    not(feature = "nostd-libm")
))]
compile_error!("Enable either `std`, `libm`, or `nostd-libm` to select a scalar math backend.");

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
#[macro_use]
mod macros;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod deref;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub(crate) mod math;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub(crate) mod simd;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
mod vec;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
mod mat;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
#[path = "quat/mod.rs"]
mod quat_types;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod swizzle;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod construct;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod builtins;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod gtc;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod gtx;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod glsl_names;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod layout;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub mod aligned;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
mod features;

#[cfg(all(
    feature = "f16",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
#[expect(
    non_camel_case_types,
    reason = "half::f16 uses Rust primitive-style lowercase naming"
)]
pub type f16 = half::f16;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use layout::{
    Scalar, Std140, Std430, to_scalar, to_std140, to_std430, write_scalar, write_std140,
    write_std430,
};
#[cfg(all(
    feature = "derive",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use rglm_derive::{Scalar, Std140, Std430};

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use builtins::*;
#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use construct::{
    AngleAxisOp, Construct, FlatArray, MatCastQuatOp, QuatCastOp, angle_axis, bvec2, bvec3, bvec3a,
    bvec4, bvec4a, from_slice, mat2, mat2x2, mat2x3, mat2x3a, mat2x4, mat3, mat3_cast, mat3a,
    mat3x2, mat3x3, mat3x3a, mat3x4, mat4, mat4_cast, mat4x2, mat4x3, mat4x3a, mat4x4, quat_cast,
    quat_identity, quat_wxyz, quat_xyzw, to_array, vec2, vec3, vec3a, vec4, write_to_slice,
};
#[cfg(all(
    feature = "f64",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{
    dmat2, dmat2x2, dmat2x3, dmat2x4, dmat3, dmat3x2, dmat3x3, dmat3x4, dmat4, dmat4x2, dmat4x3,
    dmat4x4, dquat_identity, dquat_wxyz, dquat_xyzw, dvec2, dvec3, dvec4,
};
#[cfg(all(
    feature = "f16",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{
    f16mat2, f16mat2x2, f16mat2x3, f16mat2x4, f16mat3, f16mat3x2, f16mat3x3, f16mat3x4, f16mat4,
    f16mat4x2, f16mat4x3, f16mat4x4, f16quat_identity, f16quat_wxyz, f16quat_xyzw, f16vec2,
    f16vec3, f16vec4,
};
#[cfg(all(
    feature = "i8",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{i8vec2, i8vec3, i8vec4};
#[cfg(all(
    feature = "i16",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{i16vec2, i16vec3, i16vec4};
#[cfg(all(
    feature = "i64",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{i64vec2, i64vec3, i64vec4};
#[cfg(all(
    feature = "i32",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{ivec2, ivec3, ivec4};
#[cfg(all(
    feature = "u8",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{u8vec2, u8vec3, u8vec4};
#[cfg(all(
    feature = "u16",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{u16vec2, u16vec3, u16vec4};
#[cfg(all(
    feature = "u64",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{u64vec2, u64vec3, u64vec4};
#[cfg(all(
    feature = "u32",
    any(feature = "std", feature = "libm", feature = "nostd-libm")
))]
pub use construct::{uvec2, uvec3, uvec4};
#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use gtc::*;
#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use mat::*;
#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use quat_types::*;
#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use vec::*;

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
mod glsl_types {
    #![expect(
        non_camel_case_types,
        reason = "GLSL standard uses lowercase vector, matrix, and quaternion type names"
    )]

    pub type vec2 = crate::Vec2;
    pub type vec3 = crate::Vec3;
    pub type vec3a = crate::Vec3A;
    pub type aligned_vec3 = crate::Vec3A;
    pub type vec4 = crate::Vec4;

    pub type bvec2 = crate::BVec2;
    pub type bvec3 = crate::BVec3;
    pub type bvec3a = crate::BVec3A;
    pub type bvec4 = crate::BVec4;
    pub type bvec4a = crate::BVec4A;

    pub type mat2 = crate::Mat2;
    pub type mat2x2 = crate::Mat2;
    pub type mat2x3 = crate::Mat2x3;
    pub type mat2x3a = crate::Mat2x3A;
    pub type mat2x4 = crate::Mat2x4;
    pub type mat3x2 = crate::Mat3x2;
    pub type mat3 = crate::Mat3;
    pub type mat3x3 = crate::Mat3;
    pub type mat3a = crate::Mat3A;
    pub type mat3x3a = crate::Mat3A;
    pub type mat3x4 = crate::Mat3x4;
    pub type mat4x2 = crate::Mat4x2;
    pub type mat4x3 = crate::Mat4x3;
    pub type mat4x3a = crate::Mat4x3A;
    pub type mat4 = crate::Mat4;
    pub type mat4x4 = crate::Mat4;

    pub type quat = crate::Quat;

    #[cfg(feature = "f64")]
    pub type dvec2 = crate::DVec2;
    #[cfg(feature = "f64")]
    pub type dvec3 = crate::DVec3;
    #[cfg(feature = "f64")]
    pub type dvec4 = crate::DVec4;
    #[cfg(feature = "f64")]
    pub type dmat2 = crate::DMat2;
    #[cfg(feature = "f64")]
    pub type dmat2x2 = crate::DMat2;
    #[cfg(feature = "f64")]
    pub type dmat2x3 = crate::DMat2x3;
    #[cfg(feature = "f64")]
    pub type dmat2x4 = crate::DMat2x4;
    #[cfg(feature = "f64")]
    pub type dmat3x2 = crate::DMat3x2;
    #[cfg(feature = "f64")]
    pub type dmat3 = crate::DMat3;
    #[cfg(feature = "f64")]
    pub type dmat3x3 = crate::DMat3;
    #[cfg(feature = "f64")]
    pub type dmat3x4 = crate::DMat3x4;
    #[cfg(feature = "f64")]
    pub type dmat4x2 = crate::DMat4x2;
    #[cfg(feature = "f64")]
    pub type dmat4x3 = crate::DMat4x3;
    #[cfg(feature = "f64")]
    pub type dmat4 = crate::DMat4;
    #[cfg(feature = "f64")]
    pub type dmat4x4 = crate::DMat4;
    #[cfg(feature = "f64")]
    pub type dquat = crate::DQuat;

    #[cfg(feature = "f16")]
    pub type f16vec2 = crate::F16Vec2;
    #[cfg(feature = "f16")]
    pub type f16vec3 = crate::F16Vec3;
    #[cfg(feature = "f16")]
    pub type f16vec4 = crate::F16Vec4;
    #[cfg(feature = "f16")]
    pub type f16mat2 = crate::F16Mat2;
    #[cfg(feature = "f16")]
    pub type f16mat2x2 = crate::F16Mat2;
    #[cfg(feature = "f16")]
    pub type f16mat2x3 = crate::F16Mat2x3;
    #[cfg(feature = "f16")]
    pub type f16mat2x4 = crate::F16Mat2x4;
    #[cfg(feature = "f16")]
    pub type f16mat3x2 = crate::F16Mat3x2;
    #[cfg(feature = "f16")]
    pub type f16mat3 = crate::F16Mat3;
    #[cfg(feature = "f16")]
    pub type f16mat3x3 = crate::F16Mat3;
    #[cfg(feature = "f16")]
    pub type f16mat3x4 = crate::F16Mat3x4;
    #[cfg(feature = "f16")]
    pub type f16mat4x2 = crate::F16Mat4x2;
    #[cfg(feature = "f16")]
    pub type f16mat4x3 = crate::F16Mat4x3;
    #[cfg(feature = "f16")]
    pub type f16mat4 = crate::F16Mat4;
    #[cfg(feature = "f16")]
    pub type f16mat4x4 = crate::F16Mat4;
    #[cfg(feature = "f16")]
    pub type f16quat = crate::F16Quat;

    #[cfg(feature = "i8")]
    pub type i8vec2 = crate::I8Vec2;
    #[cfg(feature = "i8")]
    pub type i8vec3 = crate::I8Vec3;
    #[cfg(feature = "i8")]
    pub type i8vec4 = crate::I8Vec4;

    #[cfg(feature = "u8")]
    pub type u8vec2 = crate::U8Vec2;
    #[cfg(feature = "u8")]
    pub type u8vec3 = crate::U8Vec3;
    #[cfg(feature = "u8")]
    pub type u8vec4 = crate::U8Vec4;

    #[cfg(feature = "i16")]
    pub type i16vec2 = crate::I16Vec2;
    #[cfg(feature = "i16")]
    pub type i16vec3 = crate::I16Vec3;
    #[cfg(feature = "i16")]
    pub type i16vec4 = crate::I16Vec4;

    #[cfg(feature = "u16")]
    pub type u16vec2 = crate::U16Vec2;
    #[cfg(feature = "u16")]
    pub type u16vec3 = crate::U16Vec3;
    #[cfg(feature = "u16")]
    pub type u16vec4 = crate::U16Vec4;

    #[cfg(feature = "i32")]
    pub type ivec2 = crate::IVec2;
    #[cfg(feature = "i32")]
    pub type ivec3 = crate::IVec3;
    #[cfg(feature = "i32")]
    pub type ivec4 = crate::IVec4;
    #[cfg(feature = "i32")]
    pub type i32vec2 = crate::IVec2;
    #[cfg(feature = "i32")]
    pub type i32vec3 = crate::IVec3;
    #[cfg(feature = "i32")]
    pub type i32vec4 = crate::IVec4;

    #[cfg(feature = "u32")]
    pub type uvec2 = crate::UVec2;
    #[cfg(feature = "u32")]
    pub type uvec3 = crate::UVec3;
    #[cfg(feature = "u32")]
    pub type uvec4 = crate::UVec4;
    #[cfg(feature = "u32")]
    pub type u32vec2 = crate::UVec2;
    #[cfg(feature = "u32")]
    pub type u32vec3 = crate::UVec3;
    #[cfg(feature = "u32")]
    pub type u32vec4 = crate::UVec4;

    #[cfg(feature = "i64")]
    pub type i64vec2 = crate::I64Vec2;
    #[cfg(feature = "i64")]
    pub type i64vec3 = crate::I64Vec3;
    #[cfg(feature = "i64")]
    pub type i64vec4 = crate::I64Vec4;

    #[cfg(feature = "u64")]
    pub type u64vec2 = crate::U64Vec2;
    #[cfg(feature = "u64")]
    pub type u64vec3 = crate::U64Vec3;
    #[cfg(feature = "u64")]
    pub type u64vec4 = crate::U64Vec4;
}

#[cfg(any(feature = "std", feature = "libm", feature = "nostd-libm"))]
pub use glsl_types::*;
