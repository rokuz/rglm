use anyhow::{Context, bail};
use rustfmt_wrapper::config::{Config as RustfmtConfig, Edition};
use rustfmt_wrapper::rustfmt_config;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct MulTarget {
    rhs_mat_t: String,
    rhs_cols: usize,
    out_mat_t: String,
}

struct GeneratedFile {
    rel_path: String,
    template: &'static str,
    context: tera::Context,
}

fn workspace_root() -> anyhow::Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .context("failed to locate workspace root")
}

fn build_mask_files(out: &mut Vec<GeneratedFile>) {
    let specs = [
        ("BVec2", 2usize, false, "rglm/src/vec/bool/bvec2.rs"),
        ("BVec3", 3usize, false, "rglm/src/vec/bool/bvec3.rs"),
        ("BVec3A", 3usize, true, "rglm/src/vec/bool/bvec3a.rs"),
        ("BVec4", 4usize, false, "rglm/src/vec/bool/bvec4.rs"),
        ("BVec4A", 4usize, true, "rglm/src/vec/bool/bvec4a.rs"),
    ];
    for (self_t, dim, is_simd, rel_path) in specs {
        let mut ctx = tera::Context::new();
        ctx.insert("self_t", self_t);
        ctx.insert("dim", &dim);
        ctx.insert("is_simd", &is_simd);
        out.push(GeneratedFile {
            rel_path: rel_path.to_string(),
            template: "vec_mask.rs.tera",
            context: ctx,
        });
    }
}

struct VecFamily {
    prefix: &'static str,
    scalar_t: &'static str,
    subdir: &'static str,
    is_float: bool,
    is_signed: bool,
    is_int: bool,
    is_f16: bool,
    math_mod: &'static str,
    zero_lit: &'static str,
    one_lit: &'static str,
    neg_one_lit: &'static str,
}

fn build_vec_files(out: &mut Vec<GeneratedFile>) {
    let families = [
        VecFamily {
            prefix: "Vec",
            scalar_t: "f32",
            subdir: "f32",
            is_float: true,
            is_signed: true,
            is_int: false,
            is_f16: false,
            math_mod: "f32_math",
            zero_lit: "0.0",
            one_lit: "1.0",
            neg_one_lit: "-1.0",
        },
        VecFamily {
            prefix: "DVec",
            scalar_t: "f64",
            subdir: "f64",
            is_float: true,
            is_signed: true,
            is_int: false,
            is_f16: false,
            math_mod: "f64_math",
            zero_lit: "0.0",
            one_lit: "1.0",
            neg_one_lit: "-1.0",
        },
        VecFamily {
            prefix: "F16Vec",
            scalar_t: "f16",
            subdir: "f16",
            is_float: true,
            is_signed: true,
            is_int: false,
            is_f16: true,
            math_mod: "f16_math",
            zero_lit: "f16::ZERO",
            one_lit: "f16::ONE",
            neg_one_lit: "f16::NEG_ONE",
        },
        VecFamily {
            prefix: "I8Vec",
            scalar_t: "i8",
            subdir: "int",
            is_float: false,
            is_signed: true,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "-1",
        },
        VecFamily {
            prefix: "U8Vec",
            scalar_t: "u8",
            subdir: "int",
            is_float: false,
            is_signed: false,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "0",
        },
        VecFamily {
            prefix: "I16Vec",
            scalar_t: "i16",
            subdir: "int",
            is_float: false,
            is_signed: true,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "-1",
        },
        VecFamily {
            prefix: "U16Vec",
            scalar_t: "u16",
            subdir: "int",
            is_float: false,
            is_signed: false,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "0",
        },
        VecFamily {
            prefix: "IVec",
            scalar_t: "i32",
            subdir: "int",
            is_float: false,
            is_signed: true,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "-1",
        },
        VecFamily {
            prefix: "UVec",
            scalar_t: "u32",
            subdir: "int",
            is_float: false,
            is_signed: false,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "0",
        },
        VecFamily {
            prefix: "I64Vec",
            scalar_t: "i64",
            subdir: "int",
            is_float: false,
            is_signed: true,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "-1",
        },
        VecFamily {
            prefix: "U64Vec",
            scalar_t: "u64",
            subdir: "int",
            is_float: false,
            is_signed: false,
            is_int: true,
            is_f16: false,
            math_mod: "",
            zero_lit: "0",
            one_lit: "1",
            neg_one_lit: "0",
        },
    ];

    for fam in &families {
        for dim in [2usize, 3, 4] {
            let self_t = format!("{}{dim}", fam.prefix);
            let storage = match (fam.scalar_t, dim) {
                ("f32", 4) => "simd_f32x4",
                ("f64", 2) => "simd_f64x2",
                ("f64", 4) => "simd_f64x2_pair",
                _ => "fields",
            };
            push_vec_spec(out, fam, &self_t, dim, storage);

            if fam.scalar_t == "f32" && dim == 3 {
                push_vec_spec(out, fam, "Vec3A", 3, "simd_f32x4");
            }
        }
    }
}

