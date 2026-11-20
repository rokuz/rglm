use crate::math::f32_math;
use crate::{Vec2, Vec4};

#[inline(always)]
fn f32_to_f16_bits(val: f32) -> u16 {
    #[cfg(feature = "f16")]
    {
        half::f16::from_f32(val).to_bits()
    }
    #[cfg(not(feature = "f16"))]
    {
        let bits = val.to_bits();
        let sign = ((bits >> 16) & 0x8000) as u16;
        let exp = ((bits >> 23) & 0xff) as i32;
        let mant = bits & 0x007f_ffff;
        if exp == 0xff {
            if mant != 0 {
                return sign | 0x7e00 | ((mant >> 13) as u16);
            }
            return sign | 0x7c00;
        }
        let new_exp = exp - 127 + 15;
        if new_exp >= 31 {
            return sign | 0x7c00;
        }
        if new_exp <= 0 {
            if new_exp < -10 {
                return sign;
            }
            let m = mant | 0x0080_0000;
            let shift = (14 - new_exp) as u32;
            let half_bit = 1u32 << (shift - 1);
            let rem = m & ((1u32 << shift) - 1);
            let mut sub = (m >> shift) as u16;
            if rem > half_bit || (rem == half_bit && (sub & 1) != 0) {
                sub += 1;
            }
            return sign | sub;
        }
        let rem = mant & 0x1fff;
        let mut h = sign | ((new_exp as u16) << 10) | ((mant >> 13) as u16);
        if rem > 0x1000 || (rem == 0x1000 && (h & 1) != 0) {
            h = h.wrapping_add(1);
        }
        h
    }
}

#[inline(always)]
fn f16_bits_to_f32(bits: u16) -> f32 {
    #[cfg(feature = "f16")]
    {
        half::f16::from_bits(bits).to_f32()
    }
    #[cfg(not(feature = "f16"))]
    {
        let sign = ((bits & 0x8000) as u32) << 16;
        let exp = ((bits >> 10) & 0x1f) as u32;
        let mant = (bits & 0x03ff) as u32;
        if exp == 0 {
            if mant == 0 {
                return f32::from_bits(sign);
            }
            let mut m = mant;
            let mut e = 127u32 - 14;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e -= 1;
            }
            m &= 0x03ff;
            return f32::from_bits(sign | (e << 23) | (m << 13));
        }
        if exp == 31 {
            return f32::from_bits(sign | 0x7f80_0000 | (mant << 13));
        }
        f32::from_bits(sign | ((exp + 127 - 15) << 23) | (mant << 13))
    }
}

/// Packs a `Vec2` in `[0.0, 1.0]` into two 16-bit unsigned normalized integers in a `u32` (`packUnorm2x16`).
#[inline]
#[must_use]
pub fn pack_unorm_2x16(v: Vec2) -> u32 {
    let x = f32_math::round_even(v.x.clamp(0.0, 1.0) * 65535.0) as u16;
    let y = f32_math::round_even(v.y.clamp(0.0, 1.0) * 65535.0) as u16;
    (x as u32) | ((y as u32) << 16)
}

/// Unpacks a `u32` containing two 16-bit unsigned normalized integers into a `Vec2` in `[0.0, 1.0]` (`unpackUnorm2x16`).
#[inline]
#[must_use]
pub fn unpack_unorm_2x16(p: u32) -> Vec2 {
    let x = (p & 0xffff) as u16;
    let y = (p >> 16) as u16;
    Vec2::new((x as f32) / 65535.0, (y as f32) / 65535.0)
}

/// Packs a `Vec2` in `[-1.0, 1.0]` into two 16-bit signed normalized integers in a `u32` (`packSnorm2x16`).
#[inline]
#[must_use]
pub fn pack_snorm_2x16(v: Vec2) -> u32 {
    let x = (f32_math::round_even(v.x.clamp(-1.0, 1.0) * 32767.0) as i16) as u16;
    let y = (f32_math::round_even(v.y.clamp(-1.0, 1.0) * 32767.0) as i16) as u16;
    (x as u32) | ((y as u32) << 16)
}

/// Unpacks a `u32` containing two 16-bit signed normalized integers into a `Vec2` in `[-1.0, 1.0]` (`unpackSnorm2x16`).
#[inline]
#[must_use]
pub fn unpack_snorm_2x16(p: u32) -> Vec2 {
    let x = (p & 0xffff) as u16 as i16;
    let y = (p >> 16) as u16 as i16;
    Vec2::new(
        ((x as f32) / 32767.0).clamp(-1.0, 1.0),
        ((y as f32) / 32767.0).clamp(-1.0, 1.0),
    )
}

