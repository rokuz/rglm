#[cfg(feature = "f16")]
use crate::math::f16_math;
use crate::math::f32_math;
#[cfg(feature = "f64")]
use crate::math::f64_math;
use crate::{Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

#[cfg(feature = "f64")]
use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

#[cfg(feature = "f16")]
use crate::{F16Mat2, F16Mat3, F16Mat4, F16Quat, F16Vec2, F16Vec3, F16Vec4, f16};

mod private {
    pub trait Sealed {}
}

pub trait SafeNormalizeOp: private::Sealed + Copy + Sized {
    fn try_normalize_op(self) -> Option<Self>;
    fn normalize_or_zero_op(self) -> Self;
}

pub trait SafeLengthClampOp: SafeNormalizeOp {
    type Scalar: Copy;

    fn clamp_length_op(self, min_len: Self::Scalar, max_len: Self::Scalar) -> Self;
    fn clamp_length_max_op(self, max_len: Self::Scalar) -> Self;
}

pub trait SafeInverseOp: private::Sealed + Copy + Sized {
    fn try_inverse_op(self) -> Option<Self>;
}

macro_rules! impl_safe_vec {
    ($v:ty, $s:ty, $math:ident, $zero:expr) => {
        impl private::Sealed for $v {}

        impl SafeNormalizeOp for $v {
            #[inline]
            fn try_normalize_op(self) -> Option<Self> {
                let len_sq = self.dot(self);
                if len_sq > $zero && len_sq.is_finite() {
                    let rcp = $math::inversesqrt(len_sq);
                    if rcp.is_finite() {
                        return Some(self * rcp);
                    }
                }
                None
            }

            #[inline]
            fn normalize_or_zero_op(self) -> Self {
                self.try_normalize_op().unwrap_or(Self::ZERO)
            }
        }

        impl SafeLengthClampOp for $v {
            type Scalar = $s;

            #[inline]
            fn clamp_length_op(self, min_len: $s, max_len: $s) -> Self {
                let len_sq = self.dot(self);
                if !(len_sq > $zero && len_sq.is_finite()) {
                    return Self::ZERO;
                }
                if len_sq < min_len * min_len {
                    self * (min_len * $math::inversesqrt(len_sq))
                } else if len_sq > max_len * max_len {
                    self * (max_len * $math::inversesqrt(len_sq))
                } else {
                    self
                }
            }

            #[inline]
            fn clamp_length_max_op(self, max_len: $s) -> Self {
                let len_sq = self.dot(self);
                if len_sq > max_len * max_len && len_sq.is_finite() {
                    self * (max_len * $math::inversesqrt(len_sq))
                } else {
                    self
                }
            }
        }
    };
}

impl_safe_vec!(Vec2, f32, f32_math, 0.0_f32);
impl_safe_vec!(Vec3, f32, f32_math, 0.0_f32);
impl_safe_vec!(Vec3A, f32, f32_math, 0.0_f32);
impl_safe_vec!(Vec4, f32, f32_math, 0.0_f32);

#[cfg(feature = "f64")]
impl_safe_vec!(DVec2, f64, f64_math, 0.0_f64);
#[cfg(feature = "f64")]
impl_safe_vec!(DVec3, f64, f64_math, 0.0_f64);
#[cfg(feature = "f64")]
impl_safe_vec!(DVec4, f64, f64_math, 0.0_f64);

#[cfg(feature = "f16")]
impl_safe_vec!(F16Vec2, f16, f16_math, f16::ZERO);
#[cfg(feature = "f16")]
impl_safe_vec!(F16Vec3, f16, f16_math, f16::ZERO);
#[cfg(feature = "f16")]
impl_safe_vec!(F16Vec4, f16, f16_math, f16::ZERO);

macro_rules! impl_safe_quat {
    ($q:ty, $math:ident, $zero:expr) => {
        impl private::Sealed for $q {}

        impl SafeNormalizeOp for $q {
            #[inline]
            fn try_normalize_op(self) -> Option<Self> {
                let len_sq = self.dot(self);
                if len_sq > $zero && len_sq.is_finite() {
                    let rcp = $math::inversesqrt(len_sq);
                    if rcp.is_finite() {
                        return Some(self * rcp);
                    }
                }
                None
            }

            #[inline]
            fn normalize_or_zero_op(self) -> Self {
                self.try_normalize_op().unwrap_or(Self::ZERO)
            }
        }

        impl SafeInverseOp for $q {
            #[inline]
            fn try_inverse_op(self) -> Option<Self> {
                let len_sq = self.dot(self);
                if len_sq > $zero && len_sq.is_finite() {
                    let inv = self.conjugate() / len_sq;
                    let a = inv.to_array();
                    if a[0].is_finite() && a[1].is_finite() && a[2].is_finite() && a[3].is_finite()
                    {
                        return Some(inv);
                    }
                }
                None
            }
        }
    };
}

impl_safe_quat!(Quat, f32_math, 0.0_f32);
#[cfg(feature = "f64")]
impl_safe_quat!(DQuat, f64_math, 0.0_f64);
#[cfg(feature = "f16")]
impl_safe_quat!(F16Quat, f16_math, f16::ZERO);

macro_rules! impl_safe_mat {
    ($($m:ty => $zero:expr),+ $(,)?) => {
        $(
            impl private::Sealed for $m {}

            impl SafeInverseOp for $m {
                #[inline]
                fn try_inverse_op(self) -> Option<Self> {
                    let det = self.determinant();
                    if det == $zero || !det.is_finite() {
                        return None;
                    }
                    let inv = self.inverse();
                    let flat = crate::to_array(inv);
                    let mut i = 0;
                    while i < flat.len() {
                        if !flat[i].is_finite() {
                            return None;
                        }
                        i += 1;
                    }
                    Some(inv)
                }
            }
        )+
    };
}

impl_safe_mat!(
    Mat2 => 0.0_f32,
    Mat3 => 0.0_f32,
    Mat3A => 0.0_f32,
    Mat4 => 0.0_f32,
);

#[cfg(feature = "f64")]
impl_safe_mat!(
    DMat2 => 0.0_f64,
    DMat3 => 0.0_f64,
    DMat4 => 0.0_f64,
);

#[cfg(feature = "f16")]
impl_safe_mat!(
    F16Mat2 => f16::ZERO,
    F16Mat3 => f16::ZERO,
    F16Mat4 => f16::ZERO,
);

/// Normalizes `x`, returning `None` if `x` is zero, subnormal-overflowing, or non-finite.
#[inline]
#[must_use]
pub fn try_normalize<T: SafeNormalizeOp>(x: T) -> Option<T> {
    x.try_normalize_op()
}

/// Normalizes `x`, returning `ZERO` if `x` cannot be normalized finitely.
#[inline]
#[must_use]
pub fn normalize_or_zero<T: SafeNormalizeOp>(x: T) -> T {
    x.normalize_or_zero_op()
}

/// Normalizes `x`, returning `fallback` if `x` cannot be normalized finitely.
#[inline]
#[must_use]
pub fn normalize_or<T: SafeNormalizeOp>(x: T, fallback: T) -> T {
    x.try_normalize_op().unwrap_or(fallback)
}

/// Computes the multiplicative inverse of `x`, returning `None` if `x` is singular or non-finite.
#[inline]
#[must_use]
pub fn try_inverse<T: SafeInverseOp>(x: T) -> Option<T> {
    x.try_inverse_op()
}

/// Clamps the Euclidean length of `v` into `[min_len, max_len]`, returning `ZERO` if `v` is zero.
#[inline]
#[must_use]
pub fn clamp_length<V: SafeLengthClampOp>(v: V, min_len: V::Scalar, max_len: V::Scalar) -> V {
    v.clamp_length_op(min_len, max_len)
}

/// Caps the Euclidean length of `v` at `max_len` without panicking or producing `NaN` at zero.
#[inline]
#[must_use]
pub fn clamp_length_max<V: SafeLengthClampOp>(v: V, max_len: V::Scalar) -> V {
    v.clamp_length_max_op(max_len)
}
