pub(crate) mod f32_math {
    #[cfg(any(feature = "libm", all(feature = "nostd-libm", not(feature = "std"))))]
    mod backend {
        #[inline(always)]
        pub(crate) fn sin(x: f32) -> f32 {
            libm::sinf(x)
        }

        #[inline(always)]
        pub(crate) fn cos(x: f32) -> f32 {
            libm::cosf(x)
        }

        #[inline(always)]
        pub(crate) fn sin_cos(x: f32) -> (f32, f32) {
            libm::sincosf(x)
        }

        #[inline(always)]
        pub(crate) fn tan(x: f32) -> f32 {
            libm::tanf(x)
        }

        #[inline(always)]
        pub(crate) fn asin(x: f32) -> f32 {
            libm::asinf(x)
        }

        #[inline(always)]
        pub(crate) fn acos(x: f32) -> f32 {
            libm::acosf(x)
        }

        #[inline(always)]
        pub(crate) fn atan(x: f32) -> f32 {
            libm::atanf(x)
        }

        #[inline(always)]
        pub(crate) fn atan2(y: f32, x: f32) -> f32 {
            libm::atan2f(y, x)
        }

        #[inline(always)]
        pub(crate) fn sinh(x: f32) -> f32 {
            libm::sinhf(x)
        }

        #[inline(always)]
        pub(crate) fn cosh(x: f32) -> f32 {
            libm::coshf(x)
        }

        #[inline(always)]
        pub(crate) fn tanh(x: f32) -> f32 {
            libm::tanhf(x)
        }

        #[inline(always)]
        pub(crate) fn asinh(x: f32) -> f32 {
            libm::asinhf(x)
        }

        #[inline(always)]
        pub(crate) fn acosh(x: f32) -> f32 {
            libm::acoshf(x)
        }

        #[inline(always)]
        pub(crate) fn atanh(x: f32) -> f32 {
            libm::atanhf(x)
        }

        #[inline(always)]
        pub(crate) fn exp(x: f32) -> f32 {
            libm::expf(x)
        }

        #[inline(always)]
        pub(crate) fn exp2(x: f32) -> f32 {
            libm::exp2f(x)
        }

        #[inline(always)]
        pub(crate) fn ln(x: f32) -> f32 {
            libm::logf(x)
        }

        #[inline(always)]
        pub(crate) fn log2(x: f32) -> f32 {
            libm::log2f(x)
        }

        #[inline(always)]
        pub(crate) fn powf(x: f32, y: f32) -> f32 {
            libm::powf(x, y)
        }

        #[inline(always)]
        pub(crate) fn sqrt(x: f32) -> f32 {
            libm::sqrtf(x)
        }

        #[inline(always)]
        pub(crate) fn floor(x: f32) -> f32 {
            libm::floorf(x)
        }

        #[inline(always)]
        pub(crate) fn ceil(x: f32) -> f32 {
            libm::ceilf(x)
        }

        #[inline(always)]
        pub(crate) fn trunc(x: f32) -> f32 {
            libm::truncf(x)
        }

        #[inline(always)]
        pub(crate) fn round(x: f32) -> f32 {
            libm::roundf(x)
        }

