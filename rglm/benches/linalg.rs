use core::hint::black_box;
use std::time::Instant;

const BATCH: usize = 256;
const WARMUP: usize = 64;
const ROUNDS: usize = 1500;
const SAMPLES: usize = 15;

fn measure_ns_per_op<F: FnMut()>(full_run: bool, mut f: F) -> f64 {
    if !full_run {
        f();
        let start = Instant::now();
        f();
        return (start.elapsed().as_nanos() as f64) / (BATCH as f64);
    }

    for _ in 0..WARMUP {
        f();
    }
    let mut times = [0.0_f64; SAMPLES];
    for slot in &mut times {
        let start = Instant::now();
        for _ in 0..ROUNDS {
            f();
        }
        let ns = start.elapsed().as_nanos() as f64;
        *slot = ns / ((ROUNDS * BATCH) as f64);
    }
    times.sort_by(f64::total_cmp);
    times[SAMPLES / 2]
}

fn bench_rglm(full_run: bool) -> Vec<(&'static str, f64)> {
    use rglm::*;

    let mut mats_a = Vec::with_capacity(BATCH);
    let mut mats_b = Vec::with_capacity(BATCH);
    let mut mats_chain = Vec::with_capacity(BATCH);
    let mut mats3 = Vec::with_capacity(BATCH);
    let mut vecs4_a = Vec::with_capacity(BATCH);
    let mut vecs4_b = Vec::with_capacity(BATCH);
    let mut vecs3 = Vec::with_capacity(BATCH);
    let mut eyes = Vec::with_capacity(BATCH);
    let mut centers = Vec::with_capacity(BATCH);
    let mut fovs = Vec::with_capacity(BATCH);
    let mut vecs3a_a = Vec::with_capacity(BATCH);
    let mut vecs3a_b = Vec::with_capacity(BATCH);
    let mut quats_a = Vec::with_capacity(BATCH);
    let mut quats_b = Vec::with_capacity(BATCH);

    for i in 0..BATCH {
        let f = (i as f32) * 0.01 + 0.1;
        let axis = normalize(vec3!(1.0, 2.0, 3.0));
        let m = rotate(translate(mat4(1.0), vec3!(f, -f * 0.5, f * 2.0)), f, axis);
        let mb = transpose(m) + mat4(2.0);
        mats_a.push(m);
        mats_b.push(mb);
        mats_chain.push(rotate(mat4(1.0), 0.005 + f * 0.001, axis));
        mats3.push(mat3(mb));
        vecs4_a.push(vec4!(f, f * 0.5, -f * 0.3, 1.0));
        vecs4_b.push(vec4!(0.4 + f * 0.1, 0.7 - f * 0.05, -0.5 + f * 0.08, 0.9));
        vecs3.push(vec3!(f, -f * 0.7, 1.5));
        eyes.push(vec3!(f * 2.0, 1.5 + f * 0.3, 5.0 + f));
        centers.push(vec3!(-f * 0.2, 0.5, 0.0));
        fovs.push(0.8 + ((i % 16) as f32) * 0.02);
        vecs3a_a.push(vec3a!(f + 0.2, -f * 0.7 - 0.1, 1.5));
        vecs3a_b.push(vec3a!(-0.6, 1.1 + f * 0.2, 0.4 - f * 0.1));
        quats_a.push(angle_axis(f, normalize(vec3!(0.3, 0.8, -0.5))));
        quats_b.push(angle_axis(f * 0.7 + 0.2, normalize(vec3!(-0.6, 0.2, 0.7))));
    }

    let mut out_m4 = vec![mat4(0.0); BATCH];
    let mut out_m3 = vec![mat3(0.0); BATCH];
    let mut out_v4 = vec![vec4(0.0); BATCH];
    let mut out_v3 = vec![vec3(0.0); BATCH];
    let mut out_v3a = vec![vec3a(0.0); BATCH];
    let mut out_q = vec![quat_identity(); BATCH];
    let mut out_f = vec![0.0_f32; BATCH];

    vec![
        (
            "mat4_mul_mat4",
            measure_ns_per_op(full_run, || {
                let a = black_box(mats_a.as_slice());
                let b = black_box(mats_b.as_slice());
                let out = out_m4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = a[i] * b[i];
                }
                black_box(out);
            }),
        ),
        (
            "mat4_chain",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_chain.as_slice());
                let mut acc = m[0];
                for item in m.iter().take(BATCH) {
                    acc *= *item;
                }
                black_box(acc);
            }),
        ),
        (
            "mat4_mul_vec4",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_a.as_slice());
                let v = black_box(vecs4_a.as_slice());
                let out = out_v4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = m[i] * v[i];
                }
                black_box(out);
            }),
        ),
        (
            "mat4_inverse",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_b.as_slice());
                let out = out_m4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = inverse(m[i]);
                }
                black_box(out);
            }),
        ),
        (
            "mat4_determinant",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_b.as_slice());
                let out = out_f.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = determinant(m[i]);
                }
                black_box(out);
            }),
        ),
        (
            "mat3_inverse",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats3.as_slice());
                let out = out_m3.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = inverse(m[i]);
                }
                black_box(out);
            }),
        ),
        (
            "quat_mul_quat",
            measure_ns_per_op(full_run, || {
                let a = black_box(quats_a.as_slice());
                let b = black_box(quats_b.as_slice());
                let out = out_q.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = a[i] * b[i];
                }
                black_box(out);
            }),
        ),
        (
            "quat_mul_vec3",
            measure_ns_per_op(full_run, || {
                let q = black_box(quats_a.as_slice());
                let v = black_box(vecs3.as_slice());
                let out = out_v3.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = q[i] * v[i];
                }
                black_box(out);
            }),
        ),
        (
            "quat_mul_vec3a",
            measure_ns_per_op(full_run, || {
                let q = black_box(quats_a.as_slice());
                let v = black_box(vecs3a_a.as_slice());
                let out = out_v3a.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = q[i] * v[i];
                }
                black_box(out);
            }),
        ),
        (
            "vec3a_geom",
            measure_ns_per_op(full_run, || {
                let a = black_box(vecs3a_a.as_slice());
                let b = black_box(vecs3a_b.as_slice());
                let out = out_v3a.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = normalize(cross(a[i], b[i]) + a[i] * dot(a[i], b[i]));
                }
                black_box(out);
            }),
        ),
        (
            "camera_vp",
            measure_ns_per_op(full_run, || {
                let e = black_box(eyes.as_slice());
                let c = black_box(centers.as_slice());
                let f = black_box(fovs.as_slice());
                let out = out_m4.as_mut_slice();
                let up = vec3!(0.0, 1.0, 0.0);
                for i in 0..BATCH {
                    let proj = perspective_rh_zo(f[i], 16.0 / 9.0, 0.1, 1000.0);
                    let view = look_at_rh(e[i], c[i], up);
                    out[i] = proj * view;
                }
                black_box(out);
            }),
        ),
        (
            "vec4_sin_cos",
            measure_ns_per_op(full_run, || {
                let v = black_box(vecs4_a.as_slice());
                let out = out_v4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = sin(v[i]) + cos(v[i]);
                }
                black_box(out);
            }),
        ),
        (
            "vec4_exp",
            measure_ns_per_op(full_run, || {
                let v = black_box(vecs4_b.as_slice());
                let out = out_v4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = exp(v[i]);
                }
                black_box(out);
            }),
        ),
    ]
}

