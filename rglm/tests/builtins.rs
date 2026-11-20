use rglm::glsl_names::{
    bitCount, bitfieldExtract, bitfieldInsert, findLSB, findMSB, floatBitsToUint, lessThan,
    matrixCompMult, outerProduct, packHalf2x16, roundEven, uintBitsToFloat, unpackHalf2x16,
};
use rglm::*;

fn ulp_diff_f32(got: f32, expected: f64) -> f64 {
    let ref_f32 = expected as f32;
    if got.is_nan() && expected.is_nan() {
        return 0.0;
    }
    if got.is_infinite() && ref_f32.is_infinite() && got.signum() == ref_f32.signum() {
        return 0.0;
    }
    let diff = ((got as f64) - expected).abs();
    if diff == 0.0 {
        return 0.0;
    }
    let ax = expected.abs();
    let min_normal = f64::from_bits((1023 - 126) << 52);
    let ulp = if ax < min_normal {
        f64::from_bits((1023 - 149) << 52)
    } else {
        let exp_bits = (ax.to_bits() >> 52) & 0x7ff;
        f64::from_bits((exp_bits - 23) << 52)
    };
    diff / ulp
}

#[test]
fn glsl_common_edge_cases() {
    assert_eq!(fract(vec3!(-0.25, 0.0, 1.75)), vec3!(0.75, 0.0, 0.75));
    assert_eq!(
        fract(vec4!(-0.375, -1.0, 0.25, 2.5)),
        vec4!(0.625, 0.0, 0.25, 0.5)
    );

    let s = sign(vec4!(-0.0, 0.0, f32::NAN, -4.0));
    assert_eq!(s.x.to_bits(), 0.0_f32.to_bits());
    assert_eq!(s.y.to_bits(), 0.0_f32.to_bits());
    assert_eq!(s.z.to_bits(), 0.0_f32.to_bits());
    assert_eq!(s.w, -1.0);

    assert_eq!(modulo(vec2!(-5.0, 5.0), 3.0), vec2!(1.0, 2.0));
    assert_eq!(r#mod(-5.0_f32, 3.0_f32), 1.0);

    assert_eq!(
        round_even(vec4!(0.5, 1.5, 2.5, -2.5)),
        vec4!(0.0, 2.0, 2.0, -2.0)
    );
    assert_eq!(roundEven(2.5_f32), 2.0_f32);

    let (f_part, i_part) = modf(vec3!(-3.25, 0.0, f32::INFINITY));
    assert_eq!(f_part.x, -0.25);
    assert_eq!(i_part.x, -3.0);
    assert_eq!(f_part.z, 0.0);
    assert!(i_part.z.is_infinite());

    assert_eq!(
        mix(vec3!(0.0, 10.0, 20.0), vec3!(10.0, 20.0, 30.0), 0.5),
        vec3!(5.0, 15.0, 25.0)
    );
    assert_eq!(
        mix(
            vec3!(1.0, 2.0, 3.0),
            vec3!(9.0, 8.0, 7.0),
            bvec3!(true, false, true)
        ),
        vec3!(9.0, 2.0, 7.0)
    );
    assert_eq!(step(0.5, vec3!(0.2, 0.5, 0.8)), vec3!(0.0, 1.0, 1.0));
    assert_eq!(
        smoothstep(0.0, 1.0, vec3!(-1.0, 0.5, 2.0)),
        vec3!(0.0, 0.5, 1.0)
    );
    assert_eq!(
        isnan(vec3!(f32::NAN, 0.0, f32::INFINITY)),
        bvec3!(true, false, false)
    );
    assert_eq!(
        isinf(vec3!(f32::NAN, 0.0, f32::NEG_INFINITY)),
        bvec3!(false, false, true)
    );

    let bits = floatBitsToUint(vec2!(1.0, -2.0));
    assert_eq!(uintBitsToFloat(bits), vec2!(1.0, -2.0));

    let (sig, exp_i) = frexp(vec2!(6.0, -0.5));
    assert_eq!(sig, vec2!(0.75, -0.5));
    assert_eq!(exp_i, ivec2!(3, 0));
    assert_eq!(ldexp(sig, exp_i), vec2!(6.0, -0.5));
}

#[test]
fn simd_transcendentals_ulp_accuracy() {
    for i in 0..256 {
        let t = (i as f32) / 255.0;
        let angle = -10.0 + 20.0 * t;
        let v = vec4!(angle, angle * 0.5, angle * 0.25, -angle);

        let sv = to_array(sin(v));
        let cv = to_array(cos(v));
        let (sc_s, sc_c) = sincos(v);
        let va = to_array(v);
        for lane in 0..4 {
            let x64 = va[lane] as f64;
            assert!(ulp_diff_f32(sv[lane], x64.sin()) <= 1.5);
            assert!(ulp_diff_f32(cv[lane], x64.cos()) <= 1.5);
            assert_eq!(sv[lane].to_bits(), sc_s[lane].to_bits());
            assert_eq!(cv[lane].to_bits(), sc_c[lane].to_bits());
        }

        let unit = -0.98 + 1.96 * t;
        let uv = vec4!(unit, -unit, unit * 0.5, 0.0);
        let as_v = to_array(asin(uv));
        let ac_v = to_array(acos(uv));
        let at_v = to_array(atan!(uv));
        let uva = to_array(uv);
        for lane in 0..4 {
            let u64v = uva[lane] as f64;
            assert!(ulp_diff_f32(as_v[lane], u64v.asin()) <= 1.5);
            assert!(ulp_diff_f32(ac_v[lane], u64v.acos()) <= 1.5);
            assert!(ulp_diff_f32(at_v[lane], u64v.atan()) <= 1.5);
        }

        let ex_in = vec4!(-5.0 + 10.0 * t, 0.1 + 4.0 * t, 0.5 + 10.0 * t, 2.0);
        let ev = to_array(exp(ex_in));
        let e2v = to_array(exp2(ex_in));
        let pos_in = vec4!(0.05 + 20.0 * t, 0.5 + t, 1.0 + 3.0 * t, 100.0 * t + 0.01);
        let lv = to_array(log(pos_in));
        let l2v = to_array(log2(pos_in));
        for lane in 0..4 {
            assert!(ulp_diff_f32(ev[lane], (ex_in[lane] as f64).exp()) <= 1.5);
            assert!(ulp_diff_f32(e2v[lane], (ex_in[lane] as f64).exp2()) <= 1.5);
            assert!(ulp_diff_f32(lv[lane], (pos_in[lane] as f64).ln()) <= 1.5);
            assert!(ulp_diff_f32(l2v[lane], (pos_in[lane] as f64).log2()) <= 1.5);
        }
    }

    assert_eq!(atan!(1.0_f32, 1.0_f32), atan2(1.0_f32, 1.0_f32));
    #[cfg(not(feature = "debug-assert"))]
    assert!(normalize(vec3(0.0)).x.is_nan());
}

#[cfg(feature = "debug-assert")]
#[test]
#[should_panic]
fn normalize_zero_triggers_debug_assert() {
    let _ = normalize(vec3(0.0));
}

#[test]
fn geometric_relational_packing_and_integer_builtins() {
    assert_eq!(dot(vec3!(1.0, 2.0, 3.0), vec3!(4.0, 5.0, 6.0)), 32.0);
    assert_eq!(
        cross(vec3!(1.0, 0.0, 0.0), vec3!(0.0, 1.0, 0.0)),
        vec3!(0.0, 0.0, 1.0)
    );
    assert_eq!(distance(vec2!(0.0, 0.0), vec2!(3.0, 4.0)), 5.0);
    assert_eq!(reflect(vec2!(1.0, -1.0), vec2!(0.0, 1.0)), vec2!(1.0, 1.0));
    assert_eq!(
        faceforward(vec2!(0.0, 1.0), vec2!(0.0, -1.0), vec2!(0.0, 1.0)),
        vec2!(0.0, 1.0)
    );
    assert_eq!(
        refract(vec2!(1.0, 0.0), vec2!(0.0, 1.0), 2.0),
        vec2!(0.0, 0.0)
    );

    let m = outerProduct(vec2!(1.0, 2.0), vec3!(3.0, 4.0, 5.0));
    assert_eq!(
        m,
        mat3x2!(vec2!(3.0, 6.0), vec2!(4.0, 8.0), vec2!(5.0, 10.0))
    );
    assert_eq!(matrixCompMult(mat2(2.0), mat2(3.0)), mat2(6.0));

    let lt = lessThan(ivec3!(1, 5, 3), ivec3!(2, 5, 1));
    assert_eq!(lt, bvec3!(true, false, false));
    assert!(any(lt));
    assert!(!all(lt));
    assert_eq!(not(lt), bvec3!(false, true, true));

    let h = packHalf2x16(vec2!(1.5, -0.25));
    assert_eq!(unpackHalf2x16(h), vec2!(1.5, -0.25));
    assert_eq!(
        unpack_unorm_2x16(pack_unorm_2x16(vec2!(0.0, 1.0))),
        vec2!(0.0, 1.0)
    );
    assert_eq!(
        unpack_snorm_2x16(pack_snorm_2x16(vec2!(-1.0, 1.0))),
        vec2!(-1.0, 1.0)
    );
    assert_eq!(
        unpack_unorm_4x8(pack_unorm_4x8(vec4!(0.0, 1.0, 0.0, 1.0))),
        vec4!(0.0, 1.0, 0.0, 1.0)
    );
    assert_eq!(
        unpack_snorm_4x8(pack_snorm_4x8(vec4!(-1.0, 0.0, 1.0, -1.0))),
        vec4!(-1.0, 0.0, 1.0, -1.0)
    );

    assert_eq!(uadd_carry(u32::MAX, 2), (1, 1));
    assert_eq!(usub_borrow(1u32, 3u32), (u32::MAX - 1, 1));
    assert_eq!(umul_extended(0x1_0000_u32, 0x2_0000_u32), (2, 0));
    assert_eq!(imul_extended(-2_i32, 3_i32), (-1, -6));

    assert_eq!(bitfieldExtract(-1_i32, 4, 0), 0);
    assert_eq!(bitfieldExtract(0b0001_1100_i32, 2, 3), -1);
    assert_eq!(bitfieldExtract(0b0001_1100_u32, 2, 3), 0b111);
    assert_eq!(bitfieldInsert(0_u32, 0b101_u32, 4, 3), 0b0101_0000);
    assert_eq!(bitfield_reverse(0x8000_0001_u32), 0x8000_0001_u32);
    assert_eq!(bitCount(uvec3!(0, 7, u32::MAX)), ivec3!(0, 3, 32));
    assert_eq!(findLSB(ivec3!(0, 8, -4)), ivec3!(-1, 3, 2));
    assert_eq!(findMSB(ivec4!(0, -1, -2, 8)), ivec4!(-1, -1, 0, 3));
}