        #[inline(always)]
        pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
            libm::fmaf(a, b, c)
        }
    }

    #[cfg(all(not(feature = "libm"), feature = "std"))]
    mod backend {
        #[inline(always)]
        pub(crate) fn sin(x: f32) -> f32 {
            f32::sin(x)
        }

        #[inline(always)]
        pub(crate) fn cos(x: f32) -> f32 {
            f32::cos(x)
        }

        #[inline(always)]
        pub(crate) fn sin_cos(x: f32) -> (f32, f32) {
            f32::sin_cos(x)
        }

        #[inline(always)]
        pub(crate) fn tan(x: f32) -> f32 {
            f32::tan(x)
        }

        #[inline(always)]
        pub(crate) fn asin(x: f32) -> f32 {
            f32::asin(x)
        }

        #[inline(always)]
        pub(crate) fn acos(x: f32) -> f32 {
            f32::acos(x)
        }

        #[inline(always)]
        pub(crate) fn atan(x: f32) -> f32 {
            f32::atan(x)
        }

        #[inline(always)]
        pub(crate) fn atan2(y: f32, x: f32) -> f32 {
            f32::atan2(y, x)
        }

        #[inline(always)]
        pub(crate) fn sinh(x: f32) -> f32 {
            f64::sinh(x as f64) as f32
        }

        #[inline(always)]
        pub(crate) fn cosh(x: f32) -> f32 {
            f64::cosh(x as f64) as f32
        }

        #[inline(always)]
        pub(crate) fn tanh(x: f32) -> f32 {
            f64::tanh(x as f64) as f32
        }

        #[inline(always)]
        pub(crate) fn asinh(x: f32) -> f32 {
            f64::asinh(x as f64) as f32
        }

        #[inline(always)]
        pub(crate) fn acosh(x: f32) -> f32 {
            f64::acosh(x as f64) as f32
        }

        #[inline(always)]
        pub(crate) fn atanh(x: f32) -> f32 {
            f64::atanh(x as f64) as f32
        }

        #[inline(always)]
        pub(crate) fn exp(x: f32) -> f32 {
            f32::exp(x)
        }

        #[inline(always)]
        pub(crate) fn exp2(x: f32) -> f32 {
            f32::exp2(x)
        }

        #[inline(always)]
        pub(crate) fn ln(x: f32) -> f32 {
            f32::ln(x)
        }

        #[inline(always)]
        pub(crate) fn log2(x: f32) -> f32 {
            f32::log2(x)
        }

        #[inline(always)]
        pub(crate) fn powf(x: f32, y: f32) -> f32 {
            f32::powf(x, y)
        }

        #[inline(always)]
        pub(crate) fn sqrt(x: f32) -> f32 {
            f32::sqrt(x)
        }

        #[inline(always)]
        pub(crate) fn floor(x: f32) -> f32 {
            f32::floor(x)
        }

        #[inline(always)]
        pub(crate) fn ceil(x: f32) -> f32 {
            f32::ceil(x)
        }

        #[inline(always)]
        pub(crate) fn trunc(x: f32) -> f32 {
            f32::trunc(x)
        }

        #[inline(always)]
        pub(crate) fn round(x: f32) -> f32 {
            f32::round(x)
        }

        #[inline(always)]
        pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
            f32::mul_add(a, b, c)
        }
    }

    pub(crate) use backend::*;

    #[inline(always)]
    pub(crate) fn abs(x: f32) -> f32 {
        f32::from_bits(x.to_bits() & 0x7fff_ffff)
    }

    #[inline(always)]
    pub(crate) fn sign(x: f32) -> f32 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub(crate) fn round_even(x: f32) -> f32 {
        const MAGIC: f32 = 8_388_608.0;
        let ax = abs(x);
        if ax < MAGIC {
            let s = f32::from_bits((x.to_bits() & 0x8000_0000) | MAGIC.to_bits());
            let r = (x + s) - s;
            if r == 0.0 {
                f32::from_bits(x.to_bits() & 0x8000_0000)
            } else {
                r
            }
        } else {
            x
        }
    }

    #[inline(always)]
    pub(crate) fn fract(x: f32) -> f32 {
        x - floor(x)
    }

    #[inline(always)]
    pub(crate) fn modulo(x: f32, y: f32) -> f32 {
        x - y * floor(x / y)
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(x: f32) -> f32 {
        1.0 / sqrt(x)
    }

    #[inline]
    pub(crate) fn frexp(x: f32) -> (f32, i32) {
        let bits = x.to_bits();
        let exp = ((bits >> 23) & 0xff) as i32;
        if exp == 0xff {
            return (x, 0);
        }
        if (bits & 0x7fff_ffff) == 0 {
            return (x, 0);
        }
        if exp == 0 {
            let scaled = x * f32::from_bits(0x4b80_0000);
            let sbits = scaled.to_bits();
            let sexp = ((sbits >> 23) & 0xff) as i32;
            let mant = f32::from_bits((sbits & 0x807f_ffff) | 0x3f00_0000);
            return (mant, sexp - 126 - 24);
        }
        let mant = f32::from_bits((bits & 0x807f_ffff) | 0x3f00_0000);
        (mant, exp - 126)
    }

    #[inline]
    pub(crate) fn ldexp(x: f32, exp: i32) -> f32 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        let mut e = exp.clamp(-300, 300);
        let mut res = x;
        while e > 127 {
            res *= f32::from_bits(0x7f00_0000);
            e -= 127;
        }
        while e < -126 {
            res *= f32::from_bits(0x0080_0000);
            e += 126;
        }
        let factor = f32::from_bits(((e + 127) as u32) << 23);
        res * factor
    }
}

