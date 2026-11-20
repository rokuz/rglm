use rglm::*;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

struct OracleClient {
    child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    buf: Vec<u8>,
}

impl OracleClient {
    fn try_spawn() -> Result<Option<Self>, String> {
        let required = std::env::var_os("RGLM_REQUIRE_ORACLE").is_some();
        let path = if let Some(p) = std::env::var_os("RGLM_GLM_ORACLE_BIN") {
            PathBuf::from(p)
        } else {
            let mut fallback = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            fallback.pop();
            let name = if cfg!(windows) {
                "glm_oracle.exe"
            } else {
                "glm_oracle"
            };
            fallback.join("target/oracles/bin").join(name)
        };

        if !path.exists() {
            if required {
                return Err(format!(
                    "missing C++ GLM oracle binary at {}",
                    path.display()
                ));
            }
            return Ok(None);
        }

        let mut child = Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to spawn {}: {e}", path.display()))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| "failed to capture oracle stdin".to_string())?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "failed to capture oracle stdout".to_string())?;

        Ok(Some(Self {
            child,
            stdin: BufWriter::new(stdin),
            stdout: BufReader::new(stdout),
            buf: Vec::with_capacity(256),
        }))
    }

    fn call(&mut self, op: u32, payload: &[u8]) -> Result<&[u8], String> {
        self.stdin
            .write_all(&op.to_le_bytes())
            .map_err(|e| e.to_string())?;
        let len = payload.len() as u32;
        self.stdin
            .write_all(&len.to_le_bytes())
            .map_err(|e| e.to_string())?;
        self.stdin.write_all(payload).map_err(|e| e.to_string())?;
        self.stdin.flush().map_err(|e| e.to_string())?;

        let mut len_bytes = [0u8; 4];
        self.stdout
            .read_exact(&mut len_bytes)
            .map_err(|e| format!("oracle read failed on op {op}: {e}"))?;
        let out_len = u32::from_le_bytes(len_bytes) as usize;
        self.buf.resize(out_len, 0);
        self.stdout
            .read_exact(&mut self.buf)
            .map_err(|e| format!("oracle payload read failed on op {op}: {e}"))?;
        Ok(&self.buf)
    }

    fn finish(mut self) -> Result<(), String> {
        let _ = self.stdin.write_all(&0u32.to_le_bytes());
        let _ = self.stdin.write_all(&0u32.to_le_bytes());
        let _ = self.stdin.flush();
        drop(self.stdin);
        let status = self.child.wait().map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("oracle exited with {status}"));
        }
        Ok(())
    }
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 32) as u32
    }

    fn f32_range(&mut self, lo: f32, hi: f32) -> f32 {
        let t = (self.next_u32() as f64) / (u32::MAX as f64);
        lo + (hi - lo) * (t as f32)
    }

    fn vec2_range(&mut self, lo: f32, hi: f32) -> Vec2 {
        vec2!(self.f32_range(lo, hi), self.f32_range(lo, hi))
    }

    fn vec3_range(&mut self, lo: f32, hi: f32) -> Vec3 {
        vec3!(
            self.f32_range(lo, hi),
            self.f32_range(lo, hi),
            self.f32_range(lo, hi)
        )
    }

    fn vec4_range(&mut self, lo: f32, hi: f32) -> Vec4 {
        vec4!(
            self.f32_range(lo, hi),
            self.f32_range(lo, hi),
            self.f32_range(lo, hi),
            self.f32_range(lo, hi)
        )
    }
}

fn push_f32s(buf: &mut Vec<u8>, vals: &[f32]) {
    for &v in vals {
        buf.extend_from_slice(&v.to_le_bytes());
    }
}

fn push_f64s(buf: &mut Vec<u8>, vals: &[f64]) {
    for &v in vals {
        buf.extend_from_slice(&v.to_le_bytes());
    }
}

fn push_i32s(buf: &mut Vec<u8>, vals: &[i32]) {
    for &v in vals {
        buf.extend_from_slice(&v.to_le_bytes());
    }
}

fn push_u32s(buf: &mut Vec<u8>, vals: &[u32]) {
    for &v in vals {
        buf.extend_from_slice(&v.to_le_bytes());
    }
}

fn read_f32s<const N: usize>(bytes: &[u8]) -> Result<[f32; N], String> {
    if bytes.len() != N * 4 {
        return Err(format!("expected {} bytes, got {}", N * 4, bytes.len()));
    }
    let mut out = [0.0f32; N];
    for (i, slot) in out.iter_mut().enumerate() {
        let mut b = [0u8; 4];
        b.copy_from_slice(&bytes[i * 4..i * 4 + 4]);
        *slot = f32::from_le_bytes(b);
    }
    Ok(out)
}

fn read_f64s<const N: usize>(bytes: &[u8]) -> Result<[f64; N], String> {
    if bytes.len() != N * 8 {
        return Err(format!("expected {} bytes, got {}", N * 8, bytes.len()));
    }
    let mut out = [0.0f64; N];
    for (i, slot) in out.iter_mut().enumerate() {
        let mut b = [0u8; 8];
        b.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
        *slot = f64::from_le_bytes(b);
    }
    Ok(out)
}

fn read_i32s<const N: usize>(bytes: &[u8]) -> Result<[i32; N], String> {
    if bytes.len() != N * 4 {
        return Err(format!("expected {} bytes, got {}", N * 4, bytes.len()));
    }
    let mut out = [0i32; N];
    for (i, slot) in out.iter_mut().enumerate() {
        let mut b = [0u8; 4];
        b.copy_from_slice(&bytes[i * 4..i * 4 + 4]);
        *slot = i32::from_le_bytes(b);
    }
    Ok(out)
}

fn read_u32s<const N: usize>(bytes: &[u8]) -> Result<[u32; N], String> {
    if bytes.len() != N * 4 {
        return Err(format!("expected {} bytes, got {}", N * 4, bytes.len()));
    }
    let mut out = [0u32; N];
    for (i, slot) in out.iter_mut().enumerate() {
        let mut b = [0u8; 4];
        b.copy_from_slice(&bytes[i * 4..i * 4 + 4]);
        *slot = u32::from_le_bytes(b);
    }
    Ok(out)
}

