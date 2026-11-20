use rglm::*;

#[test]
fn read_swizzles_across_alphabets_and_lengths() {
    let v2 = vec2!(10.0, 20.0);
    assert_eq!(rswz!(v2.x), 10.0);
    assert_eq!(rswz!(v2.y), 20.0);
    assert_eq!(rswz!(v2.yx), vec2!(20.0, 10.0));
    assert_eq!(rswz!(v2.xyx), vec3!(10.0, 20.0, 10.0));
    assert_eq!(rswz!(v2.xxxx), vec4!(10.0, 10.0, 10.0, 10.0));
    assert_eq!(rswz!(v2.stst), vec4!(10.0, 20.0, 10.0, 20.0));

    let color = vec4!(0.1, 0.2, 0.3, 0.4);
    assert_eq!(rswz!(color.r), 0.1);
    assert_eq!(rswz!(color.bgr), vec3!(0.3, 0.2, 0.1));
    assert_eq!(rswz!(color.abgr), vec4!(0.4, 0.3, 0.2, 0.1));
    assert_eq!(rswz!(color.qpst), vec4!(0.4, 0.3, 0.1, 0.2));

    let a = vec4!(1.0, 2.0, 3.0, 4.0);
    let b = vec4!(10.0, 20.0, 30.0, 40.0);
    assert_eq!(rswz!((a + b).wzyx), vec4!(44.0, 33.0, 22.0, 11.0));

    let m = mat4!(
        vec4!(1.0, 2.0, 3.0, 4.0),
        vec4!(5.0, 6.0, 7.0, 8.0),
        vec4!(9.0, 10.0, 11.0, 12.0),
        vec4!(13.0, 14.0, 15.0, 16.0),
    );
    assert_eq!(rswz!(m.z_axis.zyx), vec3!(11.0, 10.0, 9.0));

    let arr = [vec3!(1.0, 2.0, 3.0), vec3!(4.0, 5.0, 6.0)];
    assert_eq!(rswz!(arr[1].zx), vec2!(6.0, 4.0));
}

#[test]
fn write_swizzles_and_compound_assignments() {
    let mut v = vec4!(1.0, 2.0, 3.0, 4.0);
    swz!(v.xz) = vec2!(10.0, 30.0);
    assert_eq!(v, vec4!(10.0, 2.0, 30.0, 4.0));

    swz!(v.wzyx) = vec4!(100.0, 200.0, 300.0, 400.0);
    assert_eq!(v, vec4!(400.0, 300.0, 200.0, 100.0));

    swz!(v.rgb) *= 0.5;
    assert_eq!(v, vec4!(200.0, 150.0, 100.0, 100.0));

    swz!(v.a) += 25.0;
    assert_eq!(v, vec4!(200.0, 150.0, 100.0, 125.0));

    let mut m = mat4(1.0);
    swz!(m.w_axis.xyz) = vec3!(7.0, 8.0, 9.0);
    assert_eq!(m.w_axis, vec4!(7.0, 8.0, 9.0, 1.0));

    let mut pts = [vec3a!(1.0, 2.0, 3.0), vec3a!(4.0, 5.0, 6.0)];
    swz!(pts[0].zy) += vec2!(10.0, 20.0);
    assert_eq!(pts[0], vec3a!(1.0, 22.0, 13.0));

    let mut mask = bvec4a!(true, false, false, true);
    swz!(mask.yz) = bvec2!(true, true);
    assert_eq!(mask, bvec4a!(true, true, true, true));
}

