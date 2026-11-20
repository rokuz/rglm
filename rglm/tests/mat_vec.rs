use core::mem::{align_of, size_of};
use rglm::*;

fn approx_eq_f32(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

fn approx_eq_f64(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn type_sizes_and_alignments() {
    assert_eq!(size_of::<Vec2>(), 8);
    assert_eq!(size_of::<Vec3>(), 12);
    assert_eq!(align_of::<Vec3>(), 4);
    assert_eq!(size_of::<Vec3A>(), 16);
    assert_eq!(align_of::<Vec3A>(), 16);
    assert_eq!(size_of::<Vec4>(), 16);
    assert_eq!(align_of::<Vec4>(), 16);

    assert_eq!(size_of::<Mat2>(), 16);
    assert_eq!(align_of::<Mat2>(), 16);
    assert_eq!(size_of::<Mat2x3>(), 24);
    assert_eq!(size_of::<Mat2x3A>(), 32);
    assert_eq!(size_of::<Mat2x4>(), 32);
    assert_eq!(size_of::<Mat3x2>(), 24);
    assert_eq!(size_of::<Mat3>(), 36);
    assert_eq!(align_of::<Mat3>(), 4);
    assert_eq!(size_of::<Mat3A>(), 48);
    assert_eq!(align_of::<Mat3A>(), 16);
    assert_eq!(size_of::<Mat3x4>(), 48);
    assert_eq!(size_of::<Mat4x2>(), 32);
    assert_eq!(size_of::<Mat4x3>(), 48);
    assert_eq!(size_of::<Mat4x3A>(), 64);
    assert_eq!(size_of::<Mat4>(), 64);
    assert_eq!(align_of::<Mat4>(), 16);
    assert_eq!(size_of::<Quat>(), 16);
    assert_eq!(align_of::<Quat>(), 16);

    assert_eq!(size_of::<dvec3>(), 24);
    assert_eq!(size_of::<dvec4>(), 32);
    assert_eq!(size_of::<dmat3x4>(), 96);
    assert_eq!(size_of::<dmat4>(), 128);
    assert_eq!(size_of::<dquat>(), 32);

    assert_eq!(size_of::<f16vec2>(), 4);
    assert_eq!(size_of::<f16vec3>(), 6);
    assert_eq!(size_of::<f16vec4>(), 8);
    assert_eq!(size_of::<f16mat3x4>(), 24);
    assert_eq!(size_of::<f16mat4>(), 32);
    assert_eq!(size_of::<f16quat>(), 8);
}

#[test]
fn rectangular_matrix_vector_and_transpose_f32() {
    let m3x2 = Mat3x2::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
    let v3 = Vec3::from([2.0, -1.0, 3.0]);
    let v2 = Vec2::from([4.0, -2.0]);

    let col_prod: Vec2 = m3x2 * v3;
    assert_eq!(col_prod, Vec2::from([14.0, 18.0]));

    let row_prod: Vec3 = v2 * m3x2;
    assert_eq!(row_prod, Vec3::from([0.0, 4.0, 8.0]));

    let m2x3: Mat2x3 = transpose(m3x2);
    assert_eq!(m2x3 * v2, row_prod);
    assert_eq!(v3 * m2x3, col_prod);
    assert_eq!(transpose(m2x3), m3x2);

    let m4x3 = Mat4x3::from([
        [1.0, 0.0, 2.0],
        [-1.0, 3.0, 1.0],
        [2.0, 1.0, 0.0],
        [0.0, -2.0, 4.0],
    ]);
    let m2x4 = Mat2x4::from([[2.0, 1.0, -1.0, 3.0], [0.0, 2.0, 4.0, -1.0]]);
    let m2x3_prod: Mat2x3 = m4x3 * m2x4;
    let m3x2_trans_prod: Mat3x2 = transpose(m2x4) * transpose(m4x3);
    assert_eq!(transpose(m2x3_prod), m3x2_trans_prod);

    let m3x4 = Mat3x4::from([
        [1.0, 2.0, 3.0, 4.0],
        [0.5, -1.0, 2.0, 0.0],
        [-2.0, 1.0, 0.0, 3.0],
    ]);
    let m4x2 = Mat4x2::from([[1.0, 0.0], [2.0, -1.0], [-1.0, 3.0], [0.5, 2.0]]);
    let m3x2_prod: Mat3x2 = m4x2 * m3x4;
    assert_eq!(transpose(m3x2_prod), transpose(m3x4) * transpose(m4x2));
}

#[test]
fn aligned_matrices_match_packed_matrices() {
    let cols_2x3 = [[1.5, -2.0, 3.0], [4.0, 0.5, -1.0]];
    let m2x3 = Mat2x3::from(cols_2x3);
    let m2x3a = Mat2x3A::from([
        Vec3A::from(cols_2x3[0]).into(),
        Vec3A::from(cols_2x3[1]).into(),
    ]);
    let v2 = Vec2::from([2.0, -3.0]);
    let v3 = Vec3::from([1.0, 2.0, -4.0]);
    let v3a = Vec3A::from(v3);
    assert_eq!(Vec3::from(m2x3a * v2), m2x3 * v2);
    assert_eq!(v3a * m2x3a, v3 * m2x3);

    let cols_3x3 = [[2.0, 1.0, 0.0], [-1.0, 3.0, 2.0], [1.0, 0.0, 4.0]];
    let m3 = Mat3::from(cols_3x3);
    let m3a = Mat3A::from(cols_3x3);
    assert_eq!(Vec3::from(m3a * v3a), m3 * v3);
    assert_eq!(Vec3::from(v3a * m3a), v3 * m3);
    assert!(approx_eq_f32(determinant(m3), determinant(m3a), 1e-6));

    let inv3 = inverse(m3);
    let inv3a = inverse(m3a);
    let prod3 = m3 * inv3;
    let prod3a = m3a * inv3a;
    for col in 0..3 {
        for row in 0..3 {
            let expected = if col == row { 1.0 } else { 0.0 };
            assert!(approx_eq_f32(prod3[col][row], expected, 1e-5));
            assert!(approx_eq_f32(prod3a[col][row], expected, 1e-5));
        }
    }

    let cols_4x3 = [
        [1.0, 2.0, -1.0],
        [0.0, 3.0, 2.0],
        [-2.0, 1.0, 0.5],
        [4.0, -1.0, 1.5],
    ];
    let m4x3 = Mat4x3::from(cols_4x3);
    let m4x3a = Mat4x3A::from(cols_4x3);
    let v4 = Vec4::from([1.0, -2.0, 3.0, 0.5]);
    assert_eq!(Vec3::from(m4x3a * v4), m4x3 * v4);
    assert_eq!(v3a * m4x3a, v3 * m4x3);
}

#[test]
fn outer_product_comp_mult_and_inverses() {
    let c = Vec4::from([2.0, -1.0, 3.0, 0.5]);
    let r = Vec3::from([4.0, 5.0, -2.0]);
    let op: Mat3x4 = outer_product(c, r);
    assert_eq!(op.x_axis, c * 4.0);
    assert_eq!(op.y_axis, c * 5.0);
    assert_eq!(op.z_axis, c * -2.0);

    let b = Mat3x4::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 0.0, -1.0, 2.0],
        [-1.0, 3.0, 2.0, 1.0],
    ]);
    let cm = matrix_comp_mult(op, b);
    for col in 0..3 {
        for row in 0..4 {
            assert_eq!(cm[col][row], op[col][row] * b[col][row]);
        }
    }

    let m2 = Mat2::from([[4.0, 2.0], [7.0, 6.0]]);
    assert!(approx_eq_f32(determinant(m2), 10.0, 1e-6));
    let id2 = m2 * inverse(m2);
    assert!(approx_eq_f32(id2.x_axis.x, 1.0, 1e-6));
    assert!(approx_eq_f32(id2.x_axis.y, 0.0, 1e-6));
    assert!(approx_eq_f32(id2.y_axis.x, 0.0, 1e-6));
    assert!(approx_eq_f32(id2.y_axis.y, 1.0, 1e-6));

    let m4 = Mat4::from([
        [2.0, 0.0, 1.0, 0.0],
        [1.0, 3.0, 0.0, 2.0],
        [0.0, -1.0, 2.0, 1.0],
        [1.0, 2.0, -1.0, 4.0],
    ]);
    let id4 = m4 * inverse(m4);
    for col in 0..4 {
        for row in 0..4 {
            let expected = if col == row { 1.0 } else { 0.0 };
            assert!(approx_eq_f32(id4[col][row], expected, 1e-5));
        }
    }

    let mut row_v = Vec4::from([1.0, -2.0, 3.0, 4.0]);
    let expected_row = row_v * m4;
    row_v *= m4;
    assert_eq!(row_v, expected_row);
}