fn push_vec_spec(
    out: &mut Vec<GeneratedFile>,
    fam: &VecFamily,
    self_t: &str,
    dim: usize,
    storage: &str,
) {
    let mut ctx = tera::Context::new();
    ctx.insert("self_t", self_t);
    ctx.insert("scalar_t", fam.scalar_t);
    ctx.insert("dim", &dim);
    ctx.insert("storage", storage);
    ctx.insert("is_float", &fam.is_float);
    ctx.insert("is_signed", &fam.is_signed);
    ctx.insert("is_int", &fam.is_int);
    ctx.insert("is_f16", &fam.is_f16);
    ctx.insert("math_mod", fam.math_mod);
    ctx.insert("mask_t", &format!("BVec{dim}"));
    ctx.insert("zero_lit", fam.zero_lit);
    ctx.insert("one_lit", fam.one_lit);
    ctx.insert("neg_one_lit", fam.neg_one_lit);

    let file_stem = self_t.to_ascii_lowercase();
    out.push(GeneratedFile {
        rel_path: format!("rglm/src/vec/{}/{file_stem}.rs", fam.subdir),
        template: "vec.rs.tera",
        context: ctx,
    });
}

fn mat_type_name(mat_prefix: &str, cols: usize, rows: usize, aligned: bool) -> String {
    let base = if cols == rows {
        format!("{mat_prefix}{cols}")
    } else {
        format!("{mat_prefix}{cols}x{rows}")
    };
    if aligned { format!("{base}A") } else { base }
}

fn vec_type_name(vec_prefix: &str, dim: usize, aligned: bool) -> String {
    if aligned && dim == 3 && vec_prefix == "Vec" {
        "Vec3A".to_string()
    } else {
        format!("{vec_prefix}{dim}")
    }
}