#[test]
fn glsl_vector_constructors_and_casts() {
    assert_eq!(vec3(5.0), vec3!(5.0, 5.0, 5.0));
    let uv = vec2!(1.0, 2.0);
    assert_eq!(vec3!(uv, 3.0), vec3!(1.0, 2.0, 3.0));
    assert_eq!(vec3!(0.0, uv), vec3!(0.0, 1.0, 2.0));
    assert_eq!(vec4!(uv, 3.0, 4.0), vec4!(1.0, 2.0, 3.0, 4.0));
    assert_eq!(vec4!(0.0, uv, 4.0), vec4!(0.0, 1.0, 2.0, 4.0));
    assert_eq!(vec4!(0.0, 1.0, uv), vec4!(0.0, 1.0, 1.0, 2.0));
    assert_eq!(vec4!(uv, vec2!(5.0, 6.0)), vec4!(1.0, 2.0, 5.0, 6.0));

    let n = vec3!(7.0, 8.0, 9.0);
    assert_eq!(vec4!(n, 1.0), vec4!(7.0, 8.0, 9.0, 1.0));
    assert_eq!(vec4!(1.0, n), vec4!(1.0, 7.0, 8.0, 9.0));

    let v4 = vec4!(10.0, 20.0, 30.0, 40.0);
    assert_eq!(vec3(v4), vec3!(10.0, 20.0, 30.0));
    assert_eq!(vec2(v4), vec2!(10.0, 20.0));
    assert_eq!(vec2(n), vec2!(7.0, 8.0));

    let iv = ivec4!(-2, 0, 3, 7);
    assert_eq!(vec4(iv), vec4!(-2.0, 0.0, 3.0, 7.0));
    assert_eq!(bvec4(iv), bvec4!(true, false, true, true));
    assert_eq!(
        vec4(bvec4!(true, false, true, false)),
        vec4!(1.0, 0.0, 1.0, 0.0)
    );
    assert_eq!(ivec3(vec3!(-1.9, 0.4, 3.8)), ivec3!(-1, 0, 3));

    let hv = f16vec3!(1.5, -2.0, 0.25);
    assert_eq!(vec3(hv), vec3!(1.5, -2.0, 0.25));
    assert_eq!(dvec3(vec3!(1.0, 2.0, 3.0)), dvec3!(1.0, 2.0, 3.0));
}

#[test]
fn glsl_matrix_constructors_and_resize_rules() {
    assert_eq!(mat4(1.0), Mat4::IDENTITY);
    assert_eq!(mat3(0.0), Mat3::ZERO);

    let d3x4 = mat3x4(2.0);
    assert_eq!(d3x4.x_axis, vec4!(2.0, 0.0, 0.0, 0.0));
    assert_eq!(d3x4.y_axis, vec4!(0.0, 2.0, 0.0, 0.0));
    assert_eq!(d3x4.z_axis, vec4!(0.0, 0.0, 2.0, 0.0));

    let m3 = mat3!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    assert_eq!(m3.x_axis, vec3!(1.0, 2.0, 3.0));
    assert_eq!(m3.y_axis, vec3!(4.0, 5.0, 6.0));
    assert_eq!(m3.z_axis, vec3!(7.0, 8.0, 9.0));

    let m4 = mat4(m3);
    assert_eq!(m4.x_axis, vec4!(1.0, 2.0, 3.0, 0.0));
    assert_eq!(m4.y_axis, vec4!(4.0, 5.0, 6.0, 0.0));
    assert_eq!(m4.z_axis, vec4!(7.0, 8.0, 9.0, 0.0));
    assert_eq!(m4.w_axis, vec4!(0.0, 0.0, 0.0, 1.0));
    assert_eq!(mat3(m4), m3);

    let dm4 = dmat4(m3);
    assert_eq!(dm4.w_axis, dvec4!(0.0, 0.0, 0.0, 1.0));
    assert_eq!(dm4.x_axis, dvec4!(1.0, 2.0, 3.0, 0.0));
}

#[test]
fn quaternion_constructors_and_slice_helpers() {
    let q_id = quat_identity();
    assert_eq!(q_id, quat_wxyz(1.0, 0.0, 0.0, 0.0));
    assert_eq!(q_id, quat_xyzw(0.0, 0.0, 0.0, 1.0));

    let q_rot = angle_axis(core::f32::consts::FRAC_PI_2, vec3!(0.0, 0.0, 1.0));
    let m3 = mat3_cast(q_rot);
    let m4 = mat4_cast(q_rot);
    let q_back3 = quat_cast(m3);
    let q_back4 = quat_cast(m4);
    for i in 0..4 {
        assert!((q_rot[i] - q_back3[i]).abs() <= 1e-5);
        assert!((q_rot[i] - q_back4[i]).abs() <= 1e-5);
    }

    let mut buf = [0.0f32; 6];
    let m2x3 = mat2x3!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    write_to_slice(m2x3, &mut buf);
    assert_eq!(buf, to_array(m2x3));
    assert_eq!(from_slice::<Mat2x3>(&buf), m2x3);

    let av: aligned::vec3 = aligned::vec3!(1.0, 2.0, 3.0);
    let am: aligned::mat3 = aligned::mat3(2.0);
    assert_eq!(am * av, aligned::vec3!(2.0, 4.0, 6.0));
}
