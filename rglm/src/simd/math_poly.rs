use super::{SimdF32x4, SimdMask32x4};
use crate::math::f32_math;

const FRAC_2_PI: f32 = core::f32::consts::FRAC_2_PI;
const PIO2_C1: f32 = f32::from_bits(0x3fc9_0000);
const PIO2_C2: f32 = f32::from_bits(0x39fd_a000);
const PIO2_C3: f32 = f32::from_bits(0x33a2_2000);
const PIO2_C4: f32 = f32::from_bits(0x2c34_6000);
const PIO2_C5: f32 = f32::from_bits(0x248d_3132);
const PIO2_HI: f32 = f32::from_bits(0x3fc9_0f80);
const PIO2_TAIL: f32 = (core::f64::consts::FRAC_PI_2 - (PIO2_HI as f64)) as f32;
const PIO4_HI: f32 = f32::from_bits(0x3f49_0f80);
const PIO4_LO: f32 = (core::f64::consts::FRAC_PI_4 - (PIO4_HI as f64)) as f32;
const PI_HI: f32 = f32::from_bits(0x4049_0f80);
const PI_LO: f32 = (core::f64::consts::PI - (PI_HI as f64)) as f32;

const LN2_HI: f32 = f32::from_bits(0x3f31_7200);
const LN2_LO: f32 = (core::f64::consts::LN_2 - (LN2_HI as f64)) as f32;
const LOG2_E_HI: f32 = f32::from_bits(0x3fb8_aa00);
const LOG2_E_LO: f32 = (core::f64::consts::LOG2_E - (LOG2_E_HI as f64)) as f32;

#[inline(always)]
fn two_sum(a: SimdF32x4, b: SimdF32x4) -> (SimdF32x4, SimdF32x4) {
    let s = a.add(b);
    let v = s.sub(a);
    let e = a.sub(s.sub(v)).add(b.sub(v));
    (s, e)
}

#[inline(always)]
fn reduce_pio2(x: SimdF32x4) -> (SimdF32x4, SimdF32x4, SimdF32x4) {
    let k_flt = x
        .mul(SimdF32x4::splat(FRAC_2_PI))
        .round_even()
        .add(SimdF32x4::splat(0.0));
    let neg_k = SimdF32x4::splat(0.0).sub(k_flt);
    let p1 = neg_k.mul(SimdF32x4::splat(PIO2_C1));
    let p2 = neg_k.mul(SimdF32x4::splat(PIO2_C2));
    let p3 = neg_k.mul(SimdF32x4::splat(PIO2_C3));
    let p4 = neg_k.mul(SimdF32x4::splat(PIO2_C4));
    let p5 = neg_k.mul(SimdF32x4::splat(PIO2_C5));

    let s1 = x.add(p1);
    let (s2, e2) = two_sum(s1, p2);
    let (s3, e3) = two_sum(s2, p3);
    let (s4, e4) = two_sum(s3, p4);
    let tail = p5.add(e4).add(e3).add(e2);
    let (r_hi, r_lo) = two_sum(s4, tail);
    (k_flt, r_hi, r_lo)
}