fn ulp_diff_f32(a: f32, b: f32) -> f64 {
    if a.is_nan() && b.is_nan() {
        return 0.0;
    }
    if a.is_infinite() && b.is_infinite() && a.signum() == b.signum() {
        return 0.0;
    }
    if a == 0.0 && b == 0.0 {
        return 0.0;
    }
    if a.is_nan() || b.is_nan() || a.is_infinite() || b.is_infinite() {
        return f64::INFINITY;
    }
    let ref_val = b as f64;
    let err = ((a as f64) - ref_val).abs();
    let bits = b.abs().to_bits();
    let raw_exp = ((bits >> 23) & 0xff) as i32;
    let ulp = if raw_exp == 0 {
        f64::exp2(-149.0)
    } else {
        f64::exp2((raw_exp - 127 - 23) as f64)
    };
    err / ulp
}

fn assert_f32_slice_ulp(label: &str, got: &[f32], exp: &[f32], max_ulp: f64, abs_floor: f32) {
    assert_eq!(got.len(), exp.len(), "{label}: length mismatch");
    for i in 0..got.len() {
        let g = got[i];
        let e = exp[i];
        if g.is_nan() && e.is_nan() {
            continue;
        }
        if abs_floor > 0.0 && (g - e).abs() <= abs_floor {
            continue;
        }
        let ulp = ulp_diff_f32(g, e);
        assert!(
            ulp <= max_ulp,
            "{label}[{i}]: rglm={g:?} ({:#010x}) vs glm={e:?} ({:#010x}), diff={ulp:.3} ULP > {max_ulp}",
            g.to_bits(),
            e.to_bits()
        );
    }
}

