mod private {
    pub trait Sealed {}
}

pub trait UAddCarryOp: private::Sealed + Copy {
    fn uadd_carry_op(self, y: Self) -> (Self, Self);
    fn usub_borrow_op(self, y: Self) -> (Self, Self);
    fn umul_extended_op(self, y: Self) -> (Self, Self);
}

pub trait IMulExtendedOp: private::Sealed + Copy {
    fn imul_extended_op(self, y: Self) -> (Self, Self);
}

pub trait BitfieldOp: private::Sealed + Copy {
    fn bitfield_extract_op(self, offset: i32, bits: i32) -> Self;
    fn bitfield_insert_op(self, insert: Self, offset: i32, bits: i32) -> Self;
    fn bitfield_reverse_op(self) -> Self;
}

pub trait BitScanOp: private::Sealed + Copy {
    type IntOut;

    fn bit_count_op(self) -> Self::IntOut;
    fn find_lsb_op(self) -> Self::IntOut;
    fn find_msb_op(self) -> Self::IntOut;
}

#[inline(always)]
const fn u32_add_carry(x: u32, y: u32) -> (u32, u32) {
    let (sum, carry) = x.overflowing_add(y);
    (sum, carry as u32)
}

#[inline(always)]
const fn u32_sub_borrow(x: u32, y: u32) -> (u32, u32) {
    let (diff, borrow) = x.overflowing_sub(y);
    (diff, borrow as u32)
}

#[inline(always)]
const fn u32_mul_ext(x: u32, y: u32) -> (u32, u32) {
    let prod = (x as u64) * (y as u64);
    ((prod >> 32) as u32, prod as u32)
}

#[inline(always)]
const fn i32_mul_ext(x: i32, y: i32) -> (i32, i32) {
    let prod = (x as i64) * (y as i64);
    ((prod >> 32) as i32, prod as i32)
}

macro_rules! impl_unsigned_bitfield_scalar {
    ($u:ty, $width:literal) => {
        impl private::Sealed for $u {}

        impl BitfieldOp for $u {
            #[inline(always)]
            fn bitfield_extract_op(self, offset: i32, bits: i32) -> Self {
                if bits <= 0 || !(0..$width).contains(&offset) {
                    return 0;
                }
                let off = offset as u32;
                let b = (bits as u32).min(($width as u32) - off);
                if b == $width {
                    self
                } else {
                    (self >> off) & (((1 as $u) << b) - 1)
                }
            }

            #[inline(always)]
            fn bitfield_insert_op(self, insert: Self, offset: i32, bits: i32) -> Self {
                if bits <= 0 || !(0..$width).contains(&offset) {
                    return self;
                }
                let off = offset as u32;
                let b = (bits as u32).min(($width as u32) - off);
                if b == $width {
                    insert
                } else {
                    let mask = (((1 as $u) << b) - 1) << off;
                    (self & !mask) | ((insert << off) & mask)
                }
            }

            #[inline(always)]
            fn bitfield_reverse_op(self) -> Self {
                self.reverse_bits()
            }
        }

        impl BitScanOp for $u {
            type IntOut = i32;

            #[inline(always)]
            fn bit_count_op(self) -> i32 {
                self.count_ones() as i32
            }

            #[inline(always)]
            fn find_lsb_op(self) -> i32 {
                if self == 0 {
                    -1
                } else {
                    self.trailing_zeros() as i32
                }
            }

            #[inline(always)]
            fn find_msb_op(self) -> i32 {
                if self == 0 {
                    -1
                } else {
                    ($width - 1) - (self.leading_zeros() as i32)
                }
            }
        }
    };
}

