use crate::math::f32_math;

mod private {
    pub trait Sealed {}
}

pub trait SignedCommon: private::Sealed + Copy {
    fn abs_op(self) -> Self;
    fn sign_op(self) -> Self;
}

pub trait FloatCommon: SignedCommon {
    type Mask;
    fn floor_op(self) -> Self;
    fn trunc_op(self) -> Self;
    fn round_op(self) -> Self;
    fn round_even_op(self) -> Self;
    fn ceil_op(self) -> Self;
    fn fract_op(self) -> Self;
    fn modf_op(self) -> (Self, Self);
    fn fma_op(self, b: Self, c: Self) -> Self;
    fn isnan_op(self) -> Self::Mask;
    fn isinf_op(self) -> Self::Mask;
}

pub trait ModuloOp<Y, Out>: private::Sealed {
    fn modulo_op(self, y: Y) -> Out;
}

pub trait MinMaxOp<Y, Out>: private::Sealed {
    fn min_op(self, y: Y) -> Out;
    fn max_op(self, y: Y) -> Out;
}

pub trait ClampOp<Lo, Hi, Out>: private::Sealed {
    fn clamp_op(self, min_val: Lo, max_val: Hi) -> Out;
}

pub trait MixOp<Y, A, Out>: private::Sealed {
    fn mix_op(self, y: Y, a: A) -> Out;
}

pub trait StepOp<X, Out>: private::Sealed {
    fn step_op(self, x: X) -> Out;
}

pub trait SmoothstepOp<E1, X, Out>: private::Sealed {
    fn smoothstep_op(self, edge1: E1, x: X) -> Out;
}

pub trait FrexpLdexp: private::Sealed + Copy {
    type Exp;
    fn frexp_op(self) -> (Self, Self::Exp);
    fn ldexp_op(self, exp: Self::Exp) -> Self;
}

pub trait FloatBitsInt: private::Sealed + Copy {
    type IntRepr;
    type UintRepr;
    fn float_bits_to_int_op(self) -> Self::IntRepr;
    fn float_bits_to_uint_op(self) -> Self::UintRepr;
}

pub trait IntBitsFloat: private::Sealed + Copy {
    type FloatRepr;
    fn bits_to_float_op(self) -> Self::FloatRepr;
}

impl private::Sealed for bool {}
impl MixOp<bool, bool, bool> for bool {
    #[inline(always)]
    fn mix_op(self, y: bool, a: bool) -> bool {
        if a { y } else { self }
    }
}

impl private::Sealed for f32 {}
impl SignedCommon for f32 {
    #[inline(always)]
    fn abs_op(self) -> Self {
        f32_math::abs(self)
    }
    #[inline(always)]
    fn sign_op(self) -> Self {
        f32_math::sign(self)
    }
}

impl FloatCommon for f32 {
    type Mask = bool;
    #[inline(always)]
    fn floor_op(self) -> Self {
        f32_math::floor(self)
    }
    #[inline(always)]
    fn trunc_op(self) -> Self {
        f32_math::trunc(self)
    }
    #[inline(always)]
    fn round_op(self) -> Self {
        f32_math::round(self)
    }
    #[inline(always)]
    fn round_even_op(self) -> Self {
        f32_math::round_even(self)
    }
    #[inline(always)]
    fn ceil_op(self) -> Self {
        f32_math::ceil(self)
    }
    #[inline(always)]
    fn fract_op(self) -> Self {
        f32_math::fract(self)
    }
    #[inline(always)]
    fn modf_op(self) -> (Self, Self) {
        if self.is_infinite() {
            let zero = f32::from_bits(self.to_bits() & 0x8000_0000);
            return (zero, self);
        }
        let i = f32_math::trunc(self);
        let f = self - i;
        let f_signed = if f == 0.0 {
            f32::from_bits(self.to_bits() & 0x8000_0000)
        } else {
            f
        };
        (f_signed, i)
    }
    #[inline(always)]
    fn fma_op(self, b: Self, c: Self) -> Self {
        f32_math::mul_add(self, b, c)
    }
    #[inline(always)]
    fn isnan_op(self) -> bool {
        self.is_nan()
    }
    #[inline(always)]
    fn isinf_op(self) -> bool {
        self.is_infinite()
    }
}

impl ModuloOp<f32, f32> for f32 {
    #[inline(always)]
    fn modulo_op(self, y: f32) -> f32 {
        f32_math::modulo(self, y)
    }
}

impl MinMaxOp<f32, f32> for f32 {
    #[inline(always)]
    fn min_op(self, y: f32) -> f32 {
        self.min(y)
    }
    #[inline(always)]
    fn max_op(self, y: f32) -> f32 {
        self.max(y)
    }
}

impl ClampOp<f32, f32, f32> for f32 {
    #[inline(always)]
    fn clamp_op(self, min_val: f32, max_val: f32) -> f32 {
        self.max(min_val).min(max_val)
    }
}

impl MixOp<f32, f32, f32> for f32 {
    #[inline(always)]
    fn mix_op(self, y: f32, a: f32) -> f32 {
        self + (y - self) * a
    }
}

impl MixOp<f32, bool, f32> for f32 {
    #[inline(always)]
    fn mix_op(self, y: f32, a: bool) -> f32 {
        if a { y } else { self }
    }
}

impl StepOp<f32, f32> for f32 {
    #[inline(always)]
    fn step_op(self, x: f32) -> f32 {
        if x < self { 0.0 } else { 1.0 }
    }
}