#[cfg(feature = "f64")]
pub(crate) mod f64_math {
    #[cfg(any(feature = "libm", all(feature = "nostd-libm", not(feature = "std"))))]
    mod backend {
        #[inline(always)]
        pub(crate) fn sin(x: f64) -> f64 {
            libm::sin(x)
        }

        #[inline(always)]
        pub(crate) fn cos(x: f64) -> f64 {
            libm::cos(x)
        }

        #[inline(always)]
        pub(crate) fn sin_cos(x: f64) -> (f64, f64) {
            libm::sincos(x)
        }

        #[inline(always)]
        pub(crate) fn tan(x: f64) -> f64 {
            libm::tan(x)
        }

        #[inline(always)]
        pub(crate) fn asin(x: f64) -> f64 {
            libm::asin(x)
        }

        #[inline(always)]
        pub(crate) fn acos(x: f64) -> f64 {
            libm::acos(x)
        }

        #[inline(always)]
        pub(crate) fn atan(x: f64) -> f64 {
            libm::atan(x)
        }

        #[inline(always)]
        pub(crate) fn atan2(y: f64, x: f64) -> f64 {
            libm::atan2(y, x)
        }

        #[inline(always)]
        pub(crate) fn sinh(x: f64) -> f64 {
            libm::sinh(x)
        }

        #[inline(always)]
        pub(crate) fn cosh(x: f64) -> f64 {
            libm::cosh(x)
        }

        #[inline(always)]
        pub(crate) fn tanh(x: f64) -> f64 {
            libm::tanh(x)
        }

        #[inline(always)]
        pub(crate) fn asinh(x: f64) -> f64 {
            libm::asinh(x)
        }

        #[inline(always)]
        pub(crate) fn acosh(x: f64) -> f64 {
            libm::acosh(x)
        }

        #[inline(always)]
        pub(crate) fn atanh(x: f64) -> f64 {
            libm::atanh(x)
        }

        #[inline(always)]
        pub(crate) fn exp(x: f64) -> f64 {
            libm::exp(x)
        }

        #[inline(always)]
        pub(crate) fn exp2(x: f64) -> f64 {
            libm::exp2(x)
        }

        #[inline(always)]
        pub(crate) fn ln(x: f64) -> f64 {
            libm::log(x)
        }

        #[inline(always)]
        pub(crate) fn log2(x: f64) -> f64 {
            libm::log2(x)
        }

        #[inline(always)]
        pub(crate) fn powf(x: f64, y: f64) -> f64 {
            libm::pow(x, y)
        }

        #[inline(always)]
        pub(crate) fn sqrt(x: f64) -> f64 {
            libm::sqrt(x)
        }

        #[inline(always)]
        pub(crate) fn floor(x: f64) -> f64 {
            libm::floor(x)
        }

        #[inline(always)]
        pub(crate) fn ceil(x: f64) -> f64 {
            libm::ceil(x)
        }

        #[inline(always)]
        pub(crate) fn trunc(x: f64) -> f64 {
            libm::trunc(x)
        }

        #[inline(always)]
        pub(crate) fn round(x: f64) -> f64 {
            libm::round(x)
        }

        #[inline(always)]
        pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 {
            libm::fma(a, b, c)
        }
    }

    #[cfg(all(not(feature = "libm"), feature = "std"))]
    mod backend {
        #[inline(always)]
        pub(crate) fn sin(x: f64) -> f64 {
            f64::sin(x)
        }

        #[inline(always)]
        pub(crate) fn cos(x: f64) -> f64 {
            f64::cos(x)
        }

        #[inline(always)]
        pub(crate) fn sin_cos(x: f64) -> (f64, f64) {
            f64::sin_cos(x)
        }

        #[inline(always)]
        pub(crate) fn tan(x: f64) -> f64 {
            f64::tan(x)
        }

        #[inline(always)]
        pub(crate) fn asin(x: f64) -> f64 {
            f64::asin(x)
        }

        #[inline(always)]
        pub(crate) fn acos(x: f64) -> f64 {
            f64::acos(x)
        }

        #[inline(always)]
        pub(crate) fn atan(x: f64) -> f64 {
            f64::atan(x)
        }