impl SimdF32x4 {
    #[inline]
    pub(crate) fn sin_cos(self) -> (Self, Self) {
        if !self.abs().cmp_le(Self::splat(4096.0)).all4() {
            let a = self.to_array();
            let (s0, c0) = f32_math::sin_cos(a[0]);
            let (s1, c1) = f32_math::sin_cos(a[1]);
            let (s2, c2) = f32_math::sin_cos(a[2]);
            let (s3, c3) = f32_math::sin_cos(a[3]);
            return (
                Self::from_array([s0, s1, s2, s3]),
                Self::from_array([c0, c1, c2, c3]),
            );
        }

        let (k_flt, r, r_lo) = reduce_pio2(self);
        let r2 = r.mul(r);
        let r3 = r2.mul(r);
        let r4 = r2.mul(r2);

        let sin_poly = Self::splat(-2.505_076e-8)
            .mul_add(r2, Self::splat(2.755_731_4e-6))
            .mul_add(r2, Self::splat(-1.984_127e-4))
            .mul_add(r2, Self::splat(8.333_334e-3))
            .mul_add(r2, Self::splat(-1.666_666_7e-1));
        let sin_r = r.add(r3.mul_add(sin_poly, r_lo));

        let cos_poly = Self::splat(-2.752_155_7e-7)
            .mul_add(r2, Self::splat(2.480_060_1e-5))
            .mul_add(r2, Self::splat(-1.388_886_7e-3))
            .mul_add(r2, Self::splat(4.166_666_4e-2));
        let half_r2 = r2.mul(Self::splat(0.5));
        let w = Self::splat(1.0).sub(half_r2);
        let w_err = Self::splat(1.0).sub(w).sub(half_r2);
        let cos_r = w.add(r4.mul_add(cos_poly, w_err.sub(r.mul(r_lo))));

        let ka = k_flt.to_array();
        let q = [
            (ka[0] as i32) & 3,
            (ka[1] as i32) & 3,
            (ka[2] as i32) & 3,
            (ka[3] as i32) & 3,
        ];

        let swap_mask = SimdMask32x4::new(
            (q[0] & 1) != 0,
            (q[1] & 1) != 0,
            (q[2] & 1) != 0,
            (q[3] & 1) != 0,
        );
        let sin_neg = SimdMask32x4::new(
            (q[0] & 2) != 0,
            (q[1] & 2) != 0,
            (q[2] & 2) != 0,
            (q[3] & 2) != 0,
        );
        let cos_neg = SimdMask32x4::new(
            ((q[0].wrapping_add(1)) & 2) != 0,
            ((q[1].wrapping_add(1)) & 2) != 0,
            ((q[2].wrapping_add(1)) & 2) != 0,
            ((q[3].wrapping_add(1)) & 2) != 0,
        );

        let s_unsigned = Self::select(swap_mask, cos_r, sin_r);
        let c_unsigned = Self::select(swap_mask, sin_r, cos_r);

        let s_signed = Self::select(sin_neg, s_unsigned.neg(), s_unsigned);
        let c_signed = Self::select(cos_neg, c_unsigned.neg(), c_unsigned);

        let zero_mask = self.cmp_eq(Self::splat(0.0));
        let s_out = Self::select(zero_mask, self, s_signed);
        (s_out, c_signed)
    }

    #[inline]
    pub(crate) fn sin(self) -> Self {
        self.sin_cos().0
    }

    #[inline]
    pub(crate) fn cos(self) -> Self {
        self.sin_cos().1
    }