impl SmoothstepOp<f32, f32, f32> for f32 {
    #[inline(always)]
    fn smoothstep_op(self, edge1: f32, x: f32) -> f32 {
        let t = ((x - self) / (edge1 - self)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl FrexpLdexp for f32 {
    type Exp = i32;
    #[inline]
    fn frexp_op(self) -> (Self, i32) {
        f32_math::frexp(self)
    }
    #[inline]
    fn ldexp_op(self, exp: i32) -> Self {
        f32_math::ldexp(self, exp)
    }
}

impl FloatBitsInt for f32 {
    type IntRepr = i32;
    type UintRepr = u32;
    #[inline(always)]
    fn float_bits_to_int_op(self) -> i32 {
        self.to_bits() as i32
    }
    #[inline(always)]
    fn float_bits_to_uint_op(self) -> u32 {
        self.to_bits()
    }
}

impl IntBitsFloat for i32 {
    type FloatRepr = f32;
    #[inline(always)]
    fn bits_to_float_op(self) -> f32 {
        f32::from_bits(self as u32)
    }
}

impl IntBitsFloat for u32 {
    type FloatRepr = f32;
    #[inline(always)]
    fn bits_to_float_op(self) -> f32 {
        f32::from_bits(self)
    }
}

#[cfg(feature = "f64")]
impl private::Sealed for f64 {}
#[cfg(feature = "f64")]
impl SignedCommon for f64 {
    #[inline(always)]
    fn abs_op(self) -> Self {
        crate::math::f64_math::abs(self)
    }
    #[inline(always)]
    fn sign_op(self) -> Self {
        crate::math::f64_math::sign(self)
    }
}

#[cfg(feature = "f64")]
impl FloatCommon for f64 {
    type Mask = bool;
    #[inline(always)]
    fn floor_op(self) -> Self {
        crate::math::f64_math::floor(self)
    }
    #[inline(always)]
    fn trunc_op(self) -> Self {
        crate::math::f64_math::trunc(self)
    }
    #[inline(always)]
    fn round_op(self) -> Self {
        crate::math::f64_math::round(self)
    }
    #[inline(always)]
    fn round_even_op(self) -> Self {
        crate::math::f64_math::round_even(self)
    }
    #[inline(always)]
    fn ceil_op(self) -> Self {
        crate::math::f64_math::ceil(self)
    }
    #[inline(always)]
    fn fract_op(self) -> Self {
        crate::math::f64_math::fract(self)
    }
    #[inline(always)]
    fn modf_op(self) -> (Self, Self) {
        if self.is_infinite() {
            let zero = f64::from_bits(self.to_bits() & 0x8000_0000_0000_0000);
            return (zero, self);
        }
        let i = crate::math::f64_math::trunc(self);
        let f = self - i;
        let f_signed = if f == 0.0 {
            f64::from_bits(self.to_bits() & 0x8000_0000_0000_0000)
        } else {
            f
        };
        (f_signed, i)
    }
    #[inline(always)]
    fn fma_op(self, b: Self, c: Self) -> Self {
        crate::math::f64_math::mul_add(self, b, c)
    }
    #[inline(always)]
    fn isnan_op(self) -> bool {
        self.is_nan()
    }
    #[inline(always)]
    fn isinf_op(self) -> bool {
        self.is_infinite()
    }
}

#[cfg(feature = "f64")]
impl ModuloOp<f64, f64> for f64 {
    #[inline(always)]
    fn modulo_op(self, y: f64) -> f64 {
        crate::math::f64_math::modulo(self, y)
    }
}

#[cfg(feature = "f64")]
impl MinMaxOp<f64, f64> for f64 {
    #[inline(always)]
    fn min_op(self, y: f64) -> f64 {
        self.min(y)
    }
    #[inline(always)]
    fn max_op(self, y: f64) -> f64 {
        self.max(y)
    }
}

#[cfg(feature = "f64")]
impl ClampOp<f64, f64, f64> for f64 {
    #[inline(always)]
    fn clamp_op(self, min_val: f64, max_val: f64) -> f64 {
        self.max(min_val).min(max_val)
    }
}

#[cfg(feature = "f64")]
impl MixOp<f64, f64, f64> for f64 {
    #[inline(always)]
    fn mix_op(self, y: f64, a: f64) -> f64 {
        self + (y - self) * a
    }
}

#[cfg(feature = "f64")]
impl MixOp<f64, bool, f64> for f64 {
    #[inline(always)]
    fn mix_op(self, y: f64, a: bool) -> f64 {
        if a { y } else { self }
    }
}

#[cfg(feature = "f64")]
impl StepOp<f64, f64> for f64 {
    #[inline(always)]
    fn step_op(self, x: f64) -> f64 {
        if x < self { 0.0 } else { 1.0 }
    }
}

#[cfg(feature = "f64")]
impl SmoothstepOp<f64, f64, f64> for f64 {
    #[inline(always)]
    fn smoothstep_op(self, edge1: f64, x: f64) -> f64 {
        let t = ((x - self) / (edge1 - self)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

#[cfg(feature = "f64")]
impl FrexpLdexp for f64 {
    type Exp = i32;
    #[inline]
    fn frexp_op(self) -> (Self, i32) {
        crate::math::f64_math::frexp(self)
    }
    #[inline]
    fn ldexp_op(self, exp: i32) -> Self {
        crate::math::f64_math::ldexp(self, exp)
    }
}

#[cfg(feature = "f16")]
impl private::Sealed for half::f16 {}
#[cfg(feature = "f16")]
impl SignedCommon for half::f16 {
    #[inline(always)]
    fn abs_op(self) -> Self {
        crate::math::f16_math::abs(self)
    }
    #[inline(always)]
    fn sign_op(self) -> Self {
        crate::math::f16_math::sign(self)
    }
}

#[cfg(feature = "f16")]
impl FloatCommon for half::f16 {
    type Mask = bool;
    #[inline(always)]
    fn floor_op(self) -> Self {
        crate::math::f16_math::floor(self)
    }
    #[inline(always)]
    fn trunc_op(self) -> Self {
        crate::math::f16_math::trunc(self)
    }
    #[inline(always)]
    fn round_op(self) -> Self {
        crate::math::f16_math::round(self)
    }
    #[inline(always)]
    fn round_even_op(self) -> Self {
        crate::math::f16_math::round_even(self)
    }
    #[inline(always)]
    fn ceil_op(self) -> Self {
        crate::math::f16_math::ceil(self)
    }
    #[inline(always)]
    fn fract_op(self) -> Self {
        crate::math::f16_math::fract(self)
    }
    #[inline(always)]
    fn modf_op(self) -> (Self, Self) {
        let (f, i) = self.to_f32().modf_op();
        (half::f16::from_f32(f), half::f16::from_f32(i))
    }
    #[inline(always)]
    fn fma_op(self, b: Self, c: Self) -> Self {
        crate::math::f16_math::mul_add(self, b, c)
    }
    #[inline(always)]
    fn isnan_op(self) -> bool {
        self.is_nan()
    }
    #[inline(always)]
    fn isinf_op(self) -> bool {
        self.is_infinite()
    }
}

#[cfg(feature = "f16")]
impl ModuloOp<half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn modulo_op(self, y: half::f16) -> half::f16 {
        crate::math::f16_math::modulo(self, y)
    }
}

#[cfg(feature = "f16")]
impl MinMaxOp<half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn min_op(self, y: half::f16) -> half::f16 {
        self.min(y)
    }
    #[inline(always)]
    fn max_op(self, y: half::f16) -> half::f16 {
        self.max(y)
    }
}

#[cfg(feature = "f16")]
impl ClampOp<half::f16, half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn clamp_op(self, min_val: half::f16, max_val: half::f16) -> half::f16 {
        self.max(min_val).min(max_val)
    }
}

#[cfg(feature = "f16")]
impl MixOp<half::f16, half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn mix_op(self, y: half::f16, a: half::f16) -> half::f16 {
        half::f16::from_f32(self.to_f32() + (y.to_f32() - self.to_f32()) * a.to_f32())
    }
}

#[cfg(feature = "f16")]
impl MixOp<half::f16, bool, half::f16> for half::f16 {
    #[inline(always)]
    fn mix_op(self, y: half::f16, a: bool) -> half::f16 {
        if a { y } else { self }
    }
}

#[cfg(feature = "f16")]
impl StepOp<half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn step_op(self, x: half::f16) -> half::f16 {
        if x < self {
            half::f16::ZERO
        } else {
            half::f16::ONE
        }
    }
}

#[cfg(feature = "f16")]
impl SmoothstepOp<half::f16, half::f16, half::f16> for half::f16 {
    #[inline(always)]
    fn smoothstep_op(self, edge1: half::f16, x: half::f16) -> half::f16 {
        half::f16::from_f32(self.to_f32().smoothstep_op(edge1.to_f32(), x.to_f32()))
    }
}

#[cfg(feature = "f16")]
impl FrexpLdexp for half::f16 {
    type Exp = i32;
    #[inline]
    fn frexp_op(self) -> (Self, i32) {
        crate::math::f16_math::frexp(self)
    }
    #[inline]
    fn ldexp_op(self, exp: i32) -> Self {
        crate::math::f16_math::ldexp(self, exp)
    }
}

macro_rules! impl_float_vec_common {
    ($vec_t:ty, $s:ty, $mask_t:ty, $n:literal, $zero:expr, $one:expr, $two:expr, $three:expr) => {
        impl private::Sealed for $vec_t {}

        impl SignedCommon for $vec_t {
            #[inline(always)]
            fn abs_op(self) -> Self {
                self.abs()
            }
            #[inline(always)]
            fn sign_op(self) -> Self {
                self.sign()
            }
        }

        impl FloatCommon for $vec_t {
            type Mask = $mask_t;

            #[inline(always)]
            fn floor_op(self) -> Self {
                self.floor()
            }
            #[inline(always)]
            fn trunc_op(self) -> Self {
                self.trunc()
            }
            #[inline(always)]
            fn round_op(self) -> Self {
                self.round()
            }
            #[inline(always)]
            fn round_even_op(self) -> Self {
                self.round_even()
            }
            #[inline(always)]
            fn ceil_op(self) -> Self {
                self.ceil()
            }
            #[inline(always)]
            fn fract_op(self) -> Self {
                self.fract()
            }
            #[inline]
            fn modf_op(self) -> (Self, Self) {
                let a = self.to_array();
                let mut f_arr = a;
                let mut i_arr = a;
                let mut k = 0;
                while k < $n {
                    let (fk, ik) = a[k].modf_op();
                    f_arr[k] = fk;
                    i_arr[k] = ik;
                    k += 1;
                }
                (Self::from_array(f_arr), Self::from_array(i_arr))
            }
            #[inline(always)]
            fn fma_op(self, b: Self, c: Self) -> Self {
                self.fma_exact(b, c)
            }
            #[inline(always)]
            fn isnan_op(self) -> $mask_t {
                let a = self.to_array();
                <$mask_t>::from_array(a.map(FloatCommon::isnan_op))
            }
            #[inline(always)]
            fn isinf_op(self) -> $mask_t {
                let a = self.to_array();
                <$mask_t>::from_array(a.map(FloatCommon::isinf_op))
            }
        }

        impl ModuloOp<$vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn modulo_op(self, y: $vec_t) -> $vec_t {
                self.modulo(y)
            }
        }

        impl ModuloOp<$s, $vec_t> for $vec_t {
            #[inline(always)]
            fn modulo_op(self, y: $s) -> $vec_t {
                self.modulo(Self::splat(y))
            }
        }

        impl MinMaxOp<$vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn min_op(self, y: $vec_t) -> $vec_t {
                self.min(y)
            }
            #[inline(always)]
            fn max_op(self, y: $vec_t) -> $vec_t {
                self.max(y)
            }
        }