fn bench_glam(full_run: bool) -> Vec<(&'static str, f64)> {
    use glam::{Mat3, Mat4, Quat, Vec3, Vec3A, Vec4};

    let mut mats_a = Vec::with_capacity(BATCH);
    let mut mats_b = Vec::with_capacity(BATCH);
    let mut mats_chain = Vec::with_capacity(BATCH);
    let mut mats3 = Vec::with_capacity(BATCH);
    let mut vecs4_a = Vec::with_capacity(BATCH);
    let mut vecs4_b = Vec::with_capacity(BATCH);
    let mut vecs3 = Vec::with_capacity(BATCH);
    let mut eyes = Vec::with_capacity(BATCH);
    let mut centers = Vec::with_capacity(BATCH);
    let mut fovs = Vec::with_capacity(BATCH);
    let mut vecs3a_a = Vec::with_capacity(BATCH);
    let mut vecs3a_b = Vec::with_capacity(BATCH);
    let mut quats_a = Vec::with_capacity(BATCH);
    let mut quats_b = Vec::with_capacity(BATCH);

    for i in 0..BATCH {
        let f = (i as f32) * 0.01 + 0.1;
        let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
        let m = Mat4::from_translation(Vec3::new(f, -f * 0.5, f * 2.0))
            * Mat4::from_axis_angle(axis, f);
        let mb = m.transpose() + Mat4::from_diagonal(Vec4::splat(2.0));
        mats_a.push(m);
        mats_b.push(mb);
        mats_chain.push(Mat4::from_axis_angle(axis, 0.005 + f * 0.001));
        mats3.push(Mat3::from_mat4(mb));
        vecs4_a.push(Vec4::new(f, f * 0.5, -f * 0.3, 1.0));
        vecs4_b.push(Vec4::new(
            0.4 + f * 0.1,
            0.7 - f * 0.05,
            -0.5 + f * 0.08,
            0.9,
        ));
        vecs3.push(Vec3::new(f, -f * 0.7, 1.5));
        eyes.push(Vec3::new(f * 2.0, 1.5 + f * 0.3, 5.0 + f));
        centers.push(Vec3::new(-f * 0.2, 0.5, 0.0));
        fovs.push(0.8 + ((i % 16) as f32) * 0.02);
        vecs3a_a.push(Vec3A::new(f + 0.2, -f * 0.7 - 0.1, 1.5));
        vecs3a_b.push(Vec3A::new(-0.6, 1.1 + f * 0.2, 0.4 - f * 0.1));
        quats_a.push(Quat::from_axis_angle(
            Vec3::new(0.3, 0.8, -0.5).normalize(),
            f,
        ));
        quats_b.push(Quat::from_axis_angle(
            Vec3::new(-0.6, 0.2, 0.7).normalize(),
            f * 0.7 + 0.2,
        ));
    }

    let mut out_m4 = vec![Mat4::ZERO; BATCH];
    let mut out_m3 = vec![Mat3::ZERO; BATCH];
    let mut out_v4 = vec![Vec4::ZERO; BATCH];
    let mut out_v3 = vec![Vec3::ZERO; BATCH];
    let mut out_v3a = vec![Vec3A::ZERO; BATCH];
    let mut out_q = vec![Quat::IDENTITY; BATCH];
    let mut out_f = vec![0.0_f32; BATCH];

    vec![
        (
            "mat4_mul_mat4",
            measure_ns_per_op(full_run, || {
                let a = black_box(mats_a.as_slice());
                let b = black_box(mats_b.as_slice());
                let out = out_m4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = a[i] * b[i];
                }
                black_box(out);
            }),
        ),
        (
            "mat4_chain",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_chain.as_slice());
                let mut acc = m[0];
                for item in m.iter().take(BATCH) {
                    acc *= *item;
                }
                black_box(acc);
            }),
        ),
        (
            "mat4_mul_vec4",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_a.as_slice());
                let v = black_box(vecs4_a.as_slice());
                let out = out_v4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = m[i] * v[i];
                }
                black_box(out);
            }),
        ),
        (
            "mat4_inverse",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_b.as_slice());
                let out = out_m4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = m[i].inverse();
                }
                black_box(out);
            }),
        ),
        (
            "mat4_determinant",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats_b.as_slice());
                let out = out_f.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = m[i].determinant();
                }
                black_box(out);
            }),
        ),
        (
            "mat3_inverse",
            measure_ns_per_op(full_run, || {
                let m = black_box(mats3.as_slice());
                let out = out_m3.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = m[i].inverse();
                }
                black_box(out);
            }),
        ),
        (
            "quat_mul_quat",
            measure_ns_per_op(full_run, || {
                let a = black_box(quats_a.as_slice());
                let b = black_box(quats_b.as_slice());
                let out = out_q.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = a[i] * b[i];
                }
                black_box(out);
            }),
        ),
        (
            "quat_mul_vec3",
            measure_ns_per_op(full_run, || {
                let q = black_box(quats_a.as_slice());
                let v = black_box(vecs3.as_slice());
                let out = out_v3.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = q[i] * v[i];
                }
                black_box(out);
            }),
        ),
        (
            "quat_mul_vec3a",
            measure_ns_per_op(full_run, || {
                let q = black_box(quats_a.as_slice());
                let v = black_box(vecs3a_a.as_slice());
                let out = out_v3a.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = q[i] * v[i];
                }
                black_box(out);
            }),
        ),
        (
            "vec3a_geom",
            measure_ns_per_op(full_run, || {
                let a = black_box(vecs3a_a.as_slice());
                let b = black_box(vecs3a_b.as_slice());
                let out = out_v3a.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = (a[i].cross(b[i]) + a[i] * a[i].dot(b[i])).normalize();
                }
                black_box(out);
            }),
        ),
        (
            "camera_vp",
            measure_ns_per_op(full_run, || {
                let e = black_box(eyes.as_slice());
                let c = black_box(centers.as_slice());
                let f = black_box(fovs.as_slice());
                let out = out_m4.as_mut_slice();
                let up = Vec3::new(0.0, 1.0, 0.0);
                for i in 0..BATCH {
                    let proj =
                        glam::camera::rh::proj::directx::perspective(f[i], 16.0 / 9.0, 0.1, 1000.0);
                    let view = glam::camera::rh::view::look_at_mat4(e[i], c[i], up);
                    out[i] = proj * view;
                }
                black_box(out);
            }),
        ),
        (
            "vec4_sin_cos",
            measure_ns_per_op(full_run, || {
                let v = black_box(vecs4_a.as_slice());
                let out = out_v4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = v[i].sin() + v[i].cos();
                }
                black_box(out);
            }),
        ),
        (
            "vec4_exp",
            measure_ns_per_op(full_run, || {
                let v = black_box(vecs4_b.as_slice());
                let out = out_v4.as_mut_slice();
                for i in 0..BATCH {
                    out[i] = v[i].exp();
                }
                black_box(out);
            }),
        ),
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let full_run = args.iter().any(|a| a == "--bench" || a == "--run-bench");
    let engine = args
        .windows(2)
        .find(|w| w[0] == "--engine")
        .map(|w| w[1].as_str())
        .unwrap_or("rglm");

    let results = match engine {
        "glam" => bench_glam(full_run),
        _ => bench_rglm(full_run),
    };

    if full_run {
        for (name, ns) in results {
            println!("{name}\t{ns:.3}");
        }
    }
}