macro_rules! impl_signed_bitfield_scalar {
    ($i:ty, $u:ty, $width:literal) => {
        impl private::Sealed for $i {}

        impl BitfieldOp for $i {
            #[inline(always)]
            fn bitfield_extract_op(self, offset: i32, bits: i32) -> Self {
                if bits <= 0 || !(0..$width).contains(&offset) {
                    return 0;
                }
                let off = offset as u32;
                let b = (bits as u32).min(($width as u32) - off);
                let lshift = ($width as u32) - off - b;
                let rshift = ($width as u32) - b;
                (self << lshift) >> rshift
            }

            #[inline(always)]
            fn bitfield_insert_op(self, insert: Self, offset: i32, bits: i32) -> Self {
                ((self as $u).bitfield_insert_op(insert as $u, offset, bits)) as $i
            }

            #[inline(always)]
            fn bitfield_reverse_op(self) -> Self {
                self.reverse_bits()
            }
        }

        impl BitScanOp for $i {
            type IntOut = i32;

            #[inline(always)]
            fn bit_count_op(self) -> i32 {
                self.count_ones() as i32
            }

            #[inline(always)]
            fn find_lsb_op(self) -> i32 {
                if self == 0 {
                    -1
                } else {
                    self.trailing_zeros() as i32
                }
            }

            #[inline(always)]
            fn find_msb_op(self) -> i32 {
                let u = if self < 0 { !self as $u } else { self as $u };
                ($width - 1) - (u.leading_zeros() as i32)
            }
        }
    };
}

impl_unsigned_bitfield_scalar!(u8, 8);
impl_unsigned_bitfield_scalar!(u16, 16);
impl_unsigned_bitfield_scalar!(u32, 32);
impl_unsigned_bitfield_scalar!(u64, 64);

impl_signed_bitfield_scalar!(i8, u8, 8);
impl_signed_bitfield_scalar!(i16, u16, 16);
impl_signed_bitfield_scalar!(i32, u32, 32);
impl_signed_bitfield_scalar!(i64, u64, 64);

impl UAddCarryOp for u32 {
    #[inline(always)]
    fn uadd_carry_op(self, y: Self) -> (Self, Self) {
        u32_add_carry(self, y)
    }

    #[inline(always)]
    fn usub_borrow_op(self, y: Self) -> (Self, Self) {
        u32_sub_borrow(self, y)
    }

    #[inline(always)]
    fn umul_extended_op(self, y: Self) -> (Self, Self) {
        u32_mul_ext(self, y)
    }
}

impl IMulExtendedOp for i32 {
    #[inline(always)]
    fn imul_extended_op(self, y: Self) -> (Self, Self) {
        i32_mul_ext(self, y)
    }
}

macro_rules! impl_bitfield_vec {
    ($vec_t:ty, $n:literal) => {
        impl private::Sealed for $vec_t {}

        impl BitfieldOp for $vec_t {
            #[inline(always)]
            fn bitfield_extract_op(self, offset: i32, bits: i32) -> Self {
                Self::from_array(self.to_array().map(|v| v.bitfield_extract_op(offset, bits)))
            }

            #[inline(always)]
            fn bitfield_insert_op(self, insert: Self, offset: i32, bits: i32) -> Self {
                let b = self.to_array();
                let ins = insert.to_array();
                let mut out = b;
                let mut k = 0;
                while k < $n {
                    out[k] = b[k].bitfield_insert_op(ins[k], offset, bits);
                    k += 1;
                }
                Self::from_array(out)
            }

            #[inline(always)]
            fn bitfield_reverse_op(self) -> Self {
                Self::from_array(self.to_array().map(BitfieldOp::bitfield_reverse_op))
            }
        }
    };
}

#[cfg(feature = "i32")]
macro_rules! impl_bitscan_vec {
    ($vec_t:ty => $ivec_t:ty) => {
        impl BitScanOp for $vec_t {
            type IntOut = $ivec_t;

            #[inline(always)]
            fn bit_count_op(self) -> $ivec_t {
                <$ivec_t>::from_array(self.to_array().map(BitScanOp::bit_count_op))
            }

            #[inline(always)]
            fn find_lsb_op(self) -> $ivec_t {
                <$ivec_t>::from_array(self.to_array().map(BitScanOp::find_lsb_op))
            }

            #[inline(always)]
            fn find_msb_op(self) -> $ivec_t {
                <$ivec_t>::from_array(self.to_array().map(BitScanOp::find_msb_op))
            }
        }
    };
}