        impl MinMaxOp<$s, $vec_t> for $vec_t {
            #[inline(always)]
            fn min_op(self, y: $s) -> $vec_t {
                self.min(Self::splat(y))
            }
            #[inline(always)]
            fn max_op(self, y: $s) -> $vec_t {
                self.max(Self::splat(y))
            }
        }

        impl ClampOp<$vec_t, $vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn clamp_op(self, min_val: $vec_t, max_val: $vec_t) -> $vec_t {
                self.clamp(min_val, max_val)
            }
        }

        impl ClampOp<$s, $s, $vec_t> for $vec_t {
            #[inline(always)]
            fn clamp_op(self, min_val: $s, max_val: $s) -> $vec_t {
                self.clamp(Self::splat(min_val), Self::splat(max_val))
            }
        }

        impl MixOp<$vec_t, $vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn mix_op(self, y: $vec_t, a: $vec_t) -> $vec_t {
                (y - self).mul_add(a, self)
            }
        }

        impl MixOp<$vec_t, $s, $vec_t> for $vec_t {
            #[inline(always)]
            fn mix_op(self, y: $vec_t, a: $s) -> $vec_t {
                (y - self).mul_add(Self::splat(a), self)
            }
        }

        impl MixOp<$vec_t, $mask_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn mix_op(self, y: $vec_t, a: $mask_t) -> $vec_t {
                Self::select_mask(a, y, self)
            }
        }

        impl StepOp<$vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn step_op(self, x: $vec_t) -> $vec_t {
                Self::select_mask(x.cmp_lt(self), Self::splat($zero), Self::splat($one))
            }
        }

        impl StepOp<$vec_t, $vec_t> for $s {
            #[inline(always)]
            fn step_op(self, x: $vec_t) -> $vec_t {
                <$vec_t>::splat(self).step_op(x)
            }
        }

        impl SmoothstepOp<$vec_t, $vec_t, $vec_t> for $vec_t {
            #[inline]
            fn smoothstep_op(self, edge1: $vec_t, x: $vec_t) -> $vec_t {
                let t = ((x - self) / (edge1 - self)).clamp(Self::splat($zero), Self::splat($one));
                (Self::splat($three) - (t * $two)) * (t * t)
            }
        }

        impl SmoothstepOp<$s, $vec_t, $vec_t> for $s {
            #[inline]
            fn smoothstep_op(self, edge1: $s, x: $vec_t) -> $vec_t {
                <$vec_t>::splat(self).smoothstep_op(<$vec_t>::splat(edge1), x)
            }
        }
    };
}

