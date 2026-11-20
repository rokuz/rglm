#![cfg_attr(all(feature = "libfuzzer", fuzzing), no_main)]

use arbitrary::Arbitrary;
use core::mem::offset_of;
use rglm::layout::{
    Scalar, Std140, Std430, to_scalar, to_std140, to_std430, write_scalar, write_std140,
    write_std430,
};
use rglm::*;

#[derive(Arbitrary, Clone, Copy, Debug, PartialEq, Std140, Std430, Scalar)]
struct FuzzInner {
    pos: Vec3,
    radius: f32,
    flag: bool,
}

#[derive(Arbitrary, Clone, Copy, Debug, PartialEq, Std140, Std430, Scalar)]
struct FuzzUniformLeaf {
    a: f32,
    b: Vec3,
    c: i32,
}

#[derive(Arbitrary, Clone, Copy, Debug, PartialEq, Std140, Std430, Scalar)]
struct FuzzUniform {
    tint: Vec3,
    exposure: f32,
    basis: Mat3,
    proj: Mat4,
    rot: Quat,
    inner: FuzzUniformLeaf,
    uvs: [Vec2; 2],
}

fn read_u32_at(buf: &[u8], off: usize) -> u32 {
    u32::from_ne_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

fn check_layout(u: FuzzUniform, inner: FuzzInner) {
    let repr140 = to_std140(&u);
    assert_eq!(repr140.exposure.to_bits(), u.exposure.to_bits());
    let repr430 = to_std430(&u);
    assert_eq!(repr430.exposure.to_bits(), u.exposure.to_bits());
    let repr_sc = to_scalar(&u);
    assert_eq!(repr_sc.exposure.to_bits(), u.exposure.to_bits());

    let mut buf140 = [0xff_u8; <FuzzUniform as Std140>::SIZE];
    write_std140(&u, &mut buf140);
    let tint_arr = to_array(u.tint);
    assert_eq!(read_u32_at(&buf140, 0), tint_arr[0].to_bits());
    assert_eq!(read_u32_at(&buf140, 4), tint_arr[1].to_bits());
    assert_eq!(read_u32_at(&buf140, 8), tint_arr[2].to_bits());
    assert_eq!(
        read_u32_at(&buf140, offset_of!(FuzzUniformStd140, exposure)),
        u.exposure.to_bits()
    );

    let leaf_off_430 = offset_of!(FuzzUniformStd430, inner);
    let mut buf430 = [0xff_u8; <FuzzUniform as Std430>::SIZE];
    write_std430(&u, &mut buf430);
    assert_eq!(read_u32_at(&buf430, leaf_off_430), u.inner.a.to_bits());
    for &pad in &buf430[leaf_off_430 + 4..leaf_off_430 + 16] {
        assert_eq!(pad, 0);
    }
    assert_eq!(read_u32_at(&buf430, leaf_off_430 + 28) as i32, u.inner.c);

    let mut buf_sc = [0xff_u8; <FuzzUniform as Scalar>::SIZE];
    write_scalar(&u, &mut buf_sc);
    let leaf_off_sc = offset_of!(FuzzUniformScalar, inner);
    assert_eq!(read_u32_at(&buf_sc, leaf_off_sc), u.inner.a.to_bits());
    assert_eq!(read_u32_at(&buf_sc, leaf_off_sc + 4), u.inner.b.x.to_bits());
    assert_eq!(read_u32_at(&buf_sc, leaf_off_sc + 16) as i32, u.inner.c);

    let mut ibuf = [0xff_u8; <FuzzInner as Std140>::SIZE];
    write_std140(&inner, &mut ibuf);
    assert_eq!(
        read_u32_at(&ibuf, offset_of!(FuzzInnerStd140, flag)),
        u32::from(inner.flag)
    );
    for &pad in &ibuf[20..<FuzzInner as Std140>::SIZE] {
        assert_eq!(pad, 0);
    }
}

#[cfg(all(feature = "libfuzzer", fuzzing))]
libfuzzer_sys::fuzz_target!(|pair: (FuzzUniform, FuzzInner)| {
    check_layout(pair.0, pair.1);
});

#[cfg(not(all(feature = "libfuzzer", fuzzing)))]
fn main() {
    let mut state = 0x0ddc_0ffe_e123_4567_u64;
    let mut buf = [0u8; 1024];
    for _ in 0..512 {
        for b in &mut buf {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(7);
            *b = (state >> 56) as u8;
        }
        let mut u = arbitrary::Unstructured::new(&buf);
        if let Ok((uni, inner)) = <(FuzzUniform, FuzzInner)>::arbitrary(&mut u) {
            check_layout(uni, inner);
        }
    }
}
