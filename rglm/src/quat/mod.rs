#![expect(
    dead_code,
    reason = "internal quaternion kernels are wired to public free functions in M2-M4"
)]

mod f32quat;

#[cfg(feature = "f64")]
mod dquat;

#[cfg(feature = "f16")]
mod f16quat;

pub use f32quat::Quat;

#[cfg(feature = "f64")]
pub use dquat::DQuat;

#[cfg(feature = "f16")]
pub use f16quat::F16Quat;

const_assert_eq!(16, core::mem::size_of::<Quat>());
const_assert_eq!(16, core::mem::align_of::<Quat>());

#[cfg(feature = "f64")]
const_assert_eq!(32, core::mem::size_of::<DQuat>());

#[cfg(feature = "f16")]
const_assert_eq!(8, core::mem::size_of::<F16Quat>());
