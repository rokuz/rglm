use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;
use crate::{Vec3, Vec3A, Vec4};

#[cfg(feature = "f64")]
use crate::{DVec3, DVec4};

#[cfg(feature = "f16")]
use crate::{F16Vec3, F16Vec4, f16};

mod private {
    pub trait Sealed {}
}

pub trait ColorSpaceOp: private::Sealed + Copy {
    fn linear_to_srgb_op(self) -> Self;
    fn srgb_to_linear_op(self) -> Self;
}

#[inline]
fn f32_linear_to_srgb(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * f32_math::powf(c, 1.0 / 2.4) - 0.055
    }
}

#[inline]
fn f32_srgb_to_linear(c: f32) -> f32 {
    if c <= 0.040_45 {
        c / 12.92
    } else {
        f32_math::powf((c + 0.055) / 1.055, 2.4)
    }
}

#[cfg(feature = "f64")]
#[inline]
fn f64_linear_to_srgb(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * f64_math::powf(c, 1.0 / 2.4) - 0.055
    }
}

#[cfg(feature = "f64")]
#[inline]
fn f64_srgb_to_linear(c: f64) -> f64 {
    if c <= 0.040_45 {
        c / 12.92
    } else {
        f64_math::powf((c + 0.055) / 1.055, 2.4)
    }
}

impl private::Sealed for Vec3 {}
impl ColorSpaceOp for Vec3 {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f32_linear_to_srgb(self.x),
            f32_linear_to_srgb(self.y),
            f32_linear_to_srgb(self.z),
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f32_srgb_to_linear(self.x),
            f32_srgb_to_linear(self.y),
            f32_srgb_to_linear(self.z),
        )
    }
}

impl private::Sealed for Vec3A {}
impl ColorSpaceOp for Vec3A {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f32_linear_to_srgb(self.x),
            f32_linear_to_srgb(self.y),
            f32_linear_to_srgb(self.z),
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f32_srgb_to_linear(self.x),
            f32_srgb_to_linear(self.y),
            f32_srgb_to_linear(self.z),
        )
    }
}

impl private::Sealed for Vec4 {}
impl ColorSpaceOp for Vec4 {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f32_linear_to_srgb(self.x),
            f32_linear_to_srgb(self.y),
            f32_linear_to_srgb(self.z),
            self.w,
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f32_srgb_to_linear(self.x),
            f32_srgb_to_linear(self.y),
            f32_srgb_to_linear(self.z),
            self.w,
        )
    }
}

#[cfg(feature = "f64")]
impl private::Sealed for DVec3 {}
#[cfg(feature = "f64")]
impl ColorSpaceOp for DVec3 {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f64_linear_to_srgb(self.x),
            f64_linear_to_srgb(self.y),
            f64_linear_to_srgb(self.z),
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f64_srgb_to_linear(self.x),
            f64_srgb_to_linear(self.y),
            f64_srgb_to_linear(self.z),
        )
    }
}

#[cfg(feature = "f64")]
impl private::Sealed for DVec4 {}
#[cfg(feature = "f64")]
impl ColorSpaceOp for DVec4 {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f64_linear_to_srgb(self.x),
            f64_linear_to_srgb(self.y),
            f64_linear_to_srgb(self.z),
            self.w,
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f64_srgb_to_linear(self.x),
            f64_srgb_to_linear(self.y),
            f64_srgb_to_linear(self.z),
            self.w,
        )
    }
}

#[cfg(feature = "f16")]
impl private::Sealed for F16Vec3 {}
#[cfg(feature = "f16")]
impl ColorSpaceOp for F16Vec3 {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f16::from_f32(f32_linear_to_srgb(self.x.to_f32())),
            f16::from_f32(f32_linear_to_srgb(self.y.to_f32())),
            f16::from_f32(f32_linear_to_srgb(self.z.to_f32())),
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f16::from_f32(f32_srgb_to_linear(self.x.to_f32())),
            f16::from_f32(f32_srgb_to_linear(self.y.to_f32())),
            f16::from_f32(f32_srgb_to_linear(self.z.to_f32())),
        )
    }
}

#[cfg(feature = "f16")]
impl private::Sealed for F16Vec4 {}
#[cfg(feature = "f16")]
impl ColorSpaceOp for F16Vec4 {
    #[inline]
    fn linear_to_srgb_op(self) -> Self {
        Self::new(
            f16::from_f32(f32_linear_to_srgb(self.x.to_f32())),
            f16::from_f32(f32_linear_to_srgb(self.y.to_f32())),
            f16::from_f32(f32_linear_to_srgb(self.z.to_f32())),
            self.w,
        )
    }

    #[inline]
    fn srgb_to_linear_op(self) -> Self {
        Self::new(
            f16::from_f32(f32_srgb_to_linear(self.x.to_f32())),
            f16::from_f32(f32_srgb_to_linear(self.y.to_f32())),
            f16::from_f32(f32_srgb_to_linear(self.z.to_f32())),
            self.w,
        )
    }
}

/// Converts linear RGB (preserving alpha `w` on 4D vectors) to IEC 61966-2-1 sRGB (`glm::convertLinearToSRGB`).
#[inline]
#[must_use]
pub fn convert_linear_to_srgb<C: ColorSpaceOp>(color_linear: C) -> C {
    color_linear.linear_to_srgb_op()
}

/// Converts IEC 61966-2-1 sRGB (preserving alpha `w` on 4D vectors) to linear RGB (`glm::convertSRGBToLinear`).
#[inline]
#[must_use]
pub fn convert_srgb_to_linear<C: ColorSpaceOp>(color_srgb: C) -> C {
    color_srgb.srgb_to_linear_op()
}