    #[inline]
    pub(crate) fn tan(self) -> Self {
        if !self.abs().cmp_le(Self::splat(4096.0)).all4() {
            let a = self.to_array();
            return Self::from_array([
                f32_math::tan(a[0]),
                f32_math::tan(a[1]),
                f32_math::tan(a[2]),
                f32_math::tan(a[3]),
            ]);
        }

        let (k_flt, r_hi, r_lo) = reduce_pio2(self);
        let r2 = r_hi.mul(r_hi);
        let r2_lo = r_hi.fma_exact(r_hi, r2.neg());
        let r3 = r2.mul(r_hi);
        let r3_lo = r2.fma_exact(r_hi, r3.neg()).add(r2_lo.mul(r_hi));
        let poly = Self::splat(9.385_402e-3)
            .mul_add(r2, Self::splat(3.119_922_3e-3))
            .mul_add(r2, Self::splat(2.443_013_5e-2))
            .mul_add(r2, Self::splat(5.341_128e-2))
            .mul_add(r2, Self::splat(1.333_879_9e-1))
            .mul(r2);
        let c3_hi = Self::splat(f32::from_bits(0x3eaa_aa00));
        let c3_lo = Self::splat(3.333_315_6e-1 - f32::from_bits(0x3eaa_aa00));
        let t3_hi = r3.mul(c3_hi);
        let t3_lo = r3
            .fma_exact(c3_hi, t3_hi.neg())
            .add(r3.mul(c3_lo))
            .add(r3_lo.mul(Self::splat(3.333_315_6e-1)));
        let (s_hi, s_lo) = two_sum(r_hi, t3_hi);
        let tail = r3.mul_add(poly, t3_lo).add(r_lo).add(s_lo);
        let (t_hi, t_lo) = two_sum(s_hi, tail);

        let ka = k_flt.to_array();
        let odd_mask = SimdMask32x4::new(
            ((ka[0] as i32) & 1) != 0,
            ((ka[1] as i32) & 1) != 0,
            ((ka[2] as i32) & 1) != 0,
            ((ka[3] as i32) & 1) != 0,
        );
        let q0 = Self::splat(-1.0).div(t_hi);
        let rem = q0.fma_exact(t_hi, Self::splat(1.0)).add(q0.mul(t_lo));
        let cot = rem.mul_add(q0, q0);
        let res = Self::select(odd_mask, cot, t_hi);
        Self::select(self.cmp_eq(Self::splat(0.0)), self, res)
    }

    #[inline]
    pub(crate) fn exp(self) -> Self {
        if !self.abs().cmp_le(Self::splat(87.0)).all4() {
            let a = self.to_array();
            return Self::from_array([
                f32_math::exp(a[0]),
                f32_math::exp(a[1]),
                f32_math::exp(a[2]),
                f32_math::exp(a[3]),
            ]);
        }

        let k_flt = self
            .mul(Self::splat(core::f32::consts::LOG2_E))
            .round_even();
        let r = self
            .sub(k_flt.mul(Self::splat(LN2_HI)))
            .sub(k_flt.mul(Self::splat(LN2_LO)));
        let r2 = r.mul(r);

        let poly = Self::splat(1.987_569_1e-4)
            .mul_add(r, Self::splat(1.398_199_9e-3))
            .mul_add(r, Self::splat(8.333_452e-3))
            .mul_add(r, Self::splat(4.166_579_6e-2))
            .mul_add(r, Self::splat(1.666_666_6e-1))
            .mul_add(r, Self::splat(0.5));

        let exp_r = r2.mul_add(poly, r.add(Self::splat(1.0)));
        let ka = k_flt.to_array();
        let scale = Self::from_bits([
            (((ka[0] as i32) + 127) as u32) << 23,
            (((ka[1] as i32) + 127) as u32) << 23,
            (((ka[2] as i32) + 127) as u32) << 23,
            (((ka[3] as i32) + 127) as u32) << 23,
        ]);
        exp_r.mul(scale)
    }

    #[inline]
    pub(crate) fn exp2(self) -> Self {
        if !self.abs().cmp_le(Self::splat(126.0)).all4() {
            let a = self.to_array();
            return Self::from_array([
                f32_math::exp2(a[0]),
                f32_math::exp2(a[1]),
                f32_math::exp2(a[2]),
                f32_math::exp2(a[3]),
            ]);
        }

        let k_flt = self.round_even();
        let r = self.sub(k_flt);
        let poly = Self::splat(1.525_273_4e-5)
            .mul_add(r, Self::splat(1.540_353e-4))
            .mul_add(r, Self::splat(1.333_355_8e-3))
            .mul_add(r, Self::splat(9.618_129e-3))
            .mul_add(r, Self::splat(5.550_411e-2))
            .mul_add(r, Self::splat(2.402_265e-1))
            .mul_add(r, Self::splat(6.931_472e-1));

        let exp2_r = r.mul_add(poly, Self::splat(1.0));
        let ka = k_flt.to_array();
        let scale = Self::from_bits([
            (((ka[0] as i32) + 127) as u32) << 23,
            (((ka[1] as i32) + 127) as u32) << 23,
            (((ka[2] as i32) + 127) as u32) << 23,
            (((ka[3] as i32) + 127) as u32) << 23,
        ]);
        exp2_r.mul(scale)
    }

