use rglm::*;

#[unsafe(no_mangle)]
#[expect(
    unsafe_code,
    reason = "export unmangled probe symbol for xtask assembly verification"
)]
#[inline(never)]
pub fn rglm_probe_mat4_mul(a: &Mat4, b: &Mat4, out: &mut Mat4) {
    *out = *a * *b;
}

#[unsafe(no_mangle)]
#[expect(
    unsafe_code,
    reason = "export unmangled probe symbol for xtask assembly verification"
)]
#[inline(never)]
pub fn rglm_probe_mat4_mul_vec4(m: &Mat4, v: Vec4) -> Vec4 {
    *m * v
}

#[unsafe(no_mangle)]
#[expect(
    unsafe_code,
    reason = "export unmangled probe symbol for xtask assembly verification"
)]
#[inline(never)]
pub fn rglm_probe_mat3x4_mul_vec3(m: &Mat3x4, v: Vec3) -> Vec4 {
    *m * v
}

#[unsafe(no_mangle)]
#[expect(
    unsafe_code,
    reason = "export unmangled probe symbol for xtask assembly verification"
)]
#[inline(never)]
pub fn rglm_probe_vec4_mix(x: Vec4, y: Vec4, a: f32) -> Vec4 {
    mix(x, y, a)
}

#[unsafe(no_mangle)]
#[expect(
    unsafe_code,
    reason = "export unmangled probe symbol for xtask assembly verification"
)]
#[inline(never)]
pub fn rglm_probe_vec4_swizzle_blend(v: Vec4, u: Vec2) -> Vec4 {
    let mut tmp = v;
    swz!(tmp.xz) = u;
    rswz!(tmp.zyxw)
}

#[unsafe(no_mangle)]
#[expect(
    unsafe_code,
    reason = "export unmangled probe symbol for xtask assembly verification"
)]
#[inline(never)]
pub fn rglm_probe_quat_mul_vec3(q: Quat, v: Vec3A) -> Vec3A {
    q * v
}

fn main() {
    let a = core::hint::black_box(mat4(1.0));
    let b = core::hint::black_box(mat4(2.0));
    let mut out = mat4(0.0);
    rglm_probe_mat4_mul(&a, &b, &mut out);
    let v4 = rglm_probe_mat4_mul_vec4(&out, vec4!(1.0, 2.0, 3.0, 1.0));
    let m34 = core::hint::black_box(mat3x4(1.0));
    let v4b = rglm_probe_mat3x4_mul_vec3(&m34, vec3!(1.0, 2.0, 3.0));
    let mixed = rglm_probe_vec4_mix(v4, v4b, 0.5);
    let blended = rglm_probe_vec4_swizzle_blend(mixed, vec2!(4.0, 5.0));
    let rot = rglm_probe_quat_mul_vec3(quat_identity(), vec3a!(1.0, 0.0, 0.0));
    core::hint::black_box((blended, rot));
}