impl_float_vec_common!(crate::Vec2, f32, crate::BVec2, 2, 0.0, 1.0, 2.0, 3.0);
impl_float_vec_common!(crate::Vec3, f32, crate::BVec3, 3, 0.0, 1.0, 2.0, 3.0);
impl_float_vec_common!(crate::Vec3A, f32, crate::BVec3, 3, 0.0, 1.0, 2.0, 3.0);
impl_float_vec_common!(crate::Vec4, f32, crate::BVec4, 4, 0.0, 1.0, 2.0, 3.0);

impl MixOp<crate::Vec3A, crate::BVec3A, crate::Vec3A> for crate::Vec3A {
    #[inline(always)]
    fn mix_op(self, y: crate::Vec3A, a: crate::BVec3A) -> crate::Vec3A {
        crate::Vec3A::select_simd(a, y, self)
    }
}

impl MixOp<crate::Vec4, crate::BVec4A, crate::Vec4> for crate::Vec4 {
    #[inline(always)]
    fn mix_op(self, y: crate::Vec4, a: crate::BVec4A) -> crate::Vec4 {
        crate::Vec4::select_simd(a, y, self)
    }
}

#[cfg(feature = "f64")]
impl_float_vec_common!(crate::DVec2, f64, crate::BVec2, 2, 0.0, 1.0, 2.0, 3.0);
#[cfg(feature = "f64")]
impl_float_vec_common!(crate::DVec3, f64, crate::BVec3, 3, 0.0, 1.0, 2.0, 3.0);
#[cfg(feature = "f64")]
impl_float_vec_common!(crate::DVec4, f64, crate::BVec4, 4, 0.0, 1.0, 2.0, 3.0);