    #[inline]
    pub(crate) fn ln(self) -> Self {
        let bits = self.to_bits();
        let normal = (bits[0].wrapping_sub(0x0080_0000) < 0x7f00_0000)
            && (bits[1].wrapping_sub(0x0080_0000) < 0x7f00_0000)
            && (bits[2].wrapping_sub(0x0080_0000) < 0x7f00_0000)
            && (bits[3].wrapping_sub(0x0080_0000) < 0x7f00_0000);

        if !normal {
            let a = self.to_array();
            return Self::from_array([
                f32_math::ln(a[0]),
                f32_math::ln(a[1]),
                f32_math::ln(a[2]),
                f32_math::ln(a[3]),
            ]);
        }

        let mut m_bits = [0u32; 4];
        let mut exp_f = [0.0f32; 4];
        let mut i = 0;
        while i < 4 {
            let b = bits[i];
            let mut e = ((b >> 23) as i32) - 127;
            let mut mant = (b & 0x007f_ffff) | 0x3f80_0000;
            if mant >= 0x3fb5_04f3 {
                mant -= 0x0080_0000;
                e += 1;
            }
            m_bits[i] = mant;
            exp_f[i] = e as f32;
            i += 1;
        }

        let m = Self::from_bits(m_bits);
        let e = Self::from_array(exp_f);
        let one = Self::splat(1.0);
        let f = m.sub(one);
        let g = one.add(m);
        let g_lo = m.sub(g.sub(one));
        let s = f.div(g);
        let s_lo = s.neg().fma_exact(g, f).sub(s.mul(g_lo)).div(g);
        let two_s = s.add(s);
        let two_s_lo = s_lo.add(s_lo);
        let s2 = s.mul(s);

        let poly = Self::splat(1.0 / 11.0)
            .mul_add(s2, Self::splat(1.0 / 9.0))
            .mul_add(s2, Self::splat(1.0 / 7.0))
            .mul_add(s2, Self::splat(1.0 / 5.0))
            .mul_add(s2, Self::splat(1.0 / 3.0));
        let ln_m_tail = two_s.mul(s2).mul_add(poly, two_s_lo);
        let lo = e.mul_add(Self::splat(LN2_LO), ln_m_tail.add(two_s));
        e.mul_add(Self::splat(LN2_HI), lo)
    }

    #[inline]
    pub(crate) fn log2(self) -> Self {
        let bits = self.to_bits();
        let normal = (bits[0].wrapping_sub(0x0080_0000) < 0x7f00_0000)
            && (bits[1].wrapping_sub(0x0080_0000) < 0x7f00_0000)
            && (bits[2].wrapping_sub(0x0080_0000) < 0x7f00_0000)
            && (bits[3].wrapping_sub(0x0080_0000) < 0x7f00_0000);

        if !normal {
            let a = self.to_array();
            return Self::from_array([
                f32_math::log2(a[0]),
                f32_math::log2(a[1]),
                f32_math::log2(a[2]),
                f32_math::log2(a[3]),
            ]);
        }

        let mut m_bits = [0u32; 4];
        let mut exp_f = [0.0f32; 4];
        let mut i = 0;
        while i < 4 {
            let b = bits[i];
            let mut e = ((b >> 23) as i32) - 127;
            let mut mant = (b & 0x007f_ffff) | 0x3f80_0000;
            if mant >= 0x3fb5_04f3 {
                mant -= 0x0080_0000;
                e += 1;
            }
            m_bits[i] = mant;
            exp_f[i] = e as f32;
            i += 1;
        }

        let m = Self::from_bits(m_bits);
        let e = Self::from_array(exp_f);
        let one = Self::splat(1.0);
        let f = m.sub(one);
        let g = one.add(m);
        let g_lo = m.sub(g.sub(one));
        let s = f.div(g);
        let s_lo = s.neg().fma_exact(g, f).sub(s.mul(g_lo)).div(g);
        let two_s = s.add(s);
        let two_s_lo = s_lo.add(s_lo);
        let s2 = s.mul(s);

        let poly = Self::splat(1.0 / 11.0)
            .mul_add(s2, Self::splat(1.0 / 9.0))
            .mul_add(s2, Self::splat(1.0 / 7.0))
            .mul_add(s2, Self::splat(1.0 / 5.0))
            .mul_add(s2, Self::splat(1.0 / 3.0));
        let hi_prod = two_s.mul(Self::splat(LOG2_E_HI));
        let hi_err = two_s.fma_exact(Self::splat(LOG2_E_HI), hi_prod.neg());
        let tail = two_s.mul(s2).mul_add(poly, two_s_lo).mul_add(
            Self::splat(core::f32::consts::LOG2_E),
            two_s.mul_add(Self::splat(LOG2_E_LO), hi_err),
        );
        e.add(hi_prod).add(tail)
    }