/// Packs a `Vec4` in `[0.0, 1.0]` into four 8-bit unsigned normalized integers in a `u32` (`packUnorm4x8`).
#[inline]
#[must_use]
pub fn pack_unorm_4x8(v: Vec4) -> u32 {
    let c = (v.clamp(Vec4::ZERO, Vec4::ONE) * 255.0)
        .round_even()
        .to_array();
    (c[0] as u8 as u32)
        | ((c[1] as u8 as u32) << 8)
        | ((c[2] as u8 as u32) << 16)
        | ((c[3] as u8 as u32) << 24)
}

/// Unpacks a `u32` containing four 8-bit unsigned normalized integers into a `Vec4` in `[0.0, 1.0]` (`unpackUnorm4x8`).
#[inline]
#[must_use]
pub fn unpack_unorm_4x8(p: u32) -> Vec4 {
    let b0 = (p & 0xff) as f32;
    let b1 = ((p >> 8) & 0xff) as f32;
    let b2 = ((p >> 16) & 0xff) as f32;
    let b3 = ((p >> 24) & 0xff) as f32;
    Vec4::new(b0, b1, b2, b3) / 255.0
}

/// Packs a `Vec4` in `[-1.0, 1.0]` into four 8-bit signed normalized integers in a `u32` (`packSnorm4x8`).
#[inline]
#[must_use]
pub fn pack_snorm_4x8(v: Vec4) -> u32 {
    let c = (v.clamp(Vec4::NEG_ONE, Vec4::ONE) * 127.0)
        .round_even()
        .to_array();
    let b0 = (c[0] as i8) as u8 as u32;
    let b1 = (c[1] as i8) as u8 as u32;
    let b2 = (c[2] as i8) as u8 as u32;
    let b3 = (c[3] as i8) as u8 as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

/// Unpacks a `u32` containing four 8-bit signed normalized integers into a `Vec4` in `[-1.0, 1.0]` (`unpackSnorm4x8`).
#[inline]
#[must_use]
pub fn unpack_snorm_4x8(p: u32) -> Vec4 {
    let b0 = ((p & 0xff) as u8 as i8) as f32;
    let b1 = (((p >> 8) & 0xff) as u8 as i8) as f32;
    let b2 = (((p >> 16) & 0xff) as u8 as i8) as f32;
    let b3 = (((p >> 24) & 0xff) as u8 as i8) as f32;
    (Vec4::new(b0, b1, b2, b3) / 127.0).clamp(Vec4::NEG_ONE, Vec4::ONE)
}

/// Packs two `f32` lanes of `v` into IEEE-754 binary16 values stored in the low (`x`) and high (`y`) 16 bits (`packHalf2x16`).
#[inline]
#[must_use]
pub fn pack_half_2x16(v: Vec2) -> u32 {
    let lo = f32_to_f16_bits(v.x) as u32;
    let hi = f32_to_f16_bits(v.y) as u32;
    lo | (hi << 16)
}

/// Unpacks two IEEE-754 binary16 values from the low and high 16 bits of `v` into a `Vec2` (`unpackHalf2x16`).
#[inline]
#[must_use]
pub fn unpack_half_2x16(v: u32) -> Vec2 {
    let lo = (v & 0xffff) as u16;
    let hi = (v >> 16) as u16;
    Vec2::new(f16_bits_to_f32(lo), f16_bits_to_f32(hi))
}

/// Packs two 32-bit unsigned lanes of `v` (`x` as LSB, `y` as MSB) into an IEEE-754 `f64` (`packDouble2x32`).
#[cfg(all(feature = "f64", feature = "u32"))]
#[inline(always)]
#[must_use]
pub fn pack_double_2x32(v: crate::UVec2) -> f64 {
    f64::from_bits((v.x as u64) | ((v.y as u64) << 32))
}

/// Unpacks the 64-bit IEEE-754 representation of `v` into `(lsb, msb)` in a `UVec2` (`unpackDouble2x32`).
#[cfg(all(feature = "f64", feature = "u32"))]
#[inline(always)]
#[must_use]
pub fn unpack_double_2x32(v: f64) -> crate::UVec2 {
    let bits = v.to_bits();
    crate::UVec2::new(bits as u32, (bits >> 32) as u32)
}
