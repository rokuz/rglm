use rglm::gtx::fast_math::*;
use rglm::gtx::norm::*;
use rglm::gtx::safe_ops::*;
use rglm::*;

#[test]
fn vulkan_projections_and_camera_transforms() {
    let fovy = half_pi::<f32>();
    let p_vk = perspective_vk(fovy, 1.0, 0.1, 100.0);

    let near_pt = p_vk * vec4!(0.0, 0.0, -0.1, 1.0);
    assert!((near_pt.z / near_pt.w).abs() < 1e-6);

    let far_pt = p_vk * vec4!(0.0, 0.0, -100.0, 1.0);
    assert!(((far_pt.z / far_pt.w) - 1.0).abs() < 1e-5);

    let up_pt = p_vk * vec4!(0.0, 1.0, -1.0, 1.0);
    assert!((up_pt.y / up_pt.w) < 0.0);

    let p_rev = perspective_vk_rev(fovy, 1.0, 0.1, 100.0);
    let rev_near = p_rev * vec4!(0.0, 0.0, -0.1, 1.0);
    let rev_far = p_rev * vec4!(0.0, 0.0, -100.0, 1.0);
    assert!(((rev_near.z / rev_near.w) - 1.0).abs() < 1e-6);
    assert!((rev_far.z / rev_far.w).abs() < 1e-6);

    let p_inf_rev = infinite_perspective_rh_zo_rev(fovy, 1.0, 0.5);
    let inf_near = p_inf_rev * vec4!(0.0, 0.0, -0.5, 1.0);
    assert!(((inf_near.z / inf_near.w) - 1.0).abs() < 1e-6);

    let o_vk = ortho_vk(-10.0_f32, 10.0, -5.0, 5.0, 0.0, 50.0);
    let o_top = o_vk * vec4!(0.0, 5.0, -50.0, 1.0);
    assert!((o_top.y + 1.0).abs() < 1e-6);
    assert!((o_top.z - 1.0).abs() < 1e-6);

    let view = look_at_rh(
        vec3!(0.0, 0.0, 5.0),
        vec3!(0.0, 0.0, 0.0),
        vec3!(0.0, 1.0, 0.0),
    );
    let origin_in_view = view * vec4!(0.0, 0.0, 0.0, 1.0);
    assert!(all(epsilon_equal(
        origin_in_view,
        vec4!(0.0, 0.0, -5.0, 1.0),
        1e-5
    )));

    let m = scale(
        rotate(
            translate(mat4(1.0), vec3!(1.0, 2.0, 3.0)),
            half_pi(),
            vec3!(0.0, 0.0, 1.0),
        ),
        vec3!(2.0, 2.0, 2.0),
    );
    let transformed = m * vec4!(1.0, 0.0, 0.0, 1.0);
    assert!(all(epsilon_equal(
        transformed,
        vec4!(1.0, 4.0, 3.0, 1.0),
        1e-5
    )));

    let proj = perspective_rh_zo(radians(60.0_f32), 16.0 / 9.0, 0.1, 100.0);
    let viewport = vec4!(0.0, 0.0, 1920.0, 1080.0);
    let obj = vec3!(1.5, -0.75, 2.0);
    let win = project_zo(obj, view, proj, viewport);
    let roundtrip = unproject_zo(win, view, proj, viewport);
    assert!(all(epsilon_equal(obj, roundtrip, 1e-4)));
}

#[test]
fn gtc_quaternion_and_color_space() {
    let q0 = quat_identity();
    let q1 = angle_axis(half_pi::<f32>(), vec3!(0.0, 1.0, 0.0));
    let mid = slerp(q0, q1, 0.5);
    let expected = angle_axis(quarter_pi::<f32>(), vec3!(0.0, 1.0, 0.0));
    assert!(all(epsilon_equal(mid, expected, 1e-5)));

    let c = conjugate(q1);
    let prod = q1 * c;
    assert!(all(epsilon_equal(prod, quat_identity(), 1e-5)));

    let e = euler_angles(angle_axis(0.4_f32, vec3!(1.0, 0.0, 0.0)));
    assert!((e.x - 0.4).abs() < 1e-5);

    let look_q = quat_look_at_rh(vec3!(0.0, 0.0, -1.0), vec3!(0.0, 1.0, 0.0));
    assert!(all(epsilon_equal(look_q, quat_identity(), 1e-5)));

    let lin = vec4!(0.001, 0.25, 0.8, 0.42);
    let srgb = convert_linear_to_srgb(lin);
    assert_eq!(srgb.w, 0.42);
    let back = convert_srgb_to_linear(srgb);
    assert!(all(epsilon_equal(lin, back, 1e-5)));
}

#[test]
fn gtx_safe_ops_fast_math_and_norms() {
    assert_eq!(try_normalize(vec3(0.0)), None);
    assert_eq!(normalize_or_zero(vec3(0.0)), vec3(0.0));
    assert_eq!(
        normalize_or(vec3(0.0), vec3!(0.0, 1.0, 0.0)),
        vec3!(0.0, 1.0, 0.0)
    );
    assert!(try_normalize(vec3!(3.0, 4.0, 0.0)).is_some());

    assert_eq!(try_inverse(mat4(0.0)), None);
    assert_eq!(try_inverse(Quat::ZERO), None);
    assert!(try_inverse(mat4(2.0)).is_some());

    assert_eq!(clamp_length(vec3(0.0), 1.0, 5.0), vec3(0.0));
    let cl = clamp_length(vec3!(10.0, 0.0, 0.0), 1.0, 4.0);
    assert!((length(cl) - 4.0).abs() < 1e-5);

    let fn_v = fast_normalize(vec3!(3.0, 4.0, 0.0));
    assert!((length(fn_v) - 1.0).abs() < 5e-3);
    assert!((fast_sin(0.5_f32) - sin(0.5_f32)).abs() < 2e-3);
    assert!((fast_cos(0.5_f32) - cos(0.5_f32)).abs() < 2e-3);
    assert!((fast_exp(1.0_f32) - euler::<f32>()).abs() < 5e-2);
    assert!((fast_log(euler::<f32>()) - 1.0).abs() < 5e-2);

    let v = vec3!(-3.0, 4.0, 0.0);
    assert_eq!(length2(v), 25.0);
    assert_eq!(distance2(v, vec3(0.0)), 25.0);
    assert_eq!(l1_norm(v), 7.0);
    assert_eq!(l2_norm(v), 5.0);
    assert_eq!(l_max_norm(v), 4.0);
    assert!((lx_norm(vec3!(3.0, 4.0, 0.0), 2) - 5.0).abs() < 1e-5);
}
