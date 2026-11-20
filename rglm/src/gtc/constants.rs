mod private {
    pub trait Sealed {}
}

pub trait FloatConst: private::Sealed + Copy {
    const ZERO_VAL: Self;
    const ONE_VAL: Self;
    const EPSILON_VAL: Self;
    const PI_VAL: Self;
    const TWO_PI_VAL: Self;
    const HALF_PI_VAL: Self;
    const QUARTER_PI_VAL: Self;
    const ONE_OVER_PI_VAL: Self;
    const TWO_OVER_PI_VAL: Self;
    const ROOT_TWO_VAL: Self;
    const ROOT_THREE_VAL: Self;
    const EULER_VAL: Self;
    const GOLDEN_RATIO_VAL: Self;
}

impl private::Sealed for f32 {}
impl FloatConst for f32 {
    const ZERO_VAL: Self = 0.0;
    const ONE_VAL: Self = 1.0;
    const EPSILON_VAL: Self = f32::EPSILON;
    const PI_VAL: Self = core::f32::consts::PI;
    const TWO_PI_VAL: Self = core::f32::consts::TAU;
    const HALF_PI_VAL: Self = core::f32::consts::FRAC_PI_2;
    const QUARTER_PI_VAL: Self = core::f32::consts::FRAC_PI_4;
    const ONE_OVER_PI_VAL: Self = core::f32::consts::FRAC_1_PI;
    const TWO_OVER_PI_VAL: Self = core::f32::consts::FRAC_2_PI;
    const ROOT_TWO_VAL: Self = core::f32::consts::SQRT_2;
    const ROOT_THREE_VAL: Self = 1.732_050_8;
    const EULER_VAL: Self = core::f32::consts::E;
    const GOLDEN_RATIO_VAL: Self = 1.618_034;
}

#[cfg(feature = "f64")]
impl private::Sealed for f64 {}
#[cfg(feature = "f64")]
impl FloatConst for f64 {
    const ZERO_VAL: Self = 0.0;
    const ONE_VAL: Self = 1.0;
    const EPSILON_VAL: Self = f64::EPSILON;
    const PI_VAL: Self = core::f64::consts::PI;
    const TWO_PI_VAL: Self = core::f64::consts::TAU;
    const HALF_PI_VAL: Self = core::f64::consts::FRAC_PI_2;
    const QUARTER_PI_VAL: Self = core::f64::consts::FRAC_PI_4;
    const ONE_OVER_PI_VAL: Self = core::f64::consts::FRAC_1_PI;
    const TWO_OVER_PI_VAL: Self = core::f64::consts::FRAC_2_PI;
    const ROOT_TWO_VAL: Self = core::f64::consts::SQRT_2;
    const ROOT_THREE_VAL: Self = 1.732_050_807_568_877_2;
    const EULER_VAL: Self = core::f64::consts::E;
    const GOLDEN_RATIO_VAL: Self = 1.618_033_988_749_895;
}

#[cfg(feature = "f16")]
impl private::Sealed for half::f16 {}
#[cfg(feature = "f16")]
impl FloatConst for half::f16 {
    const ZERO_VAL: Self = half::f16::ZERO;
    const ONE_VAL: Self = half::f16::ONE;
    const EPSILON_VAL: Self = half::f16::EPSILON;
    const PI_VAL: Self = half::f16::PI;
    const TWO_PI_VAL: Self = half::f16::from_f32_const(core::f32::consts::TAU);
    const HALF_PI_VAL: Self = half::f16::FRAC_PI_2;
    const QUARTER_PI_VAL: Self = half::f16::FRAC_PI_4;
    const ONE_OVER_PI_VAL: Self = half::f16::FRAC_1_PI;
    const TWO_OVER_PI_VAL: Self = half::f16::FRAC_2_PI;
    const ROOT_TWO_VAL: Self = half::f16::SQRT_2;
    const ROOT_THREE_VAL: Self = half::f16::from_f32_const(1.732_050_8);
    const EULER_VAL: Self = half::f16::E;
    const GOLDEN_RATIO_VAL: Self = half::f16::from_f32_const(1.618_034);
}

/// Returns the additive identity `0` (`glm::zero`).
#[inline(always)]
#[must_use]
pub const fn zero<T: FloatConst>() -> T {
    T::ZERO_VAL
}

/// Returns the multiplicative identity `1` (`glm::one`).
#[inline(always)]
#[must_use]
pub const fn one<T: FloatConst>() -> T {
    T::ONE_VAL
}

/// Returns machine epsilon for floating-point type `T` (`glm::epsilon`).
#[inline(always)]
#[must_use]
pub const fn epsilon<T: FloatConst>() -> T {
    T::EPSILON_VAL
}

/// Returns Archimedes' constant `pi` (`glm::pi`).
#[inline(always)]
#[must_use]
pub const fn pi<T: FloatConst>() -> T {
    T::PI_VAL
}

/// Returns `2 * pi` (`glm::two_pi`).
#[inline(always)]
#[must_use]
pub const fn two_pi<T: FloatConst>() -> T {
    T::TWO_PI_VAL
}

/// Returns `pi / 2` (`glm::half_pi`).
#[inline(always)]
#[must_use]
pub const fn half_pi<T: FloatConst>() -> T {
    T::HALF_PI_VAL
}

/// Returns `pi / 4` (`glm::quarter_pi`).
#[inline(always)]
#[must_use]
pub const fn quarter_pi<T: FloatConst>() -> T {
    T::QUARTER_PI_VAL
}

/// Returns `1 / pi` (`glm::one_over_pi`).
#[inline(always)]
#[must_use]
pub const fn one_over_pi<T: FloatConst>() -> T {
    T::ONE_OVER_PI_VAL
}

/// Returns `2 / pi` (`glm::two_over_pi`).
#[inline(always)]
#[must_use]
pub const fn two_over_pi<T: FloatConst>() -> T {
    T::TWO_OVER_PI_VAL
}

/// Returns `sqrt(2)` (`glm::root_two`).
#[inline(always)]
#[must_use]
pub const fn root_two<T: FloatConst>() -> T {
    T::ROOT_TWO_VAL
}

/// Returns `sqrt(3)` (`glm::root_three`).
#[inline(always)]
#[must_use]
pub const fn root_three<T: FloatConst>() -> T {
    T::ROOT_THREE_VAL
}

/// Returns Euler's number `e` (`glm::euler`).
#[inline(always)]
#[must_use]
pub const fn euler<T: FloatConst>() -> T {
    T::EULER_VAL
}

/// Returns the golden ratio `(1 + sqrt(5)) / 2` (`glm::golden_ratio`).
#[inline(always)]
#[must_use]
pub const fn golden_ratio<T: FloatConst>() -> T {
    T::GOLDEN_RATIO_VAL
}