#[cfg(feature = "i8")]
impl_bitfield_vec!(crate::I8Vec2, 2);
#[cfg(feature = "i8")]
impl_bitfield_vec!(crate::I8Vec3, 3);
#[cfg(feature = "i8")]
impl_bitfield_vec!(crate::I8Vec4, 4);

#[cfg(feature = "u8")]
impl_bitfield_vec!(crate::U8Vec2, 2);
#[cfg(feature = "u8")]
impl_bitfield_vec!(crate::U8Vec3, 3);
#[cfg(feature = "u8")]
impl_bitfield_vec!(crate::U8Vec4, 4);

#[cfg(feature = "i16")]
impl_bitfield_vec!(crate::I16Vec2, 2);
#[cfg(feature = "i16")]
impl_bitfield_vec!(crate::I16Vec3, 3);
#[cfg(feature = "i16")]
impl_bitfield_vec!(crate::I16Vec4, 4);

#[cfg(feature = "u16")]
impl_bitfield_vec!(crate::U16Vec2, 2);
#[cfg(feature = "u16")]
impl_bitfield_vec!(crate::U16Vec3, 3);
#[cfg(feature = "u16")]
impl_bitfield_vec!(crate::U16Vec4, 4);

#[cfg(feature = "i32")]
impl_bitfield_vec!(crate::IVec2, 2);
#[cfg(feature = "i32")]
impl_bitfield_vec!(crate::IVec3, 3);
#[cfg(feature = "i32")]
impl_bitfield_vec!(crate::IVec4, 4);

#[cfg(feature = "u32")]
impl_bitfield_vec!(crate::UVec2, 2);
#[cfg(feature = "u32")]
impl_bitfield_vec!(crate::UVec3, 3);
#[cfg(feature = "u32")]
impl_bitfield_vec!(crate::UVec4, 4);

#[cfg(feature = "i64")]
impl_bitfield_vec!(crate::I64Vec2, 2);
#[cfg(feature = "i64")]
impl_bitfield_vec!(crate::I64Vec3, 3);
#[cfg(feature = "i64")]
impl_bitfield_vec!(crate::I64Vec4, 4);

#[cfg(feature = "u64")]
impl_bitfield_vec!(crate::U64Vec2, 2);
#[cfg(feature = "u64")]
impl_bitfield_vec!(crate::U64Vec3, 3);
#[cfg(feature = "u64")]
impl_bitfield_vec!(crate::U64Vec4, 4);

#[cfg(all(feature = "i8", feature = "i32"))]
impl_bitscan_vec!(crate::I8Vec2 => crate::IVec2);
#[cfg(all(feature = "i8", feature = "i32"))]
impl_bitscan_vec!(crate::I8Vec3 => crate::IVec3);
#[cfg(all(feature = "i8", feature = "i32"))]
impl_bitscan_vec!(crate::I8Vec4 => crate::IVec4);

#[cfg(all(feature = "u8", feature = "i32"))]
impl_bitscan_vec!(crate::U8Vec2 => crate::IVec2);
#[cfg(all(feature = "u8", feature = "i32"))]
impl_bitscan_vec!(crate::U8Vec3 => crate::IVec3);
#[cfg(all(feature = "u8", feature = "i32"))]
impl_bitscan_vec!(crate::U8Vec4 => crate::IVec4);

#[cfg(all(feature = "i16", feature = "i32"))]
impl_bitscan_vec!(crate::I16Vec2 => crate::IVec2);
#[cfg(all(feature = "i16", feature = "i32"))]
impl_bitscan_vec!(crate::I16Vec3 => crate::IVec3);
#[cfg(all(feature = "i16", feature = "i32"))]
impl_bitscan_vec!(crate::I16Vec4 => crate::IVec4);