fn build_mat_files(out: &mut Vec<GeneratedFile>) {
    let families = [
        ("Mat", "Vec", "f32", "f32", false, "0.0", "1.0", "-1.0"),
        ("DMat", "DVec", "f64", "f64", false, "0.0", "1.0", "-1.0"),
        (
            "F16Mat",
            "F16Vec",
            "f16",
            "f16",
            true,
            "f16::ZERO",
            "f16::ONE",
            "f16::NEG_ONE",
        ),
    ];

    for (mat_prefix, vec_prefix, scalar_t, subdir, is_f16, zero_lit, one_lit, neg_one_lit) in
        families
    {
        for cols in [2usize, 3, 4] {
            for rows in [2usize, 3, 4] {
                push_mat_spec(
                    out,
                    mat_prefix,
                    vec_prefix,
                    scalar_t,
                    subdir,
                    is_f16,
                    zero_lit,
                    one_lit,
                    neg_one_lit,
                    cols,
                    rows,
                    false,
                );
                if scalar_t == "f32" && rows == 3 {
                    push_mat_spec(
                        out,
                        mat_prefix,
                        vec_prefix,
                        scalar_t,
                        subdir,
                        is_f16,
                        zero_lit,
                        one_lit,
                        neg_one_lit,
                        cols,
                        rows,
                        true,
                    );
                }
            }
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "matrix family descriptor carries scalar literals"
)]
fn push_mat_spec(
    out: &mut Vec<GeneratedFile>,
    mat_prefix: &str,
    vec_prefix: &str,
    scalar_t: &str,
    subdir: &str,
    is_f16: bool,
    zero_lit: &str,
    one_lit: &str,
    neg_one_lit: &str,
    cols: usize,
    rows: usize,
    aligned: bool,
) {
    let self_t = mat_type_name(mat_prefix, cols, rows, aligned);
    let col_t = vec_type_name(vec_prefix, rows, aligned);
    let rhs_vec_t = vec_type_name(vec_prefix, cols, aligned && cols == 3);
    let transpose_aligned = aligned && cols == 3 && rows == 3;
    let transpose_t = mat_type_name(mat_prefix, rows, cols, transpose_aligned);
    let transpose_col_t = vec_type_name(vec_prefix, cols, transpose_aligned);

    let storage = if scalar_t == "f32" && !aligned && cols == 2 && rows == 2 {
        "simd_f32x4"
    } else if scalar_t == "f32" && !aligned && cols == 4 && rows == 2 {
        "simd_f32x4_pair"
    } else {
        "cols"
    };

    let mut mul_targets = Vec::new();
    let mut extra_imports = BTreeSet::new();
    extra_imports.insert(transpose_col_t.clone());

    for rhs_cols in [2usize, 3, 4] {
        let rhs_aligned = aligned && cols == 3;
        let rhs_mat_t = mat_type_name(mat_prefix, rhs_cols, cols, rhs_aligned);
        let out_mat_t = mat_type_name(mat_prefix, rhs_cols, rows, aligned);
        extra_imports.insert(rhs_mat_t.clone());
        extra_imports.insert(out_mat_t.clone());
        mul_targets.push(MulTarget {
            rhs_mat_t,
            rhs_cols,
            out_mat_t,
        });
    }

    extra_imports.remove(&self_t);
    extra_imports.remove(&col_t);
    extra_imports.remove(&rhs_vec_t);
    extra_imports.remove(&transpose_t);

    let extra_vec: Vec<String> = extra_imports.into_iter().collect();

    let mut ctx = tera::Context::new();
    ctx.insert("self_t", &self_t);
    ctx.insert("scalar_t", scalar_t);
    ctx.insert("is_f16", &is_f16);
    ctx.insert("cols", &cols);
    ctx.insert("rows", &rows);
    ctx.insert("col_t", &col_t);
    ctx.insert("rhs_vec_t", &rhs_vec_t);
    ctx.insert("transpose_t", &transpose_t);
    ctx.insert("transpose_col_t", &transpose_col_t);
    ctx.insert("storage", storage);
    ctx.insert("zero_lit", zero_lit);
    ctx.insert("one_lit", one_lit);
    ctx.insert("neg_one_lit", neg_one_lit);
    ctx.insert("mul_targets", &mul_targets);
    ctx.insert("extra_imports", &extra_vec);

    let file_stem = self_t.to_ascii_lowercase();
    out.push(GeneratedFile {
        rel_path: format!("rglm/src/mat/{subdir}/{file_stem}.rs"),
        template: "mat.rs.tera",
        context: ctx,
    });
}

fn build_quat_files(out: &mut Vec<GeneratedFile>) {
    let specs = [
        (
            "Quat",
            "f32",
            false,
            "f32_math",
            "simd_f32x4",
            "Vec3",
            "Vec4",
            "Mat3",
            "Mat4",
            "0.0",
            "1.0",
            "2.0",
            "0.5",
            "0.25",
            "0.9995",
        ),
        (
            "DQuat",
            "f64",
            false,
            "f64_math",
            "simd_f64x2_pair",
            "DVec3",
            "DVec4",
            "DMat3",
            "DMat4",
            "0.0",
            "1.0",
            "2.0",
            "0.5",
            "0.25",
            "0.9999995",
        ),
        (
            "F16Quat",
            "f16",
            true,
            "f16_math",
            "fields",
            "F16Vec3",
            "F16Vec4",
            "F16Mat3",
            "F16Mat4",
            "f16::ZERO",
            "f16::ONE",
            "f16::from_f32_const(2.0)",
            "f16::from_f32_const(0.5)",
            "f16::from_f32_const(0.25)",
            "f16::from_f32_const(0.995)",
        ),
    ];

    for (
        self_t,
        scalar_t,
        is_f16,
        math_mod,
        storage,
        vec3_t,
        vec4_t,
        mat3_t,
        mat4_t,
        zero_lit,
        one_lit,
        two_lit,
        half_lit,
        quarter_lit,
        slerp_threshold_lit,
    ) in specs
    {
        let mut ctx = tera::Context::new();
        ctx.insert("self_t", self_t);
        ctx.insert("scalar_t", scalar_t);
        ctx.insert("is_f16", &is_f16);
        ctx.insert("math_mod", math_mod);
        ctx.insert("storage", storage);
        ctx.insert("vec3_t", vec3_t);
        ctx.insert("vec4_t", vec4_t);
        ctx.insert("mat3_t", mat3_t);
        ctx.insert("mat4_t", mat4_t);
        ctx.insert("zero_lit", zero_lit);
        ctx.insert("one_lit", one_lit);
        ctx.insert("two_lit", two_lit);
        ctx.insert("half_lit", half_lit);
        ctx.insert("quarter_lit", quarter_lit);
        ctx.insert("slerp_threshold_lit", slerp_threshold_lit);

        let file_stem = if self_t == "Quat" {
            "f32quat".to_string()
        } else {
            self_t.to_ascii_lowercase()
        };
        out.push(GeneratedFile {
            rel_path: format!("rglm/src/quat/{file_stem}.rs"),
            template: "quat.rs.tera",
            context: ctx,
        });
    }
}

#[derive(Serialize)]
struct CtorSpec {
    name: &'static str,
    out_t: &'static str,
    cfg: &'static str,
}

fn build_construct_macros_file(out: &mut Vec<GeneratedFile>) {
    let items = [
        ("vec2", "Vec2", ""),
        ("vec3", "Vec3", ""),
        ("vec3a", "Vec3A", ""),
        ("vec4", "Vec4", ""),
        ("bvec2", "BVec2", ""),
        ("bvec3", "BVec3", ""),
        ("bvec3a", "BVec3A", ""),
        ("bvec4", "BVec4", ""),
        ("bvec4a", "BVec4A", ""),
        ("mat2", "Mat2", ""),
        ("mat2x2", "Mat2", ""),
        ("mat2x3", "Mat2x3", ""),
        ("mat2x3a", "Mat2x3A", ""),
        ("mat2x4", "Mat2x4", ""),
        ("mat3x2", "Mat3x2", ""),
        ("mat3", "Mat3", ""),
        ("mat3x3", "Mat3", ""),
        ("mat3a", "Mat3A", ""),
        ("mat3x3a", "Mat3A", ""),
        ("mat3x4", "Mat3x4", ""),
        ("mat4x2", "Mat4x2", ""),
        ("mat4x3", "Mat4x3", ""),
        ("mat4x3a", "Mat4x3A", ""),
        ("mat4", "Mat4", ""),
        ("mat4x4", "Mat4", ""),
        ("dvec2", "DVec2", "f64"),
        ("dvec3", "DVec3", "f64"),
        ("dvec4", "DVec4", "f64"),
        ("dmat2", "DMat2", "f64"),
        ("dmat2x2", "DMat2", "f64"),
        ("dmat2x3", "DMat2x3", "f64"),
        ("dmat2x4", "DMat2x4", "f64"),
        ("dmat3x2", "DMat3x2", "f64"),
        ("dmat3", "DMat3", "f64"),
        ("dmat3x3", "DMat3", "f64"),
        ("dmat3x4", "DMat3x4", "f64"),
        ("dmat4x2", "DMat4x2", "f64"),
        ("dmat4x3", "DMat4x3", "f64"),
        ("dmat4", "DMat4", "f64"),
        ("dmat4x4", "DMat4", "f64"),
        ("f16vec2", "F16Vec2", "f16"),
        ("f16vec3", "F16Vec3", "f16"),
        ("f16vec4", "F16Vec4", "f16"),
        ("f16mat2", "F16Mat2", "f16"),
        ("f16mat2x2", "F16Mat2", "f16"),
        ("f16mat2x3", "F16Mat2x3", "f16"),
        ("f16mat2x4", "F16Mat2x4", "f16"),
        ("f16mat3x2", "F16Mat3x2", "f16"),
        ("f16mat3", "F16Mat3", "f16"),
        ("f16mat3x3", "F16Mat3", "f16"),
        ("f16mat3x4", "F16Mat3x4", "f16"),
        ("f16mat4x2", "F16Mat4x2", "f16"),
        ("f16mat4x3", "F16Mat4x3", "f16"),
        ("f16mat4", "F16Mat4", "f16"),
        ("f16mat4x4", "F16Mat4", "f16"),
        ("i8vec2", "I8Vec2", "i8"),
        ("i8vec3", "I8Vec3", "i8"),
        ("i8vec4", "I8Vec4", "i8"),
        ("u8vec2", "U8Vec2", "u8"),
        ("u8vec3", "U8Vec3", "u8"),
        ("u8vec4", "U8Vec4", "u8"),
        ("i16vec2", "I16Vec2", "i16"),
        ("i16vec3", "I16Vec3", "i16"),
        ("i16vec4", "I16Vec4", "i16"),
        ("u16vec2", "U16Vec2", "u16"),
        ("u16vec3", "U16Vec3", "u16"),
        ("u16vec4", "U16Vec4", "u16"),
        ("ivec2", "IVec2", "i32"),
        ("ivec3", "IVec3", "i32"),
        ("ivec4", "IVec4", "i32"),
        ("uvec2", "UVec2", "u32"),
        ("uvec3", "UVec3", "u32"),
        ("uvec4", "UVec4", "u32"),
        ("i64vec2", "I64Vec2", "i64"),
        ("i64vec3", "I64Vec3", "i64"),
        ("i64vec4", "I64Vec4", "i64"),
        ("u64vec2", "U64Vec2", "u64"),
        ("u64vec3", "U64Vec3", "u64"),
        ("u64vec4", "U64Vec4", "u64"),
    ];
    let ctors: Vec<CtorSpec> = items
        .into_iter()
        .map(|(name, out_t, cfg)| CtorSpec { name, out_t, cfg })
        .collect();
    let mut ctx = tera::Context::new();
    ctx.insert("ctors", &ctors);
    out.push(GeneratedFile {
        rel_path: "rglm/src/construct_macros.rs".to_string(),
        template: "construct_macros.rs.tera",
        context: ctx,
    });
}

fn build_swizzle_mask_file(out: &mut Vec<GeneratedFile>) {
    out.push(GeneratedFile {
        rel_path: "rglm/src/swizzle/mask.rs".to_string(),
        template: "swizzle_mask.rs.tera",
        context: tera::Context::new(),
    });
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut check_mode = false;
    let mut filter: Option<String> = None;

    for arg in args {
        if arg == "--check" {
            check_mode = true;
        } else if !arg.starts_with('-') {
            filter = Some(arg);
        } else {
            bail!("unknown option `{arg}` (supported: `--check`, `[filter]`)");
        }
    }

    let root = workspace_root()?;
    let template_glob = root.join("codegen/templates/*.rs.tera");
    let glob_str = template_glob
        .to_str()
        .context("non-UTF8 template directory path")?;

    let mut tera = tera::Tera::default();
    tera.load_from_glob(glob_str)
        .context("failed to parse Tera templates")?;

    let mut files = Vec::new();
    build_mask_files(&mut files);
    build_vec_files(&mut files);
    build_mat_files(&mut files);
    build_quat_files(&mut files);
    build_swizzle_mask_file(&mut files);
    build_construct_macros_file(&mut files);

    let mut diffs = 0usize;
    for item in &files {
        if let Some(ref pat) = filter
            && !item.rel_path.contains(pat.as_str())
        {
            continue;
        }

        let rendered = tera
            .render(item.template, &item.context)
            .with_context(|| format!("failed to render {} for {}", item.template, item.rel_path))?;

        let fmt_cfg = RustfmtConfig {
            edition: Some(Edition::Edition2024),
            ..Default::default()
        };
        let formatted = rustfmt_config(fmt_cfg, &rendered)
            .with_context(|| format!("rustfmt failed for {}", item.rel_path))?;

        let full_path = root.join(&item.rel_path);
        if check_mode {
            match fs::read_to_string(&full_path) {
                Ok(existing) if existing == formatted => {}
                Ok(_) => {
                    println!("out of date: {}", item.rel_path);
                    diffs += 1;
                }
                Err(_) => {
                    println!("missing: {}", item.rel_path);
                    diffs += 1;
                }
            }
        } else {
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&full_path, formatted)
                .with_context(|| format!("failed to write {}", full_path.display()))?;
        }
    }

    if check_mode && diffs > 0 {
        bail!("{diffs} generated file(s) differ from templates; run `cargo codegen`");
    }

    Ok(())
}
