pub(crate) mod f32 {
    pub mod mat2;
    pub mod mat2x3;
    pub mod mat2x3a;
    pub mod mat2x4;
    pub mod mat3;
    pub mod mat3a;
    pub mod mat3x2;
    pub mod mat3x4;
    pub mod mat4;
    pub mod mat4x2;
    pub mod mat4x3;
    pub mod mat4x3a;
}

#[cfg(feature = "f64")]
pub(crate) mod f64 {
    pub mod dmat2;
    pub mod dmat2x3;
    pub mod dmat2x4;
    pub mod dmat3;
    pub mod dmat3x2;
    pub mod dmat3x4;
    pub mod dmat4;
    pub mod dmat4x2;
    pub mod dmat4x3;
}

#[cfg(feature = "f16")]
pub(crate) mod f16 {
    pub mod f16mat2;
    pub mod f16mat2x3;
    pub mod f16mat2x4;
    pub mod f16mat3;
    pub mod f16mat3x2;
    pub mod f16mat3x4;
    pub mod f16mat4;
    pub mod f16mat4x2;
    pub mod f16mat4x3;
}

pub use self::f32::{
    mat2::Mat2, mat2x3::Mat2x3, mat2x3a::Mat2x3A, mat2x4::Mat2x4, mat3::Mat3, mat3a::Mat3A,
    mat3x2::Mat3x2, mat3x4::Mat3x4, mat4::Mat4, mat4x2::Mat4x2, mat4x3::Mat4x3, mat4x3a::Mat4x3A,
};
pub type Mat2x2 = Mat2;
pub type Mat3x3 = Mat3;
pub type Mat3x3A = Mat3A;
pub type Mat4x4 = Mat4;

#[cfg(feature = "f64")]
pub use self::f64::{
    dmat2::DMat2, dmat2x3::DMat2x3, dmat2x4::DMat2x4, dmat3::DMat3, dmat3x2::DMat3x2,
    dmat3x4::DMat3x4, dmat4::DMat4, dmat4x2::DMat4x2, dmat4x3::DMat4x3,
};
#[cfg(feature = "f64")]
pub type DMat2x2 = DMat2;
#[cfg(feature = "f64")]
pub type DMat3x3 = DMat3;
#[cfg(feature = "f64")]
pub type DMat4x4 = DMat4;

#[cfg(feature = "f16")]
pub use self::f16::{
    f16mat2::F16Mat2, f16mat2x3::F16Mat2x3, f16mat2x4::F16Mat2x4, f16mat3::F16Mat3,
    f16mat3x2::F16Mat3x2, f16mat3x4::F16Mat3x4, f16mat4::F16Mat4, f16mat4x2::F16Mat4x2,
    f16mat4x3::F16Mat4x3,
};
#[cfg(feature = "f16")]
pub type F16Mat2x2 = F16Mat2;
#[cfg(feature = "f16")]
pub type F16Mat3x3 = F16Mat3;
#[cfg(feature = "f16")]
pub type F16Mat4x4 = F16Mat4;

const_assert_eq!(16, core::mem::size_of::<Mat2>());
const_assert_eq!(16, core::mem::align_of::<Mat2>());
const_assert_eq!(24, core::mem::size_of::<Mat2x3>());
const_assert_eq!(32, core::mem::size_of::<Mat2x3A>());
const_assert_eq!(32, core::mem::size_of::<Mat2x4>());
const_assert_eq!(24, core::mem::size_of::<Mat3x2>());
const_assert_eq!(36, core::mem::size_of::<Mat3>());
const_assert_eq!(48, core::mem::size_of::<Mat3A>());
const_assert_eq!(48, core::mem::size_of::<Mat3x4>());
const_assert_eq!(32, core::mem::size_of::<Mat4x2>());
const_assert_eq!(16, core::mem::align_of::<Mat4x2>());
const_assert_eq!(48, core::mem::size_of::<Mat4x3>());
const_assert_eq!(64, core::mem::size_of::<Mat4x3A>());
const_assert_eq!(64, core::mem::size_of::<Mat4>());
const_assert_eq!(16, core::mem::align_of::<Mat4>());