#[cfg(all(feature = "u16", feature = "i32"))]
impl_bitscan_vec!(crate::U16Vec2 => crate::IVec2);
#[cfg(all(feature = "u16", feature = "i32"))]
impl_bitscan_vec!(crate::U16Vec3 => crate::IVec3);
#[cfg(all(feature = "u16", feature = "i32"))]
impl_bitscan_vec!(crate::U16Vec4 => crate::IVec4);

#[cfg(feature = "i32")]
impl_bitscan_vec!(crate::IVec2 => crate::IVec2);
#[cfg(feature = "i32")]
impl_bitscan_vec!(crate::IVec3 => crate::IVec3);
#[cfg(feature = "i32")]
impl_bitscan_vec!(crate::IVec4 => crate::IVec4);

#[cfg(all(feature = "u32", feature = "i32"))]
impl_bitscan_vec!(crate::UVec2 => crate::IVec2);
#[cfg(all(feature = "u32", feature = "i32"))]
impl_bitscan_vec!(crate::UVec3 => crate::IVec3);
#[cfg(all(feature = "u32", feature = "i32"))]
impl_bitscan_vec!(crate::UVec4 => crate::IVec4);

#[cfg(all(feature = "i64", feature = "i32"))]
impl_bitscan_vec!(crate::I64Vec2 => crate::IVec2);
#[cfg(all(feature = "i64", feature = "i32"))]
impl_bitscan_vec!(crate::I64Vec3 => crate::IVec3);
#[cfg(all(feature = "i64", feature = "i32"))]
impl_bitscan_vec!(crate::I64Vec4 => crate::IVec4);

#[cfg(all(feature = "u64", feature = "i32"))]
impl_bitscan_vec!(crate::U64Vec2 => crate::IVec2);
#[cfg(all(feature = "u64", feature = "i32"))]
impl_bitscan_vec!(crate::U64Vec3 => crate::IVec3);
#[cfg(all(feature = "u64", feature = "i32"))]
impl_bitscan_vec!(crate::U64Vec4 => crate::IVec4);

#[cfg(feature = "u32")]
macro_rules! impl_u32_ext_vec {
    ($uvec:ty, $n:literal) => {
        impl UAddCarryOp for $uvec {
            #[inline(always)]
            fn uadd_carry_op(self, y: Self) -> (Self, Self) {
                let a = self.to_array();
                let b = y.to_array();
                let mut sum = [0u32; $n];
                let mut carry = [0u32; $n];
                let mut k = 0;
                while k < $n {
                    let (s, c) = u32_add_carry(a[k], b[k]);
                    sum[k] = s;
                    carry[k] = c;
                    k += 1;
                }
                (Self::from_array(sum), Self::from_array(carry))
            }

            #[inline(always)]
            fn usub_borrow_op(self, y: Self) -> (Self, Self) {
                let a = self.to_array();
                let b = y.to_array();
                let mut diff = [0u32; $n];
                let mut borrow = [0u32; $n];
                let mut k = 0;
                while k < $n {
                    let (d, br) = u32_sub_borrow(a[k], b[k]);
                    diff[k] = d;
                    borrow[k] = br;
                    k += 1;
                }
                (Self::from_array(diff), Self::from_array(borrow))
            }

            #[inline(always)]
            fn umul_extended_op(self, y: Self) -> (Self, Self) {
                let a = self.to_array();
                let b = y.to_array();
                let mut msb = [0u32; $n];
                let mut lsb = [0u32; $n];
                let mut k = 0;
                while k < $n {
                    let (hi, lo) = u32_mul_ext(a[k], b[k]);
                    msb[k] = hi;
                    lsb[k] = lo;
                    k += 1;
                }
                (Self::from_array(msb), Self::from_array(lsb))
            }
        }
    };
}

#[cfg(feature = "u32")]
impl_u32_ext_vec!(crate::UVec2, 2);
#[cfg(feature = "u32")]
impl_u32_ext_vec!(crate::UVec3, 3);
#[cfg(feature = "u32")]
impl_u32_ext_vec!(crate::UVec4, 4);