    #[inline]
    pub(crate) fn atan(self) -> Self {
        if !self.abs().cmp_lt(Self::splat(f32::INFINITY)).all4() {
            let a = self.to_array();
            return Self::from_array([
                f32_math::atan(a[0]),
                f32_math::atan(a[1]),
                f32_math::atan(a[2]),
                f32_math::atan(a[3]),
            ]);
        }

        let x = self.abs();
        let one = Self::splat(1.0);
        let gt_one = x.cmp_gt(one);
        let inv_x = one.div(x);
        let inv_err = x.neg().fma_exact(inv_x, one).mul(inv_x);
        let x1 = Self::select(gt_one, inv_x, x);
        let x1_lo = Self::select(gt_one, inv_err, Self::splat(0.0));

        let gt_tan_pio8 = x1.cmp_gt(Self::splat(0.414_213_57));
        let sum = one.add(x1);
        let sum_lo = x1.sub(sum.sub(one)).add(x1_lo);
        let diff = x1.sub(one);
        let diff_lo = x1.sub(one.add(diff)).add(x1_lo);
        let q = diff.div(sum);
        let q_corr = q
            .neg()
            .fma_exact(sum, diff)
            .add(diff_lo.sub(q.mul(sum_lo)))
            .div(sum);
        let x2 = Self::select(gt_tan_pio8, q.add(q_corr), x1);
        let x2_lo = Self::select(gt_tan_pio8, q.sub(x2).add(q_corr), x1_lo);

        let z = x2.mul(x2);
        let poly = Self::splat(1.0 / 17.0)
            .mul_add(z, Self::splat(-1.0 / 15.0))
            .mul_add(z, Self::splat(1.0 / 13.0))
            .mul_add(z, Self::splat(-1.0 / 11.0))
            .mul_add(z, Self::splat(1.0 / 9.0))
            .mul_add(z, Self::splat(-1.0 / 7.0))
            .mul_add(z, Self::splat(1.0 / 5.0))
            .mul_add(z, Self::splat(-1.0 / 3.0));
        let core_tail = x2.mul(z).mul_add(poly, x2_lo);

        let y_mid = Self::splat(PIO4_HI).add(x2.add(core_tail.add(Self::splat(PIO4_LO))));
        let y_small = x2.add(core_tail);
        let y = Self::select(gt_tan_pio8, y_mid, y_small);
        let y_hi_tail = Self::select(gt_tan_pio8, y_mid.sub(Self::splat(PIO4_HI)), y_small);
        let y_gt1 = Self::splat(PIO2_HI).sub(Self::select(
            gt_tan_pio8,
            Self::splat(PIO4_HI).add(y_hi_tail.sub(Self::splat(PIO2_TAIL))),
            y_small.sub(Self::splat(PIO2_TAIL)),
        ));
        let y = Self::select(gt_one, y_gt1, y);

        let sign_bits = [
            self.to_bits()[0] & 0x8000_0000,
            self.to_bits()[1] & 0x8000_0000,
            self.to_bits()[2] & 0x8000_0000,
            self.to_bits()[3] & 0x8000_0000,
        ];
        let yb = y.to_bits();
        Self::from_bits([
            yb[0] | sign_bits[0],
            yb[1] | sign_bits[1],
            yb[2] | sign_bits[2],
            yb[3] | sign_bits[3],
        ])
    }