#[cfg(feature = "f16")]
impl_float_vec_common!(
    crate::F16Vec2,
    half::f16,
    crate::BVec2,
    2,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0),
    half::f16::from_f32_const(3.0)
);
#[cfg(feature = "f16")]
impl_float_vec_common!(
    crate::F16Vec3,
    half::f16,
    crate::BVec3,
    3,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0),
    half::f16::from_f32_const(3.0)
);
#[cfg(feature = "f16")]
impl_float_vec_common!(
    crate::F16Vec4,
    half::f16,
    crate::BVec4,
    4,
    half::f16::ZERO,
    half::f16::ONE,
    half::f16::from_f32_const(2.0),
    half::f16::from_f32_const(3.0)
);

#[cfg(feature = "i32")]
macro_rules! impl_frexp_ldexp_vec {
    ($vec_t:ty, $ivec_t:ty, $n:literal) => {
        impl FrexpLdexp for $vec_t {
            type Exp = $ivec_t;

            #[inline]
            fn frexp_op(self) -> (Self, $ivec_t) {
                let a = self.to_array();
                let mut m = a;
                let mut e = [0i32; $n];
                let mut i = 0;
                while i < $n {
                    let (mi, ei) = a[i].frexp_op();
                    m[i] = mi;
                    e[i] = ei;
                    i += 1;
                }
                (Self::from_array(m), <$ivec_t>::from_array(e))
            }

            #[inline]
            fn ldexp_op(self, exp: $ivec_t) -> Self {
                let a = self.to_array();
                let ea = exp.to_array();
                let mut out = a;
                let mut i = 0;
                while i < $n {
                    out[i] = a[i].ldexp_op(ea[i]);
                    i += 1;
                }
                Self::from_array(out)
            }
        }
    };
}

#[cfg(feature = "i32")]
impl_frexp_ldexp_vec!(crate::Vec2, crate::IVec2, 2);
#[cfg(feature = "i32")]
impl_frexp_ldexp_vec!(crate::Vec3, crate::IVec3, 3);
#[cfg(feature = "i32")]
impl_frexp_ldexp_vec!(crate::Vec3A, crate::IVec3, 3);
#[cfg(feature = "i32")]
impl_frexp_ldexp_vec!(crate::Vec4, crate::IVec4, 4);

#[cfg(all(feature = "f64", feature = "i32"))]
impl_frexp_ldexp_vec!(crate::DVec2, crate::IVec2, 2);
#[cfg(all(feature = "f64", feature = "i32"))]
impl_frexp_ldexp_vec!(crate::DVec3, crate::IVec3, 3);
#[cfg(all(feature = "f64", feature = "i32"))]
impl_frexp_ldexp_vec!(crate::DVec4, crate::IVec4, 4);

#[cfg(all(feature = "f16", feature = "i32"))]
impl_frexp_ldexp_vec!(crate::F16Vec2, crate::IVec2, 2);
#[cfg(all(feature = "f16", feature = "i32"))]
impl_frexp_ldexp_vec!(crate::F16Vec3, crate::IVec3, 3);
#[cfg(all(feature = "f16", feature = "i32"))]
impl_frexp_ldexp_vec!(crate::F16Vec4, crate::IVec4, 4);

#[cfg(all(feature = "i32", feature = "u32"))]
macro_rules! impl_float_bits_vec {
    ($fvec:ty, $ivec:ty, $uvec:ty, $n:literal) => {
        impl FloatBitsInt for $fvec {
            type IntRepr = $ivec;
            type UintRepr = $uvec;

            #[inline(always)]
            fn float_bits_to_int_op(self) -> $ivec {
                <$ivec>::from_array(self.to_array().map(|x| x.to_bits() as i32))
            }

            #[inline(always)]
            fn float_bits_to_uint_op(self) -> $uvec {
                <$uvec>::from_array(self.to_array().map(f32::to_bits))
            }
        }
    };
}

#[cfg(all(feature = "i32", feature = "u32"))]
impl_float_bits_vec!(crate::Vec2, crate::IVec2, crate::UVec2, 2);
#[cfg(all(feature = "i32", feature = "u32"))]
impl_float_bits_vec!(crate::Vec3, crate::IVec3, crate::UVec3, 3);
#[cfg(all(feature = "i32", feature = "u32"))]
impl_float_bits_vec!(crate::Vec3A, crate::IVec3, crate::UVec3, 3);
#[cfg(all(feature = "i32", feature = "u32"))]
impl_float_bits_vec!(crate::Vec4, crate::IVec4, crate::UVec4, 4);

#[cfg(feature = "i32")]
impl IntBitsFloat for crate::IVec2 {
    type FloatRepr = crate::Vec2;
    #[inline(always)]
    fn bits_to_float_op(self) -> crate::Vec2 {
        crate::Vec2::from_array(self.to_array().map(|x| f32::from_bits(x as u32)))
    }
}
#[cfg(feature = "i32")]
impl IntBitsFloat for crate::IVec3 {
    type FloatRepr = crate::Vec3;
    #[inline(always)]
    fn bits_to_float_op(self) -> crate::Vec3 {
        crate::Vec3::from_array(self.to_array().map(|x| f32::from_bits(x as u32)))
    }
}
#[cfg(feature = "i32")]
impl IntBitsFloat for crate::IVec4 {
    type FloatRepr = crate::Vec4;
    #[inline(always)]
    fn bits_to_float_op(self) -> crate::Vec4 {
        crate::Vec4::from_array(self.to_array().map(|x| f32::from_bits(x as u32)))
    }
}

