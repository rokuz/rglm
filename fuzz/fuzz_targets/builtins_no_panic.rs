#![cfg_attr(all(feature = "libfuzzer", fuzzing), no_main)]

use arbitrary::Arbitrary;
use core::hint::black_box;
use rglm::*;

#[derive(Arbitrary, Debug)]
struct BuiltinInput {
    v2: Vec2,
    v3: Vec3,
    v3a: Vec3A,
    v4a: Vec4,
    v4b: Vec4,
    v4c: Vec4,
    dv4: DVec4,
    hv4: F16Vec4,
    iv4: IVec4,
    uv4a: UVec4,
    uv4b: UVec4,
    s: f32,
    offset: i32,
    bits: i32,
    raw_u32: u32,
}

fn check_builtins(inp: BuiltinInput) {
    let s_out = sign(inp.v4a);
    for lane in to_array(s_out) {
        assert!(lane == -1.0 || lane == 0.0 || lane == 1.0);
        if lane == 0.0 {
            assert_eq!(lane.to_bits(), 0);
        }
    }

    black_box(abs(inp.v4a));
    black_box(floor(inp.v4a));
    black_box(trunc(inp.v4a));
    black_box(round(inp.v4a));
    black_box(round_even(inp.v4a));
    black_box(ceil(inp.v4a));
    black_box(fract(inp.v4a));
    black_box(modulo(inp.v4a, inp.v4b));
    black_box(min(inp.v4a, inp.v4b));
    black_box(max(inp.v4a, inp.v4b));

    let lo = min(inp.v4a, inp.v4b);
    let hi = max(inp.v4a, inp.v4b);
    black_box(clamp(inp.v4c, lo, hi));
    black_box(mix(inp.v4a, inp.v4b, inp.v4c));
    black_box(step(inp.v4a, inp.v4b));
    black_box(smoothstep(lo, hi, inp.v4c));
    black_box(fma(inp.v4a, inp.v4b, inp.v4c));

    black_box(sin(inp.v4a));
    black_box(cos(inp.v4a));
    black_box(sincos(inp.v4a));
    black_box(tan(inp.v4a));
    black_box(asin(inp.v4a));
    black_box(acos(inp.v4a));
    black_box(atan(inp.v4a));
    black_box(atan2(inp.v4a, inp.v4b));
    black_box(sinh(inp.v4a));
    black_box(cosh(inp.v4a));
    black_box(tanh(inp.v4a));
    black_box(asinh(inp.v4a));
    black_box(acosh(inp.v4a));
    black_box(atanh(inp.v4a));
    black_box(exp(inp.v4a));
    black_box(exp2(inp.v4a));
    black_box(log(inp.v4a));
    black_box(log2(inp.v4a));
    black_box(pow(inp.v4a, inp.v4b));
    black_box(sqrt(inp.v4a));
    black_box(inversesqrt(inp.v4a));

    black_box(sin(inp.dv4));
    black_box(fract(inp.dv4));
    black_box(sin(inp.hv4));
    black_box(fract(inp.hv4));

    black_box(length(inp.v3));
    black_box(distance(inp.v3, Vec3::from(inp.v3a)));
    black_box(dot(inp.v4a, inp.v4b));
    black_box(cross(inp.v3, Vec3::from(inp.v3a)));
    black_box(normalize(inp.v3));
    black_box(normalize(inp.v3a));
    black_box(faceforward(inp.v3, Vec3::from(inp.v3a), inp.v3));
    black_box(reflect(inp.v3, Vec3::from(inp.v3a)));
    black_box(refract(inp.v3, Vec3::from(inp.v3a), inp.s));

    black_box(pack_unorm_2x16(inp.v2));
    black_box(pack_snorm_2x16(inp.v2));
    black_box(pack_unorm_4x8(inp.v4a));
    black_box(pack_snorm_4x8(inp.v4a));
    black_box(pack_half_2x16(inp.v2));
    black_box(unpack_unorm_2x16(inp.raw_u32));
    black_box(unpack_snorm_2x16(inp.raw_u32));
    black_box(unpack_unorm_4x8(inp.raw_u32));
    black_box(unpack_snorm_4x8(inp.raw_u32));
    black_box(unpack_half_2x16(inp.raw_u32));

    black_box(bitfield_extract(inp.iv4, inp.offset, inp.bits));
    black_box(bitfield_extract(inp.uv4a, inp.offset, inp.bits));
    black_box(bitfield_insert(inp.uv4a, inp.uv4b, inp.offset, inp.bits));
    black_box(bitfield_reverse(inp.uv4a));
    black_box(bit_count(inp.uv4a));
    black_box(find_lsb(inp.uv4a));
    black_box(find_msb(inp.iv4));
    black_box(find_msb(inp.uv4a));
}

#[cfg(all(feature = "libfuzzer", fuzzing))]
libfuzzer_sys::fuzz_target!(|inp: BuiltinInput| {
    check_builtins(inp);
});

#[cfg(not(all(feature = "libfuzzer", fuzzing)))]
fn main() {
    let mut state = 0x1234_5678_9abc_def0_u64;
    let mut buf = [0u8; 512];
    for _ in 0..512 {
        for b in &mut buf {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            *b = (state >> 56) as u8;
        }
        let mut u = arbitrary::Unstructured::new(&buf);
        if let Ok(inp) = BuiltinInput::arbitrary(&mut u) {
            check_builtins(inp);
        }
    }
}