#[test]
fn f64_and_f16_matrices_and_quaternions() {
    let dm = dmat4::from([
        [3.0, 1.0, 0.0, 0.0],
        [0.0, 2.0, 1.0, 0.0],
        [1.0, 0.0, 4.0, 1.0],
        [0.0, 1.0, 0.0, 2.0],
    ]);
    let did = dm * inverse(dm);
    for col in 0..4 {
        for row in 0..4 {
            let expected = if col == row { 1.0 } else { 0.0 };
            assert!(approx_eq_f64(did[col][row], expected, 1e-12));
        }
    }

    let h0 = f16::from_f32(0.0);
    let h1 = f16::from_f32(1.0);
    let h2 = f16::from_f32(2.0);
    let h3 = f16::from_f32(3.0);
    let hm = f16mat2x3::from([[h1, h2, h0], [h3, h0, h1]]);
    let hv2 = f16vec2::from([h2, h1]);
    let hv3 = hm * hv2;
    assert_eq!(
        hv3,
        f16vec3::from([f16::from_f32(5.0), f16::from_f32(4.0), h1])
    );

    let s = core::f32::consts::FRAC_1_SQRT_2;
    let q_z90 = Quat::from([0.0, 0.0, s, s]);
    let vx = Vec3::X;
    let rotated = q_z90 * vx;
    assert!(approx_eq_f32(rotated.x, 0.0, 1e-6));
    assert!(approx_eq_f32(rotated.y, 1.0, 1e-6));
    assert!(approx_eq_f32(rotated.z, 0.0, 1e-6));

    let inv_rotated = rotated * q_z90;
    assert!(approx_eq_f32(inv_rotated.x, 1.0, 1e-6));
    assert!(approx_eq_f32(inv_rotated.y, 0.0, 1e-6));
    assert!(approx_eq_f32(inv_rotated.z, 0.0, 1e-6));
}

#[test]
fn integer_and_boolean_vectors_and_aligned_prelude() {
    let a = ivec4::from([10, -20, 30, -40]);
    let b = ivec4::from([3, 5, -6, 8]);
    assert_eq!(a + b, ivec4::from([13, -15, 24, -32]));
    assert_eq!(a * 2, ivec4::from([20, -40, 60, -80]));
    assert_eq!(
        uvec2::from([0b1010, 0b0011]) << 2,
        uvec2::from([0b101000, 0b001100])
    );

    let m1 = bvec4::from([true, false, true, false]);
    let m2 = bvec4::from([true, true, false, false]);
    assert_eq!(m1 & m2, bvec4::from([true, false, false, false]));
    assert_eq!(m1 | m2, bvec4::from([true, true, true, false]));
    assert_eq!(!m1, bvec4::from([false, true, false, true]));

    let av: aligned::vec3 = aligned::vec3::from([1.0, 2.0, 3.0]);
    assert_eq!(size_of::<aligned::vec3>(), 16);
    assert_eq!(size_of::<aligned::mat3>(), 48);
    assert_eq!(aligned::mat3::IDENTITY * av, av);
}