#[cfg(feature = "u32")]
impl IntBitsFloat for crate::UVec2 {
    type FloatRepr = crate::Vec2;
    #[inline(always)]
    fn bits_to_float_op(self) -> crate::Vec2 {
        crate::Vec2::from_array(self.to_array().map(f32::from_bits))
    }
}
#[cfg(feature = "u32")]
impl IntBitsFloat for crate::UVec3 {
    type FloatRepr = crate::Vec3;
    #[inline(always)]
    fn bits_to_float_op(self) -> crate::Vec3 {
        crate::Vec3::from_array(self.to_array().map(f32::from_bits))
    }
}
#[cfg(feature = "u32")]
impl IntBitsFloat for crate::UVec4 {
    type FloatRepr = crate::Vec4;
    #[inline(always)]
    fn bits_to_float_op(self) -> crate::Vec4 {
        crate::Vec4::from_array(self.to_array().map(f32::from_bits))
    }
}

macro_rules! impl_signed_int_common {
    ($feat:literal, $s:ty, $v2:ty, $v3:ty, $v4:ty) => {
        #[cfg(feature = $feat)]
        impl private::Sealed for $s {}
        #[cfg(feature = $feat)]
        impl SignedCommon for $s {
            #[inline(always)]
            fn abs_op(self) -> Self {
                self.wrapping_abs()
            }
            #[inline(always)]
            fn sign_op(self) -> Self {
                if self > 0 {
                    1
                } else if self < 0 {
                    -1
                } else {
                    0
                }
            }
        }
        #[cfg(feature = $feat)]
        impl MinMaxOp<$s, $s> for $s {
            #[inline(always)]
            fn min_op(self, y: $s) -> $s {
                Ord::min(self, y)
            }
            #[inline(always)]
            fn max_op(self, y: $s) -> $s {
                Ord::max(self, y)
            }
        }
        #[cfg(feature = $feat)]
        impl ClampOp<$s, $s, $s> for $s {
            #[inline(always)]
            fn clamp_op(self, min_val: $s, max_val: $s) -> $s {
                Ord::min(Ord::max(self, min_val), max_val)
            }
        }
        #[cfg(feature = $feat)]
        impl MixOp<$s, bool, $s> for $s {
            #[inline(always)]
            fn mix_op(self, y: $s, a: bool) -> $s {
                if a { y } else { self }
            }
        }

        impl_signed_int_vec!($feat, $s, $v2, crate::BVec2);
        impl_signed_int_vec!($feat, $s, $v3, crate::BVec3);
        impl_signed_int_vec!($feat, $s, $v4, crate::BVec4);
    };
}

macro_rules! impl_signed_int_vec {
    ($feat:literal, $s:ty, $vec_t:ty, $mask_t:ty) => {
        #[cfg(feature = $feat)]
        impl private::Sealed for $vec_t {}
        #[cfg(feature = $feat)]
        impl SignedCommon for $vec_t {
            #[inline(always)]
            fn abs_op(self) -> Self {
                self.abs()
            }
            #[inline(always)]
            fn sign_op(self) -> Self {
                self.sign()
            }
        }
        #[cfg(feature = $feat)]
        impl MinMaxOp<$vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn min_op(self, y: $vec_t) -> $vec_t {
                self.min(y)
            }
            #[inline(always)]
            fn max_op(self, y: $vec_t) -> $vec_t {
                self.max(y)
            }
        }
        #[cfg(feature = $feat)]
        impl MinMaxOp<$s, $vec_t> for $vec_t {
            #[inline(always)]
            fn min_op(self, y: $s) -> $vec_t {
                self.min(Self::splat(y))
            }
            #[inline(always)]
            fn max_op(self, y: $s) -> $vec_t {
                self.max(Self::splat(y))
            }
        }
        #[cfg(feature = $feat)]
        impl ClampOp<$vec_t, $vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn clamp_op(self, min_val: $vec_t, max_val: $vec_t) -> $vec_t {
                self.clamp(min_val, max_val)
            }
        }
        #[cfg(feature = $feat)]
        impl ClampOp<$s, $s, $vec_t> for $vec_t {
            #[inline(always)]
            fn clamp_op(self, min_val: $s, max_val: $s) -> $vec_t {
                self.clamp(Self::splat(min_val), Self::splat(max_val))
            }
        }
        #[cfg(feature = $feat)]
        impl MixOp<$vec_t, $mask_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn mix_op(self, y: $vec_t, a: $mask_t) -> $vec_t {
                Self::select_mask(a, y, self)
            }
        }
    };
}

macro_rules! impl_unsigned_int_common {
    ($feat:literal, $s:ty, $v2:ty, $v3:ty, $v4:ty) => {
        #[cfg(feature = $feat)]
        impl private::Sealed for $s {}
        #[cfg(feature = $feat)]
        impl MinMaxOp<$s, $s> for $s {
            #[inline(always)]
            fn min_op(self, y: $s) -> $s {
                Ord::min(self, y)
            }
            #[inline(always)]
            fn max_op(self, y: $s) -> $s {
                Ord::max(self, y)
            }
        }
        #[cfg(feature = $feat)]
        impl ClampOp<$s, $s, $s> for $s {
            #[inline(always)]
            fn clamp_op(self, min_val: $s, max_val: $s) -> $s {
                Ord::min(Ord::max(self, min_val), max_val)
            }
        }
        #[cfg(feature = $feat)]
        impl MixOp<$s, bool, $s> for $s {
            #[inline(always)]
            fn mix_op(self, y: $s, a: bool) -> $s {
                if a { y } else { self }
            }
        }

        impl_unsigned_int_vec!($feat, $s, $v2, crate::BVec2);
        impl_unsigned_int_vec!($feat, $s, $v3, crate::BVec3);
        impl_unsigned_int_vec!($feat, $s, $v4, crate::BVec4);
    };
}