        #[inline(always)]
        pub(crate) fn atan2(y: f64, x: f64) -> f64 {
            f64::atan2(y, x)
        }

        #[inline(always)]
        pub(crate) fn sinh(x: f64) -> f64 {
            f64::sinh(x)
        }

        #[inline(always)]
        pub(crate) fn cosh(x: f64) -> f64 {
            f64::cosh(x)
        }

        #[inline(always)]
        pub(crate) fn tanh(x: f64) -> f64 {
            f64::tanh(x)
        }

        #[inline(always)]
        pub(crate) fn asinh(x: f64) -> f64 {
            f64::asinh(x)
        }

        #[inline(always)]
        pub(crate) fn acosh(x: f64) -> f64 {
            f64::acosh(x)
        }

        #[inline(always)]
        pub(crate) fn atanh(x: f64) -> f64 {
            f64::atanh(x)
        }

        #[inline(always)]
        pub(crate) fn exp(x: f64) -> f64 {
            f64::exp(x)
        }

        #[inline(always)]
        pub(crate) fn exp2(x: f64) -> f64 {
            f64::exp2(x)
        }

        #[inline(always)]
        pub(crate) fn ln(x: f64) -> f64 {
            f64::ln(x)
        }

        #[inline(always)]
        pub(crate) fn log2(x: f64) -> f64 {
            f64::log2(x)
        }

        #[inline(always)]
        pub(crate) fn powf(x: f64, y: f64) -> f64 {
            f64::powf(x, y)
        }

        #[inline(always)]
        pub(crate) fn sqrt(x: f64) -> f64 {
            f64::sqrt(x)
        }

        #[inline(always)]
        pub(crate) fn floor(x: f64) -> f64 {
            f64::floor(x)
        }

        #[inline(always)]
        pub(crate) fn ceil(x: f64) -> f64 {
            f64::ceil(x)
        }

        #[inline(always)]
        pub(crate) fn trunc(x: f64) -> f64 {
            f64::trunc(x)
        }

        #[inline(always)]
        pub(crate) fn round(x: f64) -> f64 {
            f64::round(x)
        }

        #[inline(always)]
        pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 {
            f64::mul_add(a, b, c)
        }
    }

    pub(crate) use backend::*;

    #[inline(always)]
    pub(crate) fn abs(x: f64) -> f64 {
        f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffff)
    }

    #[inline(always)]
    pub(crate) fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub(crate) fn round_even(x: f64) -> f64 {
        const MAGIC: f64 = 4_503_599_627_370_496.0;
        let ax = abs(x);
        if ax < MAGIC {
            let s = f64::from_bits((x.to_bits() & 0x8000_0000_0000_0000) | MAGIC.to_bits());
            let r = (x + s) - s;
            if r == 0.0 {
                f64::from_bits(x.to_bits() & 0x8000_0000_0000_0000)
            } else {
                r
            }
        } else {
            x
        }
    }

    #[inline(always)]
    pub(crate) fn fract(x: f64) -> f64 {
        x - floor(x)
    }

    #[inline(always)]
    pub(crate) fn modulo(x: f64, y: f64) -> f64 {
        x - y * floor(x / y)
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(x: f64) -> f64 {
        1.0 / sqrt(x)
    }

    #[inline]
    pub(crate) fn frexp(x: f64) -> (f64, i32) {
        let bits = x.to_bits();
        let exp = ((bits >> 52) & 0x7ff) as i32;
        if exp == 0x7ff {
            return (x, 0);
        }
        if (bits & 0x7fff_ffff_ffff_ffff) == 0 {
            return (x, 0);
        }
        if exp == 0 {
            let scaled = x * f64::from_bits(0x4350_0000_0000_0000);
            let sbits = scaled.to_bits();
            let sexp = ((sbits >> 52) & 0x7ff) as i32;
            let mant = f64::from_bits((sbits & 0x800f_ffff_ffff_ffff) | 0x3fe0_0000_0000_0000);
            return (mant, sexp - 1022 - 54);
        }
        let mant = f64::from_bits((bits & 0x800f_ffff_ffff_ffff) | 0x3fe0_0000_0000_0000);
        (mant, exp - 1022)
    }

    #[inline]
    pub(crate) fn ldexp(x: f64, exp: i32) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        let mut e = exp.clamp(-2200, 2200);
        let mut res = x;
        while e > 1023 {
            res *= f64::from_bits(0x7fe0_0000_0000_0000);
            e -= 1023;
        }
        while e < -1022 {
            res *= f64::from_bits(0x0010_0000_0000_0000);
            e += 1022;
        }
        let factor = f64::from_bits(((e + 1023) as u64) << 52);
        res * factor
    }
}

