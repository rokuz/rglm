#![cfg(feature = "derive")]

use core::mem::{align_of, offset_of, size_of};
use rglm::*;

#[derive(Clone, Copy, Debug, Std140, Std430, Scalar)]
pub struct Vec3PlusFloat {
    pub pos: Vec3,
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, Std140, Std430, Scalar)]
pub struct MisalignedVec3Sandwich {
    pub head: f32,
    pub dir: Vec3,
    pub tail: f32,
}

#[derive(Clone, Copy, Debug, Std140, Std430, Scalar)]
pub struct MatrixAndArrayBlock {
    pub m2: Mat2,
    pub m3: Mat3,
    pub weights: [f32; 4],
    pub flag: bool,
}

#[derive(Clone, Copy, Debug, Std140, Std430, Scalar)]
pub struct InnerBlock {
    pub v: Vec3,
}

#[derive(Clone, Copy, Debug, Std140, Std430, Scalar)]
pub struct OuterBlock {
    pub inner: InnerBlock,
    pub after: f32,
}

#[test]
fn vec3_plus_float_packs_to_16_bytes_in_all_three_layouts() {
    assert_eq!(offset_of!(Vec3PlusFloatStd140, pos), 0);
    assert_eq!(offset_of!(Vec3PlusFloatStd140, radius), 12);
    assert_eq!(size_of::<Vec3PlusFloatStd140>(), 16);
    assert_eq!(align_of::<Vec3PlusFloatStd140>(), 16);

    assert_eq!(offset_of!(Vec3PlusFloatStd430, pos), 0);
    assert_eq!(offset_of!(Vec3PlusFloatStd430, radius), 12);
    assert_eq!(size_of::<Vec3PlusFloatStd430>(), 16);
    assert_eq!(align_of::<Vec3PlusFloatStd430>(), 16);

    assert_eq!(offset_of!(Vec3PlusFloatScalar, pos), 0);
    assert_eq!(offset_of!(Vec3PlusFloatScalar, radius), 12);
    assert_eq!(size_of::<Vec3PlusFloatScalar>(), 16);
    assert_eq!(align_of::<Vec3PlusFloatScalar>(), 4);
}

#[test]
fn sandwich_offsets_and_zeroed_padding() {
    assert_eq!(offset_of!(MisalignedVec3SandwichStd430, head), 0);
    assert_eq!(offset_of!(MisalignedVec3SandwichStd430, dir), 16);
    assert_eq!(offset_of!(MisalignedVec3SandwichStd430, tail), 28);
    assert_eq!(size_of::<MisalignedVec3SandwichStd430>(), 32);

    assert_eq!(offset_of!(MisalignedVec3SandwichScalar, head), 0);
    assert_eq!(offset_of!(MisalignedVec3SandwichScalar, dir), 4);
    assert_eq!(offset_of!(MisalignedVec3SandwichScalar, tail), 16);
    assert_eq!(size_of::<MisalignedVec3SandwichScalar>(), 20);

    let host = MisalignedVec3Sandwich {
        head: 1.0,
        dir: vec3!(2.0, 3.0, 4.0),
        tail: 5.0,
    };
    let mut bytes = [0xff_u8; <MisalignedVec3Sandwich as Std430>::SIZE];
    write_std430(&host, &mut bytes);
    assert_eq!(&bytes[4..16], &[0u8; 12]);
    assert_eq!(
        f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        1.0
    );
    assert_eq!(
        f32::from_ne_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
        5.0
    );
}

#[test]
fn matrix_array_and_nested_struct_rules() {
    assert_eq!(offset_of!(MatrixAndArrayBlockStd140, m2), 0);
    assert_eq!(offset_of!(MatrixAndArrayBlockStd140, m3), 32);
    assert_eq!(offset_of!(MatrixAndArrayBlockStd140, weights), 80);
    assert_eq!(offset_of!(MatrixAndArrayBlockStd140, flag), 144);
    assert_eq!(size_of::<MatrixAndArrayBlockStd140>(), 160);

    assert_eq!(offset_of!(MatrixAndArrayBlockStd430, m2), 0);
    assert_eq!(offset_of!(MatrixAndArrayBlockStd430, m3), 16);
    assert_eq!(offset_of!(MatrixAndArrayBlockStd430, weights), 64);
    assert_eq!(offset_of!(MatrixAndArrayBlockStd430, flag), 80);
    assert_eq!(size_of::<MatrixAndArrayBlockStd430>(), 96);

    assert_eq!(offset_of!(MatrixAndArrayBlockScalar, m2), 0);
    assert_eq!(offset_of!(MatrixAndArrayBlockScalar, m3), 16);
    assert_eq!(offset_of!(MatrixAndArrayBlockScalar, weights), 52);
    assert_eq!(offset_of!(MatrixAndArrayBlockScalar, flag), 68);
    assert_eq!(size_of::<MatrixAndArrayBlockScalar>(), 72);

    assert_eq!(offset_of!(OuterBlockStd140, after), 16);
    assert_eq!(size_of::<OuterBlockStd140>(), 32);
    assert_eq!(offset_of!(OuterBlockStd430, after), 16);
    assert_eq!(size_of::<OuterBlockStd430>(), 32);
    assert_eq!(offset_of!(OuterBlockScalar, after), 12);
    assert_eq!(size_of::<OuterBlockScalar>(), 16);
}

#[cfg(all(feature = "f16", feature = "f64"))]
#[test]
fn f16_and_f64_vec3_packing_offsets() {
    #[derive(Clone, Copy, Debug, Std140, Std430, Scalar)]
    struct HalfAndDoubleBlock {
        h3: F16Vec3,
        h1: f16,
        d3: DVec3,
        d1: f64,
    }

    assert_eq!(offset_of!(HalfAndDoubleBlockStd430, h3), 0);
    assert_eq!(offset_of!(HalfAndDoubleBlockStd430, h1), 6);
    assert_eq!(offset_of!(HalfAndDoubleBlockStd430, d3), 32);
    assert_eq!(offset_of!(HalfAndDoubleBlockStd430, d1), 56);
    assert_eq!(size_of::<HalfAndDoubleBlockStd430>(), 64);
    assert_eq!(align_of::<HalfAndDoubleBlockStd430>(), 32);
}