macro_rules! impl_unsigned_int_vec {
    ($feat:literal, $s:ty, $vec_t:ty, $mask_t:ty) => {
        #[cfg(feature = $feat)]
        impl private::Sealed for $vec_t {}
        #[cfg(feature = $feat)]
        impl MinMaxOp<$vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn min_op(self, y: $vec_t) -> $vec_t {
                self.min(y)
            }
            #[inline(always)]
            fn max_op(self, y: $vec_t) -> $vec_t {
                self.max(y)
            }
        }
        #[cfg(feature = $feat)]
        impl MinMaxOp<$s, $vec_t> for $vec_t {
            #[inline(always)]
            fn min_op(self, y: $s) -> $vec_t {
                self.min(Self::splat(y))
            }
            #[inline(always)]
            fn max_op(self, y: $s) -> $vec_t {
                self.max(Self::splat(y))
            }
        }
        #[cfg(feature = $feat)]
        impl ClampOp<$vec_t, $vec_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn clamp_op(self, min_val: $vec_t, max_val: $vec_t) -> $vec_t {
                self.clamp(min_val, max_val)
            }
        }
        #[cfg(feature = $feat)]
        impl ClampOp<$s, $s, $vec_t> for $vec_t {
            #[inline(always)]
            fn clamp_op(self, min_val: $s, max_val: $s) -> $vec_t {
                self.clamp(Self::splat(min_val), Self::splat(max_val))
            }
        }
        #[cfg(feature = $feat)]
        impl MixOp<$vec_t, $mask_t, $vec_t> for $vec_t {
            #[inline(always)]
            fn mix_op(self, y: $vec_t, a: $mask_t) -> $vec_t {
                Self::select_mask(a, y, self)
            }
        }
    };
}

impl_signed_int_common!("i8", i8, crate::I8Vec2, crate::I8Vec3, crate::I8Vec4);
impl_signed_int_common!("i16", i16, crate::I16Vec2, crate::I16Vec3, crate::I16Vec4);
impl_signed_int_common!("i32", i32, crate::IVec2, crate::IVec3, crate::IVec4);
impl_signed_int_common!("i64", i64, crate::I64Vec2, crate::I64Vec3, crate::I64Vec4);

impl_unsigned_int_common!("u8", u8, crate::U8Vec2, crate::U8Vec3, crate::U8Vec4);
impl_unsigned_int_common!("u16", u16, crate::U16Vec2, crate::U16Vec3, crate::U16Vec4);
impl_unsigned_int_common!("u32", u32, crate::UVec2, crate::UVec3, crate::UVec4);
impl_unsigned_int_common!("u64", u64, crate::U64Vec2, crate::U64Vec3, crate::U64Vec4);

macro_rules! impl_bvec_mix {
    ($bvec_t:ty, $n:literal) => {
        impl private::Sealed for $bvec_t {}
        impl MixOp<$bvec_t, $bvec_t, $bvec_t> for $bvec_t {
            #[inline(always)]
            fn mix_op(self, y: $bvec_t, a: $bvec_t) -> $bvec_t {
                (y & a) | (self & !a)
            }
        }
    };
}

impl_bvec_mix!(crate::BVec2, 2);
impl_bvec_mix!(crate::BVec3, 3);
impl_bvec_mix!(crate::BVec3A, 3);
impl_bvec_mix!(crate::BVec4, 4);
impl_bvec_mix!(crate::BVec4A, 4);

/// Returns `|x|` component-wise (wrapping for signed integer `MIN`).
#[inline(always)]
#[must_use]
pub fn abs<T: SignedCommon>(x: T) -> T {
    x.abs_op()
}

/// Returns `1` if `x > 0`, `-1` if `x < 0`, and `+0` otherwise (including `-0.0` and `NaN`, matching GLSL/GLM).
#[inline(always)]
#[must_use]
pub fn sign<T: SignedCommon>(x: T) -> T {
    x.sign_op()
}

/// Returns the largest integer value less than or equal to `x` component-wise.
#[inline(always)]
#[must_use]
pub fn floor<T: FloatCommon>(x: T) -> T {
    x.floor_op()
}

/// Rounds `x` toward zero to the nearest integer component-wise.
#[inline(always)]
#[must_use]
pub fn trunc<T: FloatCommon>(x: T) -> T {
    x.trunc_op()
}

/// Rounds `x` to the nearest integer component-wise (halfway cases away from zero).
#[inline(always)]
#[must_use]
pub fn round<T: FloatCommon>(x: T) -> T {
    x.round_op()
}

/// Rounds `x` to the nearest integer component-wise with halfway cases rounded to the nearest even integer (`roundEven`).
#[inline(always)]
#[must_use]
pub fn round_even<T: FloatCommon>(x: T) -> T {
    x.round_even_op()
}

/// Returns the smallest integer value greater than or equal to `x` component-wise.
#[inline(always)]
#[must_use]
pub fn ceil<T: FloatCommon>(x: T) -> T {
    x.ceil_op()
}