    #[inline]
    pub(crate) fn asin(self) -> Self {
        let ax = self.abs();
        if !ax.cmp_le(Self::splat(1.0)).all4() {
            let a = self.to_array();
            return Self::from_array([
                f32_math::asin(a[0]),
                f32_math::asin(a[1]),
                f32_math::asin(a[2]),
                f32_math::asin(a[3]),
            ]);
        }

        let gt_half = ax.cmp_gt(Self::splat(0.5));
        let z_large = Self::splat(0.5).mul(Self::splat(1.0).sub(ax));
        let sq = z_large.sqrt();
        let sq_lo = Self::select(
            z_large.cmp_gt(Self::splat(0.0)),
            sq.neg().fma_exact(sq, z_large).div(sq.add(sq)),
            Self::splat(0.0),
        );
        let x_reduced = Self::select(gt_half, sq, ax);
        let x_lo = Self::select(gt_half, sq_lo, Self::splat(0.0));
        let z = Self::select(gt_half, z_large, ax.mul(ax));

        let poly = Self::splat(4.216_32e-2)
            .mul_add(z, Self::splat(2.418_131e-2))
            .mul_add(z, Self::splat(4.547_002_6e-2))
            .mul_add(z, Self::splat(7.495_300_5e-2))
            .mul_add(z, Self::splat(1.666_675_2e-1));
        let tail = x_reduced.mul(z).mul_add(poly, x_lo);
        let core_val = x_reduced.add(tail);
        let two_tail = tail.add(tail);
        let large_val = Self::splat(PIO2_HI).sub(
            x_reduced
                .add(x_reduced)
                .add(two_tail.sub(Self::splat(PIO2_TAIL))),
        );
        let mag = Self::select(gt_half, large_val, core_val);

        let sb = self.to_bits();
        let mb = mag.to_bits();
        Self::from_bits([
            mb[0] | (sb[0] & 0x8000_0000),
            mb[1] | (sb[1] & 0x8000_0000),
            mb[2] | (sb[2] & 0x8000_0000),
            mb[3] | (sb[3] & 0x8000_0000),
        ])
    }

    #[inline]
    pub(crate) fn acos(self) -> Self {
        let ax = self.abs();
        if !ax.cmp_le(Self::splat(1.0)).all4() {
            let a = self.to_array();
            return Self::from_array([
                f32_math::acos(a[0]),
                f32_math::acos(a[1]),
                f32_math::acos(a[2]),
                f32_math::acos(a[3]),
            ]);
        }

        let gt_half = ax.cmp_gt(Self::splat(0.5));
        let z_large = Self::splat(0.5).mul(Self::splat(1.0).sub(ax));
        let sq = z_large.sqrt();
        let sq_lo = Self::select(
            z_large.cmp_gt(Self::splat(0.0)),
            sq.neg().fma_exact(sq, z_large).div(sq.add(sq)),
            Self::splat(0.0),
        );
        let x_reduced = Self::select(gt_half, sq, ax);
        let x_lo = Self::select(gt_half, sq_lo, Self::splat(0.0));
        let z = Self::select(gt_half, z_large, ax.mul(ax));

        let poly = Self::splat(4.216_32e-2)
            .mul_add(z, Self::splat(2.418_131e-2))
            .mul_add(z, Self::splat(4.547_002_6e-2))
            .mul_add(z, Self::splat(7.495_300_5e-2))
            .mul_add(z, Self::splat(1.666_675_2e-1));
        let tail = x_reduced.mul(z).mul_add(poly, x_lo);
        let core_val = x_reduced.add(tail);

        let neg_mask = self.cmp_lt(Self::splat(0.0));
        let signed_core = Self::select(neg_mask, core_val.neg(), core_val);
        let signed_tail = Self::select(neg_mask, tail.neg(), tail);
        let signed_x = Self::select(neg_mask, x_reduced.neg(), x_reduced);
        let small_out =
            Self::splat(PIO2_HI).sub(signed_x.add(signed_tail.sub(Self::splat(PIO2_TAIL))));
        let two_x = x_reduced.add(x_reduced);
        let two_tail = tail.add(tail);
        let large_out = Self::select(
            neg_mask,
            Self::splat(PI_HI).sub(two_x.add(two_tail.sub(Self::splat(PI_LO)))),
            two_x.add(two_tail),
        );
        let _ = signed_core;
        Self::select(gt_half, large_out, small_out)
    }