#[test]
fn differential_common_and_ieee_edges() -> Result<(), String> {
    let Some(mut oracle) = OracleClient::try_spawn()? else {
        return Ok(());
    };

    let edge_scalars: [f32; 20] = [
        0.0,
        -0.0,
        1.0e-41,
        -1.0e-41,
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        0.5,
        -0.5,
        1.5,
        -1.5,
        2.5,
        -2.5,
        3.5,
        -3.5,
        0.25,
        -0.75,
        123.456,
        -123.456,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];

    for chunk in edge_scalars.chunks(4) {
        let v = vec4!(chunk[0], chunk[1], chunk[2], chunk[3]);
        let mut p = Vec::new();
        push_f32s(&mut p, &to_array(v));

        let exp_abs = read_f32s::<4>(oracle.call(1, &p)?)?;
        assert_f32_slice_ulp("abs", &to_array(abs(v)), &exp_abs, 0.0, 0.0);

        let exp_sign = read_f32s::<4>(oracle.call(2, &p)?)?;
        assert_f32_slice_ulp("sign", &to_array(sign(v)), &exp_sign, 0.0, 0.0);

        if v.x.is_finite() && v.y.is_finite() && v.z.is_finite() && v.w.is_finite() {
            let exp_floor = read_f32s::<4>(oracle.call(3, &p)?)?;
            assert_f32_slice_ulp("floor", &to_array(floor(v)), &exp_floor, 0.0, 0.0);

            let exp_trunc = read_f32s::<4>(oracle.call(4, &p)?)?;
            assert_f32_slice_ulp("trunc", &to_array(trunc(v)), &exp_trunc, 0.0, 0.0);

            let exp_round = read_f32s::<4>(oracle.call(5, &p)?)?;
            assert_f32_slice_ulp("round", &to_array(round(v)), &exp_round, 0.0, 0.0);

            let exp_round_even = read_f32s::<4>(oracle.call(6, &p)?)?;
            assert_f32_slice_ulp(
                "round_even",
                &to_array(round_even(v)),
                &exp_round_even,
                0.0,
                0.0,
            );

            let exp_ceil = read_f32s::<4>(oracle.call(7, &p)?)?;
            assert_f32_slice_ulp("ceil", &to_array(ceil(v)), &exp_ceil, 0.0, 0.0);

            let exp_fract = read_f32s::<4>(oracle.call(8, &p)?)?;
            assert_f32_slice_ulp("fract", &to_array(fract(v)), &exp_fract, 0.0, 0.0);
        }
    }

    let nan_vec = vec4!(f32::NAN, -0.0, 0.0, -42.0);
    assert_eq!(sign(nan_vec), vec4!(0.0, 0.0, 0.0, -1.0));
    assert!(sign(nan_vec).y.to_bits() == 0);

    let mut rng = Rng::new(0x00C0_FFEE_1234);
    for _ in 0..256 {
        let a = rng.vec4_range(-50.0, 50.0);
        let b_raw = rng.vec4_range(0.25, 15.0);
        let sign_flip = vec4!(1.0, -1.0, 1.0, -1.0);
        let b = b_raw * sign_flip;
        let t = rng.vec4_range(0.0, 1.0);

        let mut p2 = Vec::new();
        push_f32s(&mut p2, &to_array(a));
        push_f32s(&mut p2, &to_array(b));
        assert_f32_slice_ulp(
            "modulo",
            &to_array(modulo(a, b)),
            &read_f32s::<4>(oracle.call(9, &p2)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "min",
            &to_array(min(a, b)),
            &read_f32s::<4>(oracle.call(10, &p2)?)?,
            0.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "max",
            &to_array(max(a, b)),
            &read_f32s::<4>(oracle.call(11, &p2)?)?,
            0.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "step",
            &to_array(step(a, b)),
            &read_f32s::<4>(oracle.call(14, &p2)?)?,
            0.0,
            0.0,
        );

        let lo = min(a, b);
        let hi = max(a, b) + vec4(0.01);
        let mut p_clamp = Vec::new();
        push_f32s(&mut p_clamp, &to_array(t));
        push_f32s(&mut p_clamp, &to_array(lo));
        push_f32s(&mut p_clamp, &to_array(hi));
        assert_f32_slice_ulp(
            "clamp",
            &to_array(clamp(t, lo, hi)),
            &read_f32s::<4>(oracle.call(12, &p_clamp)?)?,
            0.0,
            0.0,
        );

        let mut p_mix = Vec::new();
        push_f32s(&mut p_mix, &to_array(a));
        push_f32s(&mut p_mix, &to_array(b));
        push_f32s(&mut p_mix, &to_array(t));
        assert_f32_slice_ulp(
            "mix",
            &to_array(mix(a, b, t)),
            &read_f32s::<4>(oracle.call(13, &p_mix)?)?,
            1.5,
            2.0 * f32::EPSILON * 50.0,
        );
        let fma_exact = [
            ((a.x as f64) * (b.x as f64) + (t.x as f64)) as f32,
            ((a.y as f64) * (b.y as f64) + (t.y as f64)) as f32,
            ((a.z as f64) * (b.z as f64) + (t.z as f64)) as f32,
            ((a.w as f64) * (b.w as f64) + (t.w as f64)) as f32,
        ];
        assert_f32_slice_ulp("fma_ieee", &to_array(fma(a, b, t)), &fma_exact, 0.5, 0.0);
        assert_f32_slice_ulp(
            "fma_glm",
            &to_array(fma(a, b, t)),
            &read_f32s::<4>(oracle.call(16, &p_mix)?)?,
            1.5,
            2.0 * f32::EPSILON * 50.0,
        );

        let unit_a = rng.vec4_range(0.2, 1.0);
        let unit_b = rng.vec4_range(0.2, 1.0);
        let mut p_mix_u = Vec::new();
        push_f32s(&mut p_mix_u, &to_array(unit_a));
        push_f32s(&mut p_mix_u, &to_array(unit_b));
        push_f32s(&mut p_mix_u, &to_array(t));
        assert_f32_slice_ulp(
            "mix_same_sign",
            &to_array(mix(unit_a, unit_b, t)),
            &read_f32s::<4>(oracle.call(13, &p_mix_u)?)?,
            1.5,
            0.0,
        );

        let mut p_smooth = Vec::new();
        push_f32s(&mut p_smooth, &to_array(lo));
        push_f32s(&mut p_smooth, &to_array(hi));
        push_f32s(&mut p_smooth, &to_array(a));
        assert_f32_slice_ulp(
            "smoothstep",
            &to_array(smoothstep(lo, hi, a)),
            &read_f32s::<4>(oracle.call(15, &p_smooth)?)?,
            2.0,
            1e-7,
        );
    }

    oracle.finish()
}

#[test]
fn differential_transcendentals_and_geometric() -> Result<(), String> {
    let Some(mut oracle) = OracleClient::try_spawn()? else {
        return Ok(());
    };

    let mut rng = Rng::new(0x515D_BEEF);
    for _ in 0..256 {
        let deg = rng.vec4_range(-720.0, 720.0);
        let mut p_deg = Vec::new();
        push_f32s(&mut p_deg, &to_array(deg));
        assert_f32_slice_ulp(
            "radians",
            &to_array(radians(deg)),
            &read_f32s::<4>(oracle.call(20, &p_deg)?)?,
            1.0,
            0.0,
        );

        let ang = rng.vec4_range(-12.0, 12.0);
        let mut p_ang = Vec::new();
        push_f32s(&mut p_ang, &to_array(ang));
        assert_f32_slice_ulp(
            "degrees",
            &to_array(degrees(ang)),
            &read_f32s::<4>(oracle.call(21, &p_ang)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "sin",
            &to_array(sin(ang)),
            &read_f32s::<4>(oracle.call(22, &p_ang)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "cos",
            &to_array(cos(ang)),
            &read_f32s::<4>(oracle.call(23, &p_ang)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "atan",
            &to_array(atan(ang)),
            &read_f32s::<4>(oracle.call(27, &p_ang)?)?,
            1.5,
            0.0,
        );

        let tan_in = rng.vec4_range(-1.35, 1.35);
        let mut p_tan = Vec::new();
        push_f32s(&mut p_tan, &to_array(tan_in));
        assert_f32_slice_ulp(
            "tan",
            &to_array(tan(tan_in)),
            &read_f32s::<4>(oracle.call(24, &p_tan)?)?,
            1.5,
            0.0,
        );

        let unit = rng.vec4_range(-0.9999, 0.9999);
        let mut p_unit = Vec::new();
        push_f32s(&mut p_unit, &to_array(unit));
        assert_f32_slice_ulp(
            "asin",
            &to_array(asin(unit)),
            &read_f32s::<4>(oracle.call(25, &p_unit)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "acos",
            &to_array(acos(unit)),
            &read_f32s::<4>(oracle.call(26, &p_unit)?)?,
            1.5,
            0.0,
        );

        let atanh_in = clamp(unit, vec4(-0.95), vec4(0.95));
        let mut p_atanh = Vec::new();
        push_f32s(&mut p_atanh, &to_array(atanh_in));
        assert_f32_slice_ulp(
            "atanh",
            &to_array(atanh(atanh_in)),
            &read_f32s::<4>(oracle.call(34, &p_atanh)?)?,
            2.0,
            0.0,
        );

        let y2 = rng.vec4_range(-10.0, 10.0);
        let x2 = rng.vec4_range(-10.0, 10.0);
        let mut p_atan2 = Vec::new();
        push_f32s(&mut p_atan2, &to_array(y2));
        push_f32s(&mut p_atan2, &to_array(x2));
        assert_f32_slice_ulp(
            "atan2",
            &to_array(atan2(y2, x2)),
            &read_f32s::<4>(oracle.call(28, &p_atan2)?)?,
            1.5,
            0.0,
        );

        let hyp = rng.vec4_range(-4.0, 4.0);
        let mut p_hyp = Vec::new();
        push_f32s(&mut p_hyp, &to_array(hyp));
        assert_f32_slice_ulp(
            "sinh",
            &to_array(sinh(hyp)),
            &read_f32s::<4>(oracle.call(29, &p_hyp)?)?,
            2.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "cosh",
            &to_array(cosh(hyp)),
            &read_f32s::<4>(oracle.call(30, &p_hyp)?)?,
            2.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "tanh",
            &to_array(tanh(hyp)),
            &read_f32s::<4>(oracle.call(31, &p_hyp)?)?,
            2.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "asinh",
            &to_array(asinh(hyp)),
            &read_f32s::<4>(oracle.call(32, &p_hyp)?)?,
            2.0,
            0.0,
        );

        let ge1 = rng.vec4_range(1.001, 25.0);
        let mut p_ge1 = Vec::new();
        push_f32s(&mut p_ge1, &to_array(ge1));
        assert_f32_slice_ulp(
            "acosh",
            &to_array(acosh(ge1)),
            &read_f32s::<4>(oracle.call(33, &p_ge1)?)?,
            2.0,
            0.0,
        );

        let exp_in = rng.vec4_range(-20.0, 20.0);
        let mut p_exp = Vec::new();
        push_f32s(&mut p_exp, &to_array(exp_in));
        assert_f32_slice_ulp(
            "exp",
            &to_array(exp(exp_in)),
            &read_f32s::<4>(oracle.call(36, &p_exp)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "exp2",
            &to_array(exp2(exp_in)),
            &read_f32s::<4>(oracle.call(38, &p_exp)?)?,
            1.5,
            0.0,
        );

        let pos = rng.vec4_range(0.001, 500.0);
        let mut p_pos = Vec::new();
        push_f32s(&mut p_pos, &to_array(pos));
        assert_f32_slice_ulp(
            "log",
            &to_array(log(pos)),
            &read_f32s::<4>(oracle.call(37, &p_pos)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "log2",
            &to_array(log2(pos)),
            &read_f32s::<4>(oracle.call(39, &p_pos)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "sqrt",
            &to_array(sqrt(pos)),
            &read_f32s::<4>(oracle.call(40, &p_pos)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "inversesqrt",
            &to_array(inversesqrt(pos)),
            &read_f32s::<4>(oracle.call(41, &p_pos)?)?,
            1.5,
            0.0,
        );

        let base = rng.vec4_range(0.1, 8.0);
        let expo = rng.vec4_range(-3.0, 3.0);
        let mut p_pow = Vec::new();
        push_f32s(&mut p_pow, &to_array(base));
        push_f32s(&mut p_pow, &to_array(expo));
        assert_f32_slice_ulp(
            "pow",
            &to_array(pow(base, expo)),
            &read_f32s::<4>(oracle.call(35, &p_pow)?)?,
            2.0,
            0.0,
        );

        let va = rng.vec3_range(-10.0, 10.0) + vec3!(0.1, -0.2, 0.3);
        let vb = rng.vec3_range(-10.0, 10.0) + vec3!(-0.3, 0.1, 0.2);
        let mut p_va = Vec::new();
        push_f32s(&mut p_va, &to_array(va));
        assert_f32_slice_ulp(
            "length",
            &[length(va)],
            &read_f32s::<1>(oracle.call(50, &p_va)?)?,
            1.5,
            0.0,
        );
        assert_f32_slice_ulp(
            "normalize",
            &to_array(normalize(va)),
            &read_f32s::<3>(oracle.call(54, &p_va)?)?,
            2.0,
            1e-7,
        );

        let mut p_vab = Vec::new();
        push_f32s(&mut p_vab, &to_array(va));
        push_f32s(&mut p_vab, &to_array(vb));
        assert_f32_slice_ulp(
            "distance",
            &[distance(va, vb)],
            &read_f32s::<1>(oracle.call(51, &p_vab)?)?,
            2.0,
            1e-7,
        );
        assert_f32_slice_ulp(
            "cross",
            &to_array(cross(va, vb)),
            &read_f32s::<3>(oracle.call(53, &p_vab)?)?,
            2.0,
            2.0 * f32::EPSILON * 100.0,
        );

        let mut p_dot = Vec::new();
        push_f32s(&mut p_dot, &to_array(y2));
        push_f32s(&mut p_dot, &to_array(x2));
        assert_f32_slice_ulp(
            "dot4",
            &[dot(y2, x2)],
            &read_f32s::<1>(oracle.call(52, &p_dot)?)?,
            2.0,
            2.0 * f32::EPSILON * 200.0,
        );

        let n_unit = normalize(vb);
        let i_unit = normalize(va);
        let nref = rng.vec3_range(-5.0, 5.0);
        let mut p_ff = Vec::new();
        push_f32s(&mut p_ff, &to_array(n_unit));
        push_f32s(&mut p_ff, &to_array(i_unit));
        push_f32s(&mut p_ff, &to_array(nref));
        assert_f32_slice_ulp(
            "faceforward",
            &to_array(faceforward(n_unit, i_unit, nref)),
            &read_f32s::<3>(oracle.call(55, &p_ff)?)?,
            1.0,
            0.0,
        );

        let mut p_refl = Vec::new();
        push_f32s(&mut p_refl, &to_array(i_unit));
        push_f32s(&mut p_refl, &to_array(n_unit));
        assert_f32_slice_ulp(
            "reflect",
            &to_array(reflect(i_unit, n_unit)),
            &read_f32s::<3>(oracle.call(56, &p_refl)?)?,
            2.0,
            1e-7,
        );

        let eta = rng.f32_range(0.5, 1.5);
        let mut p_refr = Vec::new();
        push_f32s(&mut p_refr, &to_array(i_unit));
        push_f32s(&mut p_refr, &to_array(n_unit));
        push_f32s(&mut p_refr, &[eta]);
        assert_f32_slice_ulp(
            "refract",
            &to_array(refract(i_unit, n_unit, eta)),
            &read_f32s::<3>(oracle.call(57, &p_refr)?)?,
            4.0,
            1e-6,
        );
    }

    oracle.finish()
}

#[test]
fn differential_packing_and_integer_bits() -> Result<(), String> {
    let Some(mut oracle) = OracleClient::try_spawn()? else {
        return Ok(());
    };

    let mut rng = Rng::new(0x9AC4_7777);
    for _ in 0..256 {
        let v2 = rng.vec2_range(-1.25, 1.25);
        let v4 = rng.vec4_range(-1.25, 1.25);
        let raw_u = rng.next_u32();

        let mut p_v2 = Vec::new();
        push_f32s(&mut p_v2, &to_array(v2));
        let exp_pu2 = read_u32s::<1>(oracle.call(60, &p_v2)?)?[0];
        let got_pu2 = pack_unorm_2x16(v2);
        assert!(
            ((got_pu2 & 0xffff) as i32 - (exp_pu2 & 0xffff) as i32).abs() <= 1
                && ((got_pu2 >> 16) as i32 - (exp_pu2 >> 16) as i32).abs() <= 1,
            "pack_unorm_2x16: got {got_pu2:#010x} vs {exp_pu2:#010x}"
        );

        let exp_ps2 = read_u32s::<1>(oracle.call(62, &p_v2)?)?[0];
        let got_ps2 = pack_snorm_2x16(v2);
        assert!(
            (((got_ps2 & 0xffff) as u16 as i16 as i32) - ((exp_ps2 & 0xffff) as u16 as i16 as i32))
                .abs()
                <= 1
                && (((got_ps2 >> 16) as u16 as i16 as i32)
                    - ((exp_ps2 >> 16) as u16 as i16 as i32))
                    .abs()
                    <= 1,
            "pack_snorm_2x16: got {got_ps2:#010x} vs {exp_ps2:#010x}"
        );

        let exp_ph2 = read_u32s::<1>(oracle.call(68, &p_v2)?)?[0];
        let got_ph2 = pack_half_2x16(v2);
        for shift in [0, 16] {
            let gh = (got_ph2 >> shift) & 0xffff;
            let eh = (exp_ph2 >> shift) & 0xffff;
            assert_eq!(gh & 0x8000, eh & 0x8000, "pack_half_2x16 sign mismatch");
            assert!(
                ((gh & 0x7fff) as i32 - (eh & 0x7fff) as i32).abs() <= 1,
                "pack_half_2x16: {got_ph2:#010x} vs {exp_ph2:#010x}"
            );
        }

        let mut p_v4 = Vec::new();
        push_f32s(&mut p_v4, &to_array(v4));
        let exp_pu4 = read_u32s::<1>(oracle.call(64, &p_v4)?)?[0];
        let got_pu4 = pack_unorm_4x8(v4);
        for shift in [0, 8, 16, 24] {
            let gb = ((got_pu4 >> shift) & 0xff) as i32;
            let eb = ((exp_pu4 >> shift) & 0xff) as i32;
            assert!(
                (gb - eb).abs() <= 1,
                "pack_unorm_4x8: {got_pu4:#010x} vs {exp_pu4:#010x}"
            );
        }

        let exp_ps4 = read_u32s::<1>(oracle.call(66, &p_v4)?)?[0];
        let got_ps4 = pack_snorm_4x8(v4);
        for shift in [0, 8, 16, 24] {
            let gb = ((got_ps4 >> shift) & 0xff) as u8 as i8 as i32;
            let eb = ((exp_ps4 >> shift) & 0xff) as u8 as i8 as i32;
            assert!(
                (gb - eb).abs() <= 1,
                "pack_snorm_4x8: {got_ps4:#010x} vs {exp_ps4:#010x}"
            );
        }

        let mut p_u = Vec::new();
        push_u32s(&mut p_u, &[raw_u]);
        assert_f32_slice_ulp(
            "unpack_unorm_2x16",
            &to_array(unpack_unorm_2x16(raw_u)),
            &read_f32s::<2>(oracle.call(61, &p_u)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "unpack_snorm_2x16",
            &to_array(unpack_snorm_2x16(raw_u)),
            &read_f32s::<2>(oracle.call(63, &p_u)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "unpack_unorm_4x8",
            &to_array(unpack_unorm_4x8(raw_u)),
            &read_f32s::<4>(oracle.call(65, &p_u)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "unpack_snorm_4x8",
            &to_array(unpack_snorm_4x8(raw_u)),
            &read_f32s::<4>(oracle.call(67, &p_u)?)?,
            1.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "unpack_half_2x16",
            &to_array(unpack_half_2x16(exp_ph2)),
            &read_f32s::<2>(oracle.call(69, &exp_ph2.to_le_bytes())?)?,
            0.0,
            0.0,
        );

        let uv = uvec4!(rng.next_u32(), rng.next_u32(), 0, u32::MAX);
        let iv = ivec4!(rng.next_u32() as i32, -(rng.next_u32() as i32), 0, -1);
        let offset = (rng.next_u32() % 25) as i32;
        let bits = (rng.next_u32() % (33 - offset as u32)) as i32;

        let mut p_ibfe = Vec::new();
        push_i32s(&mut p_ibfe, &to_array(iv));
        push_i32s(&mut p_ibfe, &[offset, bits]);
        let got_ibfe = to_array(bitfield_extract(iv, offset, bits));
        let glm_ibfe = read_i32s::<4>(oracle.call(70, &p_ibfe)?)?;
        let low_mask = if bits == 0 {
            0u32
        } else if bits == 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };
        for lane in 0..4 {
            let expected_sign_ext = if bits == 0 {
                0i32
            } else {
                let shl = 32 - offset - bits;
                let shr = 32 - bits;
                (to_array(iv)[lane] << shl) >> shr
            };
            assert_eq!(got_ibfe[lane], expected_sign_ext);
            assert_eq!(
                (got_ibfe[lane] as u32) & low_mask,
                (glm_ibfe[lane] as u32) & low_mask
            );
        }

        let mut p_ubfe = Vec::new();
        push_u32s(&mut p_ubfe, &to_array(uv));
        push_i32s(&mut p_ubfe, &[offset, bits]);
        assert_eq!(
            to_array(bitfield_extract(uv, offset, bits)),
            read_u32s::<4>(oracle.call(71, &p_ubfe)?)?
        );

        let ins = uvec4!(
            rng.next_u32(),
            rng.next_u32(),
            rng.next_u32(),
            rng.next_u32()
        );
        let mut p_bfi = Vec::new();
        push_u32s(&mut p_bfi, &to_array(uv));
        push_u32s(&mut p_bfi, &to_array(ins));
        push_i32s(&mut p_bfi, &[offset, bits]);
        assert_eq!(
            to_array(bitfield_insert(uv, ins, offset, bits)),
            read_u32s::<4>(oracle.call(72, &p_bfi)?)?
        );

        let mut p_uv = Vec::new();
        push_u32s(&mut p_uv, &to_array(uv));
        assert_eq!(
            to_array(bitfield_reverse(uv)),
            read_u32s::<4>(oracle.call(73, &p_uv)?)?
        );
        assert_eq!(
            to_array(bit_count(uv)),
            read_i32s::<4>(oracle.call(74, &p_uv)?)?
        );
        assert_eq!(
            to_array(find_lsb(uv)),
            read_i32s::<4>(oracle.call(75, &p_uv)?)?
        );
        assert_eq!(
            to_array(find_msb(uv)),
            read_i32s::<4>(oracle.call(77, &p_uv)?)?
        );

        let mut p_iv = Vec::new();
        push_i32s(&mut p_iv, &to_array(iv));
        let got_imsb = to_array(find_msb(iv));
        let glm_imsb = read_i32s::<4>(oracle.call(76, &p_iv)?)?;
        for lane in 0..4 {
            let val = to_array(iv)[lane];
            let expected_glsl = if val == 0 || val == -1 {
                -1
            } else if val < 0 {
                31 - ((!val) as u32).leading_zeros() as i32
            } else {
                31 - (val as u32).leading_zeros() as i32
            };
            assert_eq!(got_imsb[lane], expected_glsl);
            if val >= 0 {
                assert_eq!(got_imsb[lane], glm_imsb[lane]);
            }
        }
    }

    oracle.finish()
}

#[test]
fn differential_matrices_quaternions_and_gtc() -> Result<(), String> {
    let Some(mut oracle) = OracleClient::try_spawn()? else {
        return Ok(());
    };

    let mut rng = Rng::new(0x4A74_90A7);
    for _ in 0..128 {
        let m2a = mat2!(rng.vec2_range(-1.0, 1.0), rng.vec2_range(-1.0, 1.0)) + mat2(4.0);
        let m2b = mat2!(rng.vec2_range(-3.0, 3.0), rng.vec2_range(-3.0, 3.0));
        let mut p = Vec::new();
        push_f32s(&mut p, &to_array(m2a));
        push_f32s(&mut p, &to_array(m2b));
        assert_f32_slice_ulp(
            "mat2_mul",
            &to_array(m2a * m2b),
            &read_f32s::<4>(oracle.call(80, &p)?)?,
            2.0,
            2.0 * f32::EPSILON * 20.0,
        );

        let m3a = mat3!(
            rng.vec3_range(-1.0, 1.0),
            rng.vec3_range(-1.0, 1.0),
            rng.vec3_range(-1.0, 1.0)
        ) + mat3(5.0);
        let m3b = mat3!(
            rng.vec3_range(-2.0, 2.0),
            rng.vec3_range(-2.0, 2.0),
            rng.vec3_range(-2.0, 2.0)
        );
        p.clear();
        push_f32s(&mut p, &to_array(m3a));
        push_f32s(&mut p, &to_array(m3b));
        assert_f32_slice_ulp(
            "mat3_mul",
            &to_array(m3a * m3b),
            &read_f32s::<9>(oracle.call(81, &p)?)?,
            2.0,
            2.0 * f32::EPSILON * 20.0,
        );

        let m4a = mat4!(
            rng.vec4_range(-1.0, 1.0),
            rng.vec4_range(-1.0, 1.0),
            rng.vec4_range(-1.0, 1.0),
            rng.vec4_range(-1.0, 1.0)
        ) + mat4(6.0);
        let m4b = mat4!(
            rng.vec4_range(-2.0, 2.0),
            rng.vec4_range(-2.0, 2.0),
            rng.vec4_range(-2.0, 2.0),
            rng.vec4_range(-2.0, 2.0)
        );
        p.clear();
        push_f32s(&mut p, &to_array(m4a));
        push_f32s(&mut p, &to_array(m4b));
        assert_f32_slice_ulp(
            "mat4_mul",
            &to_array(m4a * m4b),
            &read_f32s::<16>(oracle.call(82, &p)?)?,
            4.0,
            1e-6,
        );

        let v4 = rng.vec4_range(-5.0, 5.0);
        p.clear();
        push_f32s(&mut p, &to_array(m4a));
        push_f32s(&mut p, &to_array(v4));
        assert_f32_slice_ulp(
            "mat4_mul_vec4",
            &to_array(m4a * v4),
            &read_f32s::<4>(oracle.call(83, &p)?)?,
            3.0,
            1e-6,
        );

        p.clear();
        push_f32s(&mut p, &to_array(v4));
        push_f32s(&mut p, &to_array(m4a));
        assert_f32_slice_ulp(
            "vec4_mul_mat4",
            &to_array(v4 * m4a),
            &read_f32s::<4>(oracle.call(84, &p)?)?,
            3.0,
            1e-6,
        );

        let m34a = mat3x4!(
            rng.vec4_range(-3.0, 3.0),
            rng.vec4_range(-3.0, 3.0),
            rng.vec4_range(-3.0, 3.0)
        );
        let m34b = mat3x4!(
            rng.vec4_range(-3.0, 3.0),
            rng.vec4_range(-3.0, 3.0),
            rng.vec4_range(-3.0, 3.0)
        );
        let v3 = rng.vec3_range(-4.0, 4.0);

        p.clear();
        push_f32s(&mut p, &to_array(m34a));
        push_f32s(&mut p, &to_array(v3));
        assert_f32_slice_ulp(
            "mat3x4_mul_vec3",
            &to_array(m34a * v3),
            &read_f32s::<4>(oracle.call(85, &p)?)?,
            3.0,
            1e-6,
        );

        p.clear();
        push_f32s(&mut p, &to_array(v4));
        push_f32s(&mut p, &to_array(m34a));
        assert_f32_slice_ulp(
            "vec4_mul_mat3x4",
            &to_array(v4 * m34a),
            &read_f32s::<3>(oracle.call(86, &p)?)?,
            3.0,
            1e-6,
        );

        let m23 = mat2x3!(rng.vec3_range(-3.0, 3.0), rng.vec3_range(-3.0, 3.0));
        let m42 = mat4x2!(
            rng.vec2_range(-3.0, 3.0),
            rng.vec2_range(-3.0, 3.0),
            rng.vec2_range(-3.0, 3.0),
            rng.vec2_range(-3.0, 3.0)
        );
        p.clear();
        push_f32s(&mut p, &to_array(m23));
        push_f32s(&mut p, &to_array(m42));
        assert_f32_slice_ulp(
            "mat2x3_mul_mat4x2",
            &to_array(m23 * m42),
            &read_f32s::<12>(oracle.call(87, &p)?)?,
            2.0,
            1e-6,
        );

        assert_f32_slice_ulp(
            "transpose_3x4",
            &to_array(transpose(m34a)),
            &read_f32s::<12>(oracle.call(88, &{
                let mut t = Vec::new();
                push_f32s(&mut t, &to_array(m34a));
                t
            })?)?,
            0.0,
            0.0,
        );

        p.clear();
        push_f32s(&mut p, &to_array(v4));
        push_f32s(&mut p, &to_array(v3));
        let op34: Mat3x4 = outer_product(v4, v3);
        assert_f32_slice_ulp(
            "outer_product_4x3",
            &to_array(op34),
            &read_f32s::<12>(oracle.call(89, &p)?)?,
            0.0,
            0.0,
        );

        p.clear();
        push_f32s(&mut p, &to_array(m34a));
        push_f32s(&mut p, &to_array(m34b));
        assert_f32_slice_ulp(
            "matrix_comp_mult",
            &to_array(matrix_comp_mult(m34a, m34b)),
            &read_f32s::<12>(oracle.call(90, &p)?)?,
            0.0,
            0.0,
        );

        p.clear();
        push_f32s(&mut p, &to_array(m2a));
        push_f32s(&mut p, &to_array(m3a));
        push_f32s(&mut p, &to_array(m4a));
        assert_f32_slice_ulp(
            "determinant",
            &[determinant(m2a), determinant(m3a), determinant(m4a)],
            &read_f32s::<3>(oracle.call(91, &p)?)?,
            8.0,
            1e-4,
        );

        assert_f32_slice_ulp(
            "inverse_mat2",
            &to_array(inverse(m2a)),
            &read_f32s::<4>(oracle.call(92, &{
                let mut t = Vec::new();
                push_f32s(&mut t, &to_array(m2a));
                t
            })?)?,
            4.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "inverse_mat3",
            &to_array(inverse(m3a)),
            &read_f32s::<9>(oracle.call(93, &{
                let mut t = Vec::new();
                push_f32s(&mut t, &to_array(m3a));
                t
            })?)?,
            8.0,
            1e-5,
        );
        assert_f32_slice_ulp(
            "inverse_mat4",
            &to_array(inverse(m4a)),
            &read_f32s::<16>(oracle.call(94, &{
                let mut t = Vec::new();
                push_f32s(&mut t, &to_array(m4a));
                t
            })?)?,
            16.0,
            1e-5,
        );

        let dm4a = dmat4(m4a);
        let dm4b = dmat4(m4b);
        p.clear();
        push_f64s(&mut p, &to_array(dm4a));
        push_f64s(&mut p, &to_array(dm4b));
        let exp_dmul = read_f64s::<16>(oracle.call(95, &p)?)?;
        for (g, e) in to_array(dm4a * dm4b).iter().zip(exp_dmul.iter()) {
            assert!((g - e).abs() <= 1e-12, "dmat4_mul: {g} vs {e}");
        }
        p.clear();
        push_f64s(&mut p, &to_array(dm4a));
        let exp_dinv = read_f64s::<16>(oracle.call(96, &p)?)?;
        for (g, e) in to_array(inverse(dm4a)).iter().zip(exp_dinv.iter()) {
            assert!((g - e).abs() <= 1e-12, "dmat4_inv: {g} vs {e}");
        }

        let ax1 = normalize(rng.vec3_range(-5.0, 5.0) + vec3!(0.2, -0.3, 0.4));
        let ax2 = normalize(rng.vec3_range(-5.0, 5.0) + vec3!(-0.4, 0.2, 0.3));
        let ang1 = rng.f32_range(-3.0, 3.0);
        let ang2 = rng.f32_range(-3.0, 3.0);
        let q1 = angle_axis(ang1, ax1);
        let q2 = angle_axis(ang2, ax2);

        p.clear();
        push_f32s(&mut p, &[ang1]);
        push_f32s(&mut p, &to_array(ax1));
        assert_f32_slice_ulp(
            "angle_axis",
            &to_array(q1),
            &read_f32s::<4>(oracle.call(102, &p)?)?,
            2.0,
            1e-6,
        );

        p.clear();
        push_f32s(&mut p, &to_array(q1));
        push_f32s(&mut p, &to_array(q2));
        assert_f32_slice_ulp(
            "quat_mul",
            &to_array(q1 * q2),
            &read_f32s::<4>(oracle.call(100, &p)?)?,
            3.0,
            1e-6,
        );

        p.clear();
        push_f32s(&mut p, &to_array(q1));
        push_f32s(&mut p, &to_array(v3));
        assert_f32_slice_ulp(
            "quat_mul_vec3",
            &to_array(q1 * v3),
            &read_f32s::<3>(oracle.call(101, &p)?)?,
            8.0,
            1e-5,
        );

        p.clear();
        push_f32s(&mut p, &to_array(q1));
        let rot3 = mat3_cast(q1);
        assert_f32_slice_ulp(
            "mat3_cast",
            &to_array(rot3),
            &read_f32s::<9>(oracle.call(103, &p)?)?,
            4.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "mat4_cast",
            &to_array(mat4_cast(q1)),
            &read_f32s::<16>(oracle.call(104, &p)?)?,
            4.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "conjugate",
            &to_array(conjugate(q1)),
            &read_f32s::<4>(oracle.call(107, &p)?)?,
            0.0,
            0.0,
        );
        assert_f32_slice_ulp(
            "quat_inverse",
            &to_array(inverse(q1)),
            &read_f32s::<4>(oracle.call(108, &p)?)?,
            2.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "euler_angles",
            &to_array(euler_angles(q1)),
            &read_f32s::<3>(oracle.call(109, &p)?)?,
            4.0,
            1e-5,
        );

        p.clear();
        push_f32s(&mut p, &to_array(rot3));
        let got_qc = quat_cast(rot3);
        let exp_qc = read_f32s::<4>(oracle.call(105, &p)?)?;
        let exp_qc_q = quat_xyzw(exp_qc[0], exp_qc[1], exp_qc[2], exp_qc[3]);
        assert!(
            dot(got_qc, exp_qc_q).abs() > 0.999_99,
            "quat_cast mismatch: {got_qc:?} vs {exp_qc_q:?}"
        );

        let slerp_t = rng.f32_range(0.0, 1.0);
        p.clear();
        push_f32s(&mut p, &to_array(q1));
        push_f32s(&mut p, &to_array(q2));
        push_f32s(&mut p, &[slerp_t]);
        let got_slerp = slerp(q1, q2, slerp_t);
        let exp_slerp_arr = read_f32s::<4>(oracle.call(106, &p)?)?;
        let exp_slerp = quat_xyzw(
            exp_slerp_arr[0],
            exp_slerp_arr[1],
            exp_slerp_arr[2],
            exp_slerp_arr[3],
        );
        assert!(
            dot(got_slerp, exp_slerp).abs() > 0.999_99,
            "slerp mismatch: {got_slerp:?} vs {exp_slerp:?}"
        );

        let look_dir = normalize(vec3!(0.3, -0.2, -1.0) + rng.vec3_range(-0.2, 0.2));
        let up = vec3!(0.0, 1.0, 0.0);
        p.clear();
        push_f32s(&mut p, &to_array(look_dir));
        push_f32s(&mut p, &to_array(up));
        assert_f32_slice_ulp(
            "quat_look_at_rh",
            &to_array(quat_look_at_rh(look_dir, up)),
            &read_f32s::<4>(oracle.call(110, &p)?)?,
            6.0,
            1e-5,
        );
        assert_f32_slice_ulp(
            "quat_look_at_lh",
            &to_array(quat_look_at_lh(look_dir, up)),
            &read_f32s::<4>(oracle.call(111, &p)?)?,
            6.0,
            1e-5,
        );

        p.clear();
        push_f32s(&mut p, &to_array(m4a));
        push_f32s(&mut p, &to_array(v3));
        assert_f32_slice_ulp(
            "translate",
            &to_array(translate(m4a, v3)),
            &read_f32s::<16>(oracle.call(120, &p)?)?,
            4.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "scale",
            &to_array(scale(m4a, v3)),
            &read_f32s::<16>(oracle.call(122, &p)?)?,
            1.0,
            0.0,
        );

        p.clear();
        push_f32s(&mut p, &to_array(m4a));
        push_f32s(&mut p, &[ang1]);
        push_f32s(&mut p, &to_array(ax1));
        assert_f32_slice_ulp(
            "rotate",
            &to_array(rotate(m4a, ang1, ax1)),
            &read_f32s::<16>(oracle.call(121, &p)?)?,
            8.0,
            1e-5,
        );

        let eye = vec3!(2.0, 3.0, 5.0) + rng.vec3_range(-0.5, 0.5);
        let center = rng.vec3_range(-0.5, 0.5);
        p.clear();
        push_f32s(&mut p, &to_array(eye));
        push_f32s(&mut p, &to_array(center));
        push_f32s(&mut p, &to_array(up));
        let view_rh = look_at_rh(eye, center, up);
        assert_f32_slice_ulp(
            "look_at_rh",
            &to_array(view_rh),
            &read_f32s::<16>(oracle.call(123, &p)?)?,
            6.0,
            1e-5,
        );
        assert_f32_slice_ulp(
            "look_at_lh",
            &to_array(look_at_lh(eye, center, up)),
            &read_f32s::<16>(oracle.call(124, &p)?)?,
            6.0,
            1e-5,
        );

        let fovy = rng.f32_range(0.5, 1.5);
        let aspect = rng.f32_range(0.75, 2.2);
        let zn = rng.f32_range(0.05, 1.0);
        let zf = rng.f32_range(20.0, 500.0);
        p.clear();
        push_f32s(&mut p, &[fovy, aspect, zn, zf]);
        let proj_rh = perspective_rh_zo(fovy, aspect, zn, zf);
        assert_f32_slice_ulp(
            "perspective_rh_zo",
            &to_array(proj_rh),
            &read_f32s::<16>(oracle.call(125, &p)?)?,
            3.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "perspective_lh_zo",
            &to_array(perspective_lh_zo(fovy, aspect, zn, zf)),
            &read_f32s::<16>(oracle.call(126, &p)?)?,
            3.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "perspective_rh_no",
            &to_array(perspective_rh_no(fovy, aspect, zn, zf)),
            &read_f32s::<16>(oracle.call(127, &p)?)?,
            3.0,
            1e-6,
        );

        p.clear();
        push_f32s(&mut p, &[-10.0, 12.0, -6.0, 8.0, zn, zf]);
        assert_f32_slice_ulp(
            "ortho_rh_zo",
            &to_array(ortho_rh_zo(-10.0, 12.0, -6.0, 8.0, zn, zf)),
            &read_f32s::<16>(oracle.call(128, &p)?)?,
            2.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "ortho_lh_zo",
            &to_array(ortho_lh_zo(-10.0, 12.0, -6.0, 8.0, zn, zf)),
            &read_f32s::<16>(oracle.call(129, &p)?)?,
            2.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "ortho_rh_no",
            &to_array(ortho_rh_no(-10.0, 12.0, -6.0, 8.0, zn, zf)),
            &read_f32s::<16>(oracle.call(130, &p)?)?,
            2.0,
            1e-6,
        );

        let vp = vec4!(0.0, 0.0, 1920.0, 1080.0);
        let obj_pt = vec3!(0.25, -0.5, 0.1);
        p.clear();
        push_f32s(&mut p, &to_array(obj_pt));
        push_f32s(&mut p, &to_array(view_rh));
        push_f32s(&mut p, &to_array(proj_rh));
        push_f32s(&mut p, &to_array(vp));
        let win_pt = project_zo(obj_pt, view_rh, proj_rh, vp);
        assert_f32_slice_ulp(
            "project_zo",
            &to_array(win_pt),
            &read_f32s::<3>(oracle.call(131, &p)?)?,
            16.0,
            1e-3,
        );

        p.clear();
        push_f32s(&mut p, &to_array(win_pt));
        push_f32s(&mut p, &to_array(view_rh));
        push_f32s(&mut p, &to_array(proj_rh));
        push_f32s(&mut p, &to_array(vp));
        assert_f32_slice_ulp(
            "unproject_zo",
            &to_array(unproject_zo(win_pt, view_rh, proj_rh, vp)),
            &read_f32s::<3>(oracle.call(132, &p)?)?,
            32.0,
            1e-4,
        );

        let rgba = rng.vec4_range(0.0, 1.0);
        p.clear();
        push_f32s(&mut p, &to_array(rgba));
        assert_f32_slice_ulp(
            "convert_linear_to_srgb",
            &to_array(convert_linear_to_srgb(rgba)),
            &read_f32s::<4>(oracle.call(133, &p)?)?,
            4.0,
            1e-6,
        );
        assert_f32_slice_ulp(
            "convert_srgb_to_linear",
            &to_array(convert_srgb_to_linear(rgba)),
            &read_f32s::<4>(oracle.call(134, &p)?)?,
            4.0,
            1e-6,
        );
    }

    oracle.finish()
}