/// Computes the GLSL fractional part `x - floor(x)` component-wise (always non-negative for finite `x`).
#[inline(always)]
#[must_use]
pub fn fract<T: FloatCommon>(x: T) -> T {
    x.fract_op()
}

/// Computes the GLSL floored modulo `x - y * floor(x / y)` component-wise.
#[inline(always)]
#[must_use]
pub fn modulo<X, Y>(x: X, y: Y) -> X
where
    X: ModuloOp<Y, X>,
{
    x.modulo_op(y)
}

/// Alias of [`modulo`] using Rust raw-identifier syntax for exact GLSL `mod(x, y)` naming.
#[inline(always)]
#[must_use]
pub fn r#mod<X, Y>(x: X, y: Y) -> X
where
    X: ModuloOp<Y, X>,
{
    x.modulo_op(y)
}

/// Splits `x` into `(fractional_part, integer_part)` component-wise where both parts carry the sign of `x` (`modf`).
#[inline]
#[must_use]
pub fn modf<T: FloatCommon>(x: T) -> (T, T) {
    x.modf_op()
}

/// Returns the component-wise minimum of `x` and `y` (supporting vector-scalar broadcasting).
#[inline(always)]
#[must_use]
pub fn min<X, Y>(x: X, y: Y) -> X
where
    X: MinMaxOp<Y, X>,
{
    x.min_op(y)
}

/// Returns the component-wise maximum of `x` and `y` (supporting vector-scalar broadcasting).
#[inline(always)]
#[must_use]
pub fn max<X, Y>(x: X, y: Y) -> X
where
    X: MinMaxOp<Y, X>,
{
    x.max_op(y)
}

/// Constrains `x` to lie between `min_val` and `max_val` component-wise.
#[inline(always)]
#[must_use]
pub fn clamp<X, Lo, Hi>(x: X, min_val: Lo, max_val: Hi) -> X
where
    X: ClampOp<Lo, Hi, X>,
{
    x.clamp_op(min_val, max_val)
}

/// Linearly interpolates `x + a * (y - x)` for floating-point `a`, or selects `y` where `a` is `true` for boolean masks.
#[inline(always)]
#[must_use]
pub fn mix<X, Y, A>(x: X, y: Y, a: A) -> X
where
    X: MixOp<Y, A, X>,
{
    x.mix_op(y, a)
}

/// Generates a step function returning `0.0` where `x < edge` and `1.0` otherwise.
#[inline(always)]
#[must_use]
pub fn step<E, X>(edge: E, x: X) -> X
where
    E: StepOp<X, X>,
{
    edge.step_op(x)
}

/// Performs Hermite interpolation `3t^2 - 2t^3` for `t = clamp((x - edge0) / (edge1 - edge0), 0, 1)`.
#[inline]
#[must_use]
pub fn smoothstep<E0, E1, X>(edge0: E0, edge1: E1, x: X) -> X
where
    E0: SmoothstepOp<E1, X, X>,
{
    edge0.smoothstep_op(edge1, x)
}

/// Tests component-wise whether `x` is IEEE-754 `NaN`.
#[inline(always)]
#[must_use]
pub fn isnan<T: FloatCommon>(x: T) -> T::Mask {
    x.isnan_op()
}

/// Tests component-wise whether `x` is positive or negative infinity.
#[inline(always)]
#[must_use]
pub fn isinf<T: FloatCommon>(x: T) -> T::Mask {
    x.isinf_op()
}

/// Reinterprets the IEEE-754 bit representation of single-precision `x` as signed 32-bit integer(s) (`floatBitsToInt`).
#[inline(always)]
#[must_use]
pub fn float_bits_to_int<T: FloatBitsInt>(x: T) -> T::IntRepr {
    x.float_bits_to_int_op()
}

/// Reinterprets the IEEE-754 bit representation of single-precision `x` as unsigned 32-bit integer(s) (`floatBitsToUint`).
#[inline(always)]
#[must_use]
pub fn float_bits_to_uint<T: FloatBitsInt>(x: T) -> T::UintRepr {
    x.float_bits_to_uint_op()
}

/// Reinterprets the bit representation of signed 32-bit integer(s) `x` as single-precision float(s) (`intBitsToFloat`).
#[inline(always)]
#[must_use]
pub fn int_bits_to_float<T: IntBitsFloat>(x: T) -> T::FloatRepr {
    x.bits_to_float_op()
}

/// Reinterprets the bit representation of unsigned 32-bit integer(s) `x` as single-precision float(s) (`uintBitsToFloat`).
#[inline(always)]
#[must_use]
pub fn uint_bits_to_float<T: IntBitsFloat>(x: T) -> T::FloatRepr {
    x.bits_to_float_op()
}

/// Computes the IEEE-754 single-rounding fused multiply-add `a * b + c` component-wise.
#[inline(always)]
#[must_use]
pub fn fma<T: FloatCommon>(a: T, b: T, c: T) -> T {
    a.fma_op(b, c)
}

/// Splits floating-point `x` into a normalized significand in `[0.5, 1.0)` (or `0`) and base-2 integer exponent (`frexp`).
#[inline]
#[must_use]
pub fn frexp<T: FrexpLdexp>(x: T) -> (T, T::Exp) {
    x.frexp_op()
}

/// Builds a floating-point number from `x` and base-2 integer exponent `exp` (`x * 2^exp`, `ldexp`).
#[inline]
#[must_use]
pub fn ldexp<T: FrexpLdexp>(x: T, exp: T::Exp) -> T {
    x.ldexp_op(exp)
}