#[cfg(feature = "i32")]
macro_rules! impl_i32_ext_vec {
    ($ivec:ty, $n:literal) => {
        impl IMulExtendedOp for $ivec {
            #[inline(always)]
            fn imul_extended_op(self, y: Self) -> (Self, Self) {
                let a = self.to_array();
                let b = y.to_array();
                let mut msb = [0i32; $n];
                let mut lsb = [0i32; $n];
                let mut k = 0;
                while k < $n {
                    let (hi, lo) = i32_mul_ext(a[k], b[k]);
                    msb[k] = hi;
                    lsb[k] = lo;
                    k += 1;
                }
                (Self::from_array(msb), Self::from_array(lsb))
            }
        }
    };
}

#[cfg(feature = "i32")]
impl_i32_ext_vec!(crate::IVec2, 2);
#[cfg(feature = "i32")]
impl_i32_ext_vec!(crate::IVec3, 3);
#[cfg(feature = "i32")]
impl_i32_ext_vec!(crate::IVec4, 4);

/// Adds 32-bit unsigned integers `x` and `y`, returning `(sum, carry)` where `carry` is `0` or `1` (`uaddCarry`).
#[inline(always)]
#[must_use]
pub fn uadd_carry<T: UAddCarryOp>(x: T, y: T) -> (T, T) {
    x.uadd_carry_op(y)
}

/// Subtracts 32-bit unsigned integer `y` from `x`, returning `(diff, borrow)` where `borrow` is `0` if `x >= y` else `1` (`usubBorrow`).
#[inline(always)]
#[must_use]
pub fn usub_borrow<T: UAddCarryOp>(x: T, y: T) -> (T, T) {
    x.usub_borrow_op(y)
}

/// Multiplies 32-bit unsigned integers `x` and `y` to a 64-bit product, returning `(msb, lsb)` (`umulExtended`).
#[inline(always)]
#[must_use]
pub fn umul_extended<T: UAddCarryOp>(x: T, y: T) -> (T, T) {
    x.umul_extended_op(y)
}

/// Multiplies 32-bit signed integers `x` and `y` to a 64-bit product, returning `(msb, lsb)` (`imulExtended`).
#[inline(always)]
#[must_use]
pub fn imul_extended<T: IMulExtendedOp>(x: T, y: T) -> (T, T) {
    x.imul_extended_op(y)
}

/// Extracts `bits` starting at `offset` from `value`, zero-extending unsigned types and sign-extending signed types (`bitfieldExtract`).
#[inline(always)]
#[must_use]
pub fn bitfield_extract<T: BitfieldOp>(value: T, offset: i32, bits: i32) -> T {
    value.bitfield_extract_op(offset, bits)
}

/// Inserts the `bits` least-significant bits of `insert` into `base` starting at `offset` (`bitfieldInsert`).
#[inline(always)]
#[must_use]
pub fn bitfield_insert<T: BitfieldOp>(base: T, insert: T, offset: i32, bits: i32) -> T {
    base.bitfield_insert_op(insert, offset, bits)
}

/// Reverses the bit order of `value` (`bitfieldReverse`).
#[inline(always)]
#[must_use]
pub fn bitfield_reverse<T: BitfieldOp>(value: T) -> T {
    value.bitfield_reverse_op()
}

/// Counts the number of `1` bits in `value` (`bitCount`).
#[inline(always)]
#[must_use]
pub fn bit_count<T: BitScanOp>(value: T) -> T::IntOut {
    value.bit_count_op()
}

/// Returns the bit number of the least-significant `1` bit of `value`, or `-1` if `value == 0` (`findLSB`).
#[inline(always)]
#[must_use]
pub fn find_lsb<T: BitScanOp>(value: T) -> T::IntOut {
    value.find_lsb_op()
}

/// Returns the bit number of the most-significant bit (`1` for positive/unsigned, `0` for negative), or `-1` if `0` or `-1` (`findMSB`).
#[inline(always)]
#[must_use]
pub fn find_msb<T: BitScanOp>(value: T) -> T::IntOut {
    value.find_msb_op()
}
