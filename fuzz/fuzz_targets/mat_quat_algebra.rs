#![cfg_attr(all(feature = "libfuzzer", fuzzing), no_main)]

use arbitrary::Arbitrary;
use core::hint::black_box;
use rglm::gtx::safe_ops::*;
use rglm::*;

#[derive(Arbitrary, Debug)]
struct AlgebraInput {
    m2: Mat2,
    m3: Mat3,
    m4a: Mat4,
    m4b: Mat4,
    m23: Mat2x3,
    m34: Mat3x4,
    m42: Mat4x2,
    dm4: DMat4,
    hm4: F16Mat4,
    q1: Quat,
    q2: Quat,
    v3: Vec3,
    v4: Vec4,
    t: f32,
}

fn check_algebra(inp: AlgebraInput) {
    let tt34 = transpose(transpose(inp.m34));
    let a = to_array(inp.m34);
    let b = to_array(tt34);
    for i in 0..12 {
        assert_eq!(a[i].to_bits(), b[i].to_bits());
    }

    black_box(inp.m4a * inp.m4b);
    black_box(inp.m4a * inp.v4);
    black_box(inp.v4 * inp.m4a);
    black_box(inp.m23 * inp.m42);
    black_box(inp.m34 * inp.v3);
    black_box(inp.v4 * inp.m34);
    black_box(determinant(inp.m2));
    black_box(determinant(inp.m3));
    black_box(determinant(inp.m4a));
    black_box(inverse(inp.m2));
    black_box(inverse(inp.m3));
    black_box(inverse(inp.m4a));
    black_box(try_inverse(inp.m4a));
    black_box(inverse(inp.dm4));
    black_box(inverse(inp.hm4));

    black_box(inp.q1 * inp.q2);
    black_box(inp.q1 * inp.v3);
    black_box(conjugate(inp.q1));
    black_box(inverse(inp.q1));
    black_box(try_inverse(inp.q1));
    black_box(try_normalize(inp.v3));
    black_box(normalize_or_zero(inp.v3));
    black_box(mat3_cast(inp.q1));
    black_box(mat4_cast(inp.q1));
    black_box(quat_cast(inp.m3));
    black_box(euler_angles(inp.q1));
    black_box(slerp(inp.q1, inp.q2, inp.t));
}

#[cfg(all(feature = "libfuzzer", fuzzing))]
libfuzzer_sys::fuzz_target!(|inp: AlgebraInput| {
    check_algebra(inp);
});

#[cfg(not(all(feature = "libfuzzer", fuzzing)))]
fn main() {
    let mut state = 0xfeed_face_cafe_babe_u64;
    let mut buf = [0u8; 1024];
    for _ in 0..512 {
        for b in &mut buf {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(3);
            *b = (state >> 56) as u8;
        }
        let mut u = arbitrary::Unstructured::new(&buf);
        if let Ok(inp) = AlgebraInput::arbitrary(&mut u) {
            check_algebra(inp);
        }
    }
}