#[cfg(feature = "f16")]
pub(crate) mod f16_math {
    use super::f32_math;
    use half::f16;

    #[inline(always)]
    pub(crate) fn sin(x: f16) -> f16 {
        f16::from_f32(f32_math::sin(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn cos(x: f16) -> f16 {
        f16::from_f32(f32_math::cos(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn sin_cos(x: f16) -> (f16, f16) {
        let (s, c) = f32_math::sin_cos(x.to_f32());
        (f16::from_f32(s), f16::from_f32(c))
    }

    #[inline(always)]
    pub(crate) fn tan(x: f16) -> f16 {
        f16::from_f32(f32_math::tan(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn asin(x: f16) -> f16 {
        f16::from_f32(f32_math::asin(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn acos(x: f16) -> f16 {
        f16::from_f32(f32_math::acos(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn atan(x: f16) -> f16 {
        f16::from_f32(f32_math::atan(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn atan2(y: f16, x: f16) -> f16 {
        f16::from_f32(f32_math::atan2(y.to_f32(), x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn sinh(x: f16) -> f16 {
        f16::from_f32(f32_math::sinh(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn cosh(x: f16) -> f16 {
        f16::from_f32(f32_math::cosh(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn tanh(x: f16) -> f16 {
        f16::from_f32(f32_math::tanh(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn asinh(x: f16) -> f16 {
        f16::from_f32(f32_math::asinh(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn acosh(x: f16) -> f16 {
        f16::from_f32(f32_math::acosh(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn atanh(x: f16) -> f16 {
        f16::from_f32(f32_math::atanh(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn exp(x: f16) -> f16 {
        f16::from_f32(f32_math::exp(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn exp2(x: f16) -> f16 {
        f16::from_f32(f32_math::exp2(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn ln(x: f16) -> f16 {
        f16::from_f32(f32_math::ln(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn log2(x: f16) -> f16 {
        f16::from_f32(f32_math::log2(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn powf(x: f16, y: f16) -> f16 {
        f16::from_f32(f32_math::powf(x.to_f32(), y.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn sqrt(x: f16) -> f16 {
        f16::from_f32(f32_math::sqrt(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn floor(x: f16) -> f16 {
        f16::from_f32(f32_math::floor(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn ceil(x: f16) -> f16 {
        f16::from_f32(f32_math::ceil(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn trunc(x: f16) -> f16 {
        f16::from_f32(f32_math::trunc(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn round(x: f16) -> f16 {
        f16::from_f32(f32_math::round(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn round_even(x: f16) -> f16 {
        f16::from_f32(f32_math::round_even(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn abs(x: f16) -> f16 {
        f16::from_bits(x.to_bits() & 0x7fff)
    }

    #[inline(always)]
    pub(crate) fn sign(x: f16) -> f16 {
        f16::from_f32(f32_math::sign(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn fract(x: f16) -> f16 {
        f16::from_f32(x.to_f32() - f32_math::floor(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn modulo(x: f16, y: f16) -> f16 {
        let xf = x.to_f32();
        let yf = y.to_f32();
        f16::from_f32(xf - yf * f32_math::floor(xf / yf))
    }

    #[inline(always)]
    pub(crate) fn inversesqrt(x: f16) -> f16 {
        f16::from_f32(1.0 / f32_math::sqrt(x.to_f32()))
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f16, b: f16, c: f16) -> f16 {
        f16::from_f64((a.to_f64() * b.to_f64()) + c.to_f64())
    }

    #[inline]
    pub(crate) fn frexp(x: f16) -> (f16, i32) {
        if !x.is_finite() || x.to_f32() == 0.0 {
            return (x, 0);
        }
        let (m, e) = f32_math::frexp(x.to_f32());
        (f16::from_f32(m), e)
    }

    #[inline]
    pub(crate) fn ldexp(x: f16, exp: i32) -> f16 {
        f16::from_f32(f32_math::ldexp(x.to_f32(), exp))
    }
}