    #[inline]
    pub(crate) fn atan2(self, x: Self) -> Self {
        let ya = self.to_array();
        let xa = x.to_array();
        let finite_non_zero =
            (ya[0].is_finite() && xa[0].is_finite() && (ya[0] != 0.0 || xa[0] != 0.0))
                && (ya[1].is_finite() && xa[1].is_finite() && (ya[1] != 0.0 || xa[1] != 0.0))
                && (ya[2].is_finite() && xa[2].is_finite() && (ya[2] != 0.0 || xa[2] != 0.0))
                && (ya[3].is_finite() && xa[3].is_finite() && (ya[3] != 0.0 || xa[3] != 0.0));

        if !finite_non_zero {
            return Self::from_array([
                f32_math::atan2(ya[0], xa[0]),
                f32_math::atan2(ya[1], xa[1]),
                f32_math::atan2(ya[2], xa[2]),
                f32_math::atan2(ya[3], xa[3]),
            ]);
        }

        let ay = self.abs();
        let ax = x.abs();
        let swap = ay.cmp_gt(ax);
        let num = Self::select(swap, ax, ay);
        let den = Self::select(swap, ay, ax);
        let ratio = num.div(den);
        let ratio_lo = ratio.neg().fma_exact(den, num).div(den);
        let base = ratio
            .atan()
            .add(ratio_lo.div(ratio.mul_add(ratio, Self::splat(1.0))));
        let angle = Self::select(
            swap,
            Self::splat(PIO2_HI).sub(base.sub(Self::splat(PIO2_TAIL))),
            base,
        );
        let x_neg = SimdMask32x4::new(
            (x.to_bits()[0] & 0x8000_0000) != 0,
            (x.to_bits()[1] & 0x8000_0000) != 0,
            (x.to_bits()[2] & 0x8000_0000) != 0,
            (x.to_bits()[3] & 0x8000_0000) != 0,
        );
        let quad = Self::select(
            x_neg,
            Self::splat(PI_HI).sub(angle.sub(Self::splat(PI_LO))),
            angle,
        );
        let yb = self.to_bits();
        let qb = quad.to_bits();
        Self::from_bits([
            qb[0] | (yb[0] & 0x8000_0000),
            qb[1] | (yb[1] & 0x8000_0000),
            qb[2] | (yb[2] & 0x8000_0000),
            qb[3] | (yb[3] & 0x8000_0000),
        ])
    }

    #[inline]
    pub(crate) fn powf(self, y: Self) -> Self {
        let a = self.to_array();
        let b = y.to_array();
        Self::from_array([
            f32_math::powf(a[0], b[0]),
            f32_math::powf(a[1], b[1]),
            f32_math::powf(a[2], b[2]),
            f32_math::powf(a[3], b[3]),
        ])
    }
}
