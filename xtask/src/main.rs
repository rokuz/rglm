mod package;

use anyhow::{Context, bail};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Deserialize)]
struct OraclesConfig {
    glm: GlmOracleConfig,
}

#[derive(Debug, Deserialize)]
struct GlmOracleConfig {
    repo: String,
    commit: String,
    #[serde(rename = "version")]
    _version: String,
}

fn workspace_root() -> anyhow::Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .context("failed to locate workspace root")
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        entries.push(entry.path());
    }
    entries.sort();
    for path in entries {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name == "target" || name == ".git" {
            continue;
        }
        if path.is_dir() {
            collect_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

fn strip_strings_on_line(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0;
    let mut in_str = false;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if b == b'\\' {
                out.push(' ');
                out.push(' ');
                i += 2;
                continue;
            }
            if b == b'"' {
                in_str = false;
            }
            out.push(' ');
            i += 1;
        } else if b == b'"' {
            in_str = true;
            out.push('"');
            i += 1;
        } else {
            out.push(b as char);
            i += 1;
        }
    }
    out
}

fn verify_rs_file(path: &Path, content: &str) -> anyhow::Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0;
    while idx < lines.len() {
        let raw = lines[idx];
        let trimmed = raw.trim();
        let stripped = strip_strings_on_line(trimmed);

        if stripped.contains("/*") || stripped.contains("*/") {
            bail!(
                "{}:{}: block comments are forbidden",
                path.display(),
                idx + 1
            );
        }
        if trimmed.starts_with("//!") {
            bail!(
                "{}:{}: inner doc comments are forbidden",
                path.display(),
                idx + 1
            );
        }

        if trimmed.starts_with("///") && !trimmed.starts_with("////") {
            let doc_start = idx + 1;
            let mut has_macro_export = false;
            while idx < lines.len() {
                let cur = lines[idx].trim();
                if cur.starts_with("///") && !cur.starts_with("////") {
                    if !cur.is_ascii() {
                        bail!(
                            "{}:{}: non-ASCII character in doc comment",
                            path.display(),
                            idx + 1
                        );
                    }
                    idx += 1;
                    continue;
                }
                if cur.starts_with("#[") {
                    if cur.contains("macro_export") {
                        has_macro_export = true;
                    }
                    let mut depth: i32 = 0;
                    while idx < lines.len() {
                        let attr_line = strip_strings_on_line(lines[idx]);
                        for ch in attr_line.chars() {
                            if ch == '[' {
                                depth += 1;
                            } else if ch == ']' {
                                depth -= 1;
                            }
                        }
                        idx += 1;
                        if depth <= 0 {
                            break;
                        }
                    }
                    continue;
                }
                break;
            }

            let target = lines.get(idx).map(|s| s.trim()).unwrap_or("");
            let valid_fn = target.starts_with("pub fn ")
                || target.starts_with("pub const fn ")
                || target.starts_with("pub unsafe fn ");
            let valid_macro = has_macro_export && target.starts_with("macro_rules! ");
            if !(valid_fn || valid_macro) {
                bail!(
                    "{}:{}: doc comments are only allowed on public functions and exported macros (found `{}`)",
                    path.display(),
                    doc_start,
                    target
                );
            }
            continue;
        }

        if stripped.contains("//") {
            bail!(
                "{}:{}: line comments are forbidden",
                path.display(),
                idx + 1
            );
        }

        idx += 1;
    }
    Ok(())
}

fn verify_cpp_file(path: &Path, content: &str) -> anyhow::Result<()> {
    for (i, line) in content.lines().enumerate() {
        let stripped = strip_strings_on_line(line.trim());
        if stripped.contains("//") || stripped.contains("/*") || stripped.contains("*/") {
            bail!(
                "{}:{}: comments are forbidden in C/C++ files",
                path.display(),
                i + 1
            );
        }
    }
    Ok(())
}

fn verify_tera_file(path: &Path, content: &str) -> anyhow::Result<()> {
    for (i, line) in content.lines().enumerate() {
        let stripped = strip_strings_on_line(line.trim());
        if stripped.contains("{#") || stripped.contains("/*") {
            bail!(
                "{}:{}: template comments are forbidden",
                path.display(),
                i + 1
            );
        }
        let trimmed = line.trim();
        if trimmed.starts_with("///") && !trimmed.starts_with("////") {
            if !trimmed.is_ascii() {
                bail!(
                    "{}:{}: non-ASCII doc comment in template",
                    path.display(),
                    i + 1
                );
            }
            continue;
        }
        if stripped.contains("//") {
            bail!(
                "{}:{}: line comments are forbidden in template",
                path.display(),
                i + 1
            );
        }
    }
    Ok(())
}

fn verify_toml_file(path: &Path, content: &str) -> anyhow::Result<()> {
    for (i, line) in content.lines().enumerate() {
        let stripped = strip_strings_on_line(line.trim());
        if stripped.contains('#') {
            bail!(
                "{}:{}: comments are forbidden in TOML files",
                path.display(),
                i + 1
            );
        }
    }
    Ok(())
}

fn verify_no_inherent_pub_methods(root: &Path) -> anyhow::Result<()> {
    let mut files = Vec::new();
    for sub in ["vec", "mat", "quat"] {
        let dir = root.join("rglm/src").join(sub);
        if dir.exists() {
            collect_files(&dir, &mut files)?;
        }
    }
    for path in files {
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let content = fs::read_to_string(&path)?;
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("pub fn ") || t.starts_with("pub const fn ") {
                bail!(
                    "{}:{}: public functions or inherent methods are forbidden inside type modules (`{}`)",
                    path.display(),
                    i + 1,
                    t
                );
            }
        }
    }
    Ok(())
}

fn check_style_rules(root: &Path) -> anyhow::Result<()> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    for path in &files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "rs" => {
                let content = fs::read_to_string(path)?;
                verify_rs_file(path, &content)?;
            }
            "cpp" | "cc" | "c" | "h" | "hpp" => {
                let content = fs::read_to_string(path)?;
                verify_cpp_file(path, &content)?;
            }
            "tera" => {
                let content = fs::read_to_string(path)?;
                verify_tera_file(path, &content)?;
            }
            "toml" => {
                let content = fs::read_to_string(path)?;
                verify_toml_file(path, &content)?;
            }
            _ => {}
        }
    }
    verify_no_inherent_pub_methods(root)?;
    Ok(())
}

fn run_cmd(root: &Path, program: &str, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new(program)
        .current_dir(root)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn {program}"))?;
    if !status.success() {
        bail!(
            "command `{program} {}` failed with {status}",
            args.join(" ")
        );
    }
    Ok(())
}

fn run_cmd_envs(
    root: &Path,
    program: &str,
    args: &[&str],
    envs: &[(&str, &str)],
) -> anyhow::Result<()> {
    let mut cmd = Command::new(program);
    cmd.current_dir(root).args(args);
    for &(k, v) in envs {
        cmd.env(k, v);
    }
    let status = cmd
        .status()
        .with_context(|| format!("failed to spawn {program}"))?;
    if !status.success() {
        bail!(
            "command `{program} {}` failed with {status}",
            args.join(" ")
        );
    }
    Ok(())
}

fn run_cmd_stdout(root: &Path, program: &str, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new(program)
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn {program}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "command `{program} {}` failed with {}: {stderr}",
            args.join(" "),
            output.status
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn glm_oracle_bin_path(root: &Path) -> PathBuf {
    let name = if cfg!(windows) {
        "glm_oracle.exe"
    } else {
        "glm_oracle"
    };
    root.join("target/oracles/bin").join(name)
}

fn build_oracles(root: &Path) -> anyhow::Result<PathBuf> {
    let cfg_str =
        fs::read_to_string(root.join("oracles.toml")).context("failed to read oracles.toml")?;
    let cfg: OraclesConfig = toml::from_str(&cfg_str).context("failed to parse oracles.toml")?;

    let glm_dir = root.join("target/oracles/glm");
    let head_ok = if glm_dir.join(".git").exists() {
        let out = Command::new("git")
            .current_dir(&glm_dir)
            .args(["rev-parse", "HEAD"])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                String::from_utf8_lossy(&o.stdout).trim() == cfg.glm.commit
            }
            _ => false,
        }
    } else {
        false
    };

    if !head_ok {
        fs::create_dir_all(&glm_dir)
            .with_context(|| format!("failed to create {}", glm_dir.display()))?;
        if !glm_dir.join(".git").exists() {
            run_cmd(&glm_dir, "git", &["init"])?;
        }
        let has_origin = Command::new("git")
            .current_dir(&glm_dir)
            .args(["remote", "get-url", "origin"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if has_origin {
            run_cmd(
                &glm_dir,
                "git",
                &["remote", "set-url", "origin", &cfg.glm.repo],
            )?;
        } else {
            run_cmd(&glm_dir, "git", &["remote", "add", "origin", &cfg.glm.repo])?;
        }
        run_cmd(
            &glm_dir,
            "git",
            &["fetch", "--depth", "1", "origin", &cfg.glm.commit],
        )?;
        run_cmd(&glm_dir, "git", &["checkout", "--detach", "FETCH_HEAD"])?;
    }

    let bin_path = glm_oracle_bin_path(root);
    if let Some(parent) = bin_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let cxx = std::env::var("CXX").unwrap_or_else(|_| "c++".to_string());
    let cpp_src = root.join("xtask/oracle/glm_oracle.cpp");
    let glm_inc = glm_dir
        .to_str()
        .context("non-UTF-8 glm oracle include path")?;
    let cpp_src_str = cpp_src.to_str().context("non-UTF-8 oracle source path")?;
    let bin_str = bin_path.to_str().context("non-UTF-8 oracle binary path")?;

    run_cmd(
        root,
        &cxx,
        &[
            "-std=c++17",
            "-O2",
            "-fno-fast-math",
            "-I",
            glm_inc,
            cpp_src_str,
            "-o",
            bin_str,
        ],
    )?;

    Ok(bin_path)
}

fn run_oracle_tests(root: &Path) -> anyhow::Result<()> {
    let bin_path = build_oracles(root)?;
    let bin_str = bin_path.to_str().context("non-UTF-8 oracle binary path")?;
    let envs = [
        ("RGLM_GLM_ORACLE_BIN", bin_str),
        ("RGLM_REQUIRE_ORACLE", "1"),
    ];

    run_cmd_envs(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "rglm",
            "--test",
            "glm_oracle",
            "--features",
            "bytemuck,serde,mint,arbitrary,derive",
            "--",
            "--nocapture",
        ],
        &envs,
    )?;

    run_cmd_envs(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "rglm",
            "--test",
            "glm_oracle",
            "--features",
            "scalar-math,bytemuck,serde,mint,arbitrary,derive",
            "--",
            "--nocapture",
        ],
        &envs,
    )?;

    println!("xtask oracle-test passed");
    Ok(())
}

fn extract_asm_fn<'a>(asm: &'a str, symbol: &str) -> Option<Vec<&'a str>> {
    let label_macho = format!("_{symbol}:");
    let label_elf = format!("{symbol}:");
    let mut lines = asm.lines();
    let mut body = Vec::new();
    let mut inside = false;
    for line in &mut lines {
        let trimmed = line.trim();
        if !inside {
            if trimmed == label_macho || trimmed == label_elf {
                inside = true;
            }
            continue;
        }
        if trimmed == ".cfi_endproc"
            || (trimmed.ends_with(':') && !trimmed.starts_with('L') && !trimmed.starts_with(".L"))
        {
            break;
        }
        if !trimmed.starts_with('.') && !trimmed.ends_with(':') && !trimmed.is_empty() {
            body.push(trimmed);
        }
    }
    if inside { Some(body) } else { None }
}

fn find_emitted_asm(examples_dir: &Path) -> anyhow::Result<PathBuf> {
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in fs::read_dir(examples_dir)
        .with_context(|| format!("failed to read {}", examples_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let matches_probe = (name == "asm_probes.s"
            || (name.starts_with("asm_probes-") && name.ends_with(".s")))
            && !name.contains(".rcgu.");
        if !matches_probe {
            continue;
        }
        let mtime = entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        if best.as_ref().is_none_or(|(prev, _)| mtime >= *prev) {
            best = Some((mtime, path));
        }
    }
    best.map(|(_, p)| p).with_context(|| {
        format!(
            "failed to locate emitted asm_probes*.s in {}",
            examples_dir.display()
        )
    })
}

fn run_asm_check(root: &Path) -> anyhow::Result<()> {
    run_cmd(
        root,
        "cargo",
        &[
            "rustc",
            "-p",
            "rglm",
            "--example",
            "asm_probes",
            "--release",
            "--",
            "-C",
            "codegen-units=1",
            "-C",
            "lto=no",
            "-C",
            "debuginfo=0",
            "--emit=asm",
        ],
    )?;

    let asm_path = find_emitted_asm(&root.join("target/release/examples"))?;
    let asm = fs::read_to_string(&asm_path)
        .with_context(|| format!("failed to read {}", asm_path.display()))?;

    let probes = [
        "rglm_probe_mat4_mul",
        "rglm_probe_mat4_mul_vec4",
        "rglm_probe_mat3x4_mul_vec3",
        "rglm_probe_vec4_mix",
        "rglm_probe_vec4_swizzle_blend",
        "rglm_probe_quat_mul_vec3",
    ];

    for probe in probes {
        let body = extract_asm_fn(&asm, probe)
            .with_context(|| format!("missing probe symbol `{probe}` in emitted assembly"))?;
        if body.is_empty() {
            bail!("probe `{probe}` has an empty instruction body");
        }
        for insn in &body {
            if insn.contains("[sp")
                || insn.starts_with("sub\tsp,")
                || insn.starts_with("sub sp,")
                || insn.contains("(%rsp)")
                || (insn.starts_with("sub") && insn.contains("%rsp"))
            {
                bail!(
                    "register spill or stack frame detected in `{probe}`: `{insn}`\nfull body:\n{}",
                    body.join("\n")
                );
            }
        }
        println!("{probe}: {} instructions, 0 stack spills", body.len());
    }

    println!("xtask asm passed");
    Ok(())
}

fn parse_bench_tsv(out: &str) -> Vec<(String, f64)> {
    let mut rows = Vec::new();
    for line in out.lines() {
        let Some((name, val)) = line.split_once('\t') else {
            continue;
        };
        if let Ok(ns) = val.trim().parse::<f64>() {
            rows.push((name.trim().to_string(), ns));
        }
    }
    rows
}

fn lookup_ns(rows: &[(String, f64)], name: &str) -> f64 {
    rows.iter()
        .find(|(n, _)| n == name)
        .map(|(_, v)| *v)
        .unwrap_or(0.0)
}

fn build_glm_bench_bins(root: &Path) -> anyhow::Result<(PathBuf, PathBuf)> {
    let _ = build_oracles(root)?;
    let glm_dir = root.join("target/oracles/glm");
    let bin_dir = root.join("target/oracles/bin");
    fs::create_dir_all(&bin_dir)?;

    let ext = if cfg!(windows) { ".exe" } else { "" };
    let default_bin = bin_dir.join(format!("glm_bench_default{ext}"));
    let simd_bin = bin_dir.join(format!("glm_bench_simd{ext}"));

    let cxx = std::env::var("CXX").unwrap_or_else(|_| "c++".to_string());
    let cpp_src = root.join("xtask/bench/glm_bench.cpp");
    let glm_inc = glm_dir
        .to_str()
        .context("non-UTF-8 glm oracle include path")?;
    let cpp_src_str = cpp_src.to_str().context("non-UTF-8 glm bench path")?;
    let default_bin_str = default_bin
        .to_str()
        .context("non-UTF-8 glm default bench binary path")?;
    let simd_bin_str = simd_bin
        .to_str()
        .context("non-UTF-8 glm simd bench binary path")?;

    run_cmd(
        root,
        &cxx,
        &[
            "-std=c++17",
            "-O3",
            "-fno-fast-math",
            "-I",
            glm_inc,
            cpp_src_str,
            "-o",
            default_bin_str,
        ],
    )?;

    run_cmd(
        root,
        &cxx,
        &[
            "-std=c++17",
            "-O3",
            "-fno-fast-math",
            "-DGLM_FORCE_INTRINSICS",
            "-DGLM_FORCE_DEFAULT_ALIGNED_GENTYPES",
            "-I",
            glm_inc,
            cpp_src_str,
            "-o",
            simd_bin_str,
        ],
    )?;

    Ok((default_bin, simd_bin))
}

fn run_bench_suite(root: &Path) -> anyhow::Result<()> {
    let (glm_default_bin, glm_simd_bin) = build_glm_bench_bins(root)?;
    let glm_default_bin_str = glm_default_bin
        .to_str()
        .context("non-UTF-8 glm default bench path")?;
    let glm_simd_bin_str = glm_simd_bin
        .to_str()
        .context("non-UTF-8 glm simd bench path")?;

    run_cmd(
        root,
        "cargo",
        &["bench", "-p", "rglm", "--bench", "linalg", "--no-run"],
    )?;
    run_cmd(
        root,
        "cargo",
        &[
            "bench",
            "-p",
            "rglm",
            "--bench",
            "linalg",
            "--features",
            "scalar-math",
            "--no-run",
        ],
    )?;

    let rglm_simd_out = run_cmd_stdout(
        root,
        "cargo",
        &[
            "bench",
            "-p",
            "rglm",
            "--bench",
            "linalg",
            "--",
            "--run-bench",
            "--engine",
            "rglm",
        ],
    )?;
    let glam_out = run_cmd_stdout(
        root,
        "cargo",
        &[
            "bench",
            "-p",
            "rglm",
            "--bench",
            "linalg",
            "--",
            "--run-bench",
            "--engine",
            "glam",
        ],
    )?;
    let glm_simd_out = run_cmd_stdout(root, glm_simd_bin_str, &[])?;
    let glm_default_out = run_cmd_stdout(root, glm_default_bin_str, &[])?;
    let rglm_scalar_out = run_cmd_stdout(
        root,
        "cargo",
        &[
            "bench",
            "-p",
            "rglm",
            "--bench",
            "linalg",
            "--features",
            "scalar-math",
            "--",
            "--run-bench",
            "--engine",
            "rglm",
        ],
    )?;

    let rglm_simd = parse_bench_tsv(&rglm_simd_out);
    let glam_rows = parse_bench_tsv(&glam_out);
    let glm_simd = parse_bench_tsv(&glm_simd_out);
    let glm_default = parse_bench_tsv(&glm_default_out);
    let rglm_scalar = parse_bench_tsv(&rglm_scalar_out);

    let mut md = String::from(
        "| Kernel | rglm (ns) | glam (ns) | C++ GLM SIMD (ns) | C++ GLM def (ns) | rglm scalar (ns) | vs glam | vs GLM SIMD | vs GLM def |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    let mut tsv = String::from(
        "kernel\trglm_simd_ns\tglam_ns\tcpp_glm_simd_ns\tcpp_glm_default_ns\trglm_scalar_ns\n",
    );

    for (name, r_ns) in &rglm_simd {
        let g_ns = lookup_ns(&glam_rows, name);
        let cs_ns = lookup_ns(&glm_simd, name);
        let cd_ns = lookup_ns(&glm_default, name);
        let rs_ns = lookup_ns(&rglm_scalar, name);
        let ratio = |other: f64| if *r_ns > 0.0 { other / *r_ns } else { 0.0 };
        md.push_str(&format!(
            "| `{name}` | {r_ns:.3} | {g_ns:.3} | {cs_ns:.3} | {cd_ns:.3} | {rs_ns:.3} | {:.2}x | {:.2}x | {:.2}x |\n",
            ratio(g_ns),
            ratio(cs_ns),
            ratio(cd_ns),
        ));
        tsv.push_str(&format!(
            "{name}\t{r_ns:.3}\t{g_ns:.3}\t{cs_ns:.3}\t{cd_ns:.3}\t{rs_ns:.3}\n"
        ));
    }

    let out_dir = root.join("target/xtask");
    fs::create_dir_all(&out_dir)?;
    let md_path = out_dir.join("bench.md");
    let tsv_path = out_dir.join("bench.tsv");
    fs::write(&md_path, &md)?;
    fs::write(&tsv_path, &tsv)?;
    print!("{md}");
    println!("wrote {}", md_path.display());
    Ok(())
}

fn run_fuzz_smoke(root: &Path, args: &[String]) -> anyhow::Result<()> {
    if args.first().map(String::as_str) == Some("--libfuzzer") {
        let target = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("builtins_no_panic");
        let max_time = args.get(2).map(String::as_str).unwrap_or("10");
        let time_arg = format!("-max_total_time={max_time}");
        run_cmd(
            root,
            "cargo",
            &["+nightly", "fuzz", "run", target, "--", &time_arg],
        )?;
        return Ok(());
    }

    run_cmd(
        root,
        "cargo",
        &[
            "clippy",
            "--manifest-path",
            "fuzz/Cargo.toml",
            "--no-default-features",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;

    for bin in ["builtins_no_panic", "mat_quat_algebra", "layout_roundtrip"] {
        run_cmd(
            root,
            "cargo",
            &[
                "run",
                "--manifest-path",
                "fuzz/Cargo.toml",
                "--no-default-features",
                "--release",
                "--bin",
                bin,
            ],
        )?;
    }

    println!("xtask fuzz smoke passed");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let root = workspace_root()?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let subcmd = args.first().map(String::as_str).unwrap_or("check");

    match subcmd {
        "check" => {
            check_style_rules(&root)?;
            run_cmd(&root, "cargo", &["run", "-p", "codegen", "--", "--check"])?;
            run_cmd(&root, "cargo", &["fmt", "--all", "--", "--check"])?;
            run_cmd(
                &root,
                "cargo",
                &[
                    "clippy",
                    "-p",
                    "rglm",
                    "--all-targets",
                    "--features",
                    "bytemuck,serde,mint,arbitrary,derive",
                    "--",
                    "-D",
                    "warnings",
                ],
            )?;
            run_cmd(
                &root,
                "cargo",
                &[
                    "clippy",
                    "-p",
                    "rglm",
                    "--no-default-features",
                    "--features",
                    "libm,all-types,bytemuck,serde,mint,arbitrary,derive",
                    "--",
                    "-D",
                    "warnings",
                ],
            )?;
            run_cmd(
                &root,
                "cargo",
                &[
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ],
            )?;
            run_cmd(
                &root,
                "cargo",
                &[
                    "clippy",
                    "--manifest-path",
                    "fuzz/Cargo.toml",
                    "--no-default-features",
                    "--all-targets",
                    "--",
                    "-D",
                    "warnings",
                ],
            )?;
            run_cmd(
                &root,
                "cargo",
                &[
                    "test",
                    "-p",
                    "rglm",
                    "--features",
                    "bytemuck,serde,mint,arbitrary,derive",
                ],
            )?;
            run_cmd(&root, "cargo", &["test", "--workspace", "--all-features"])?;
            println!("xtask check passed");
        }
        "style" => {
            check_style_rules(&root)?;
            println!("xtask style passed");
        }
        "oracles" => {
            let bin = build_oracles(&root)?;
            println!("built C++ GLM oracle at {}", bin.display());
        }
        "oracle-test" | "test" => {
            run_oracle_tests(&root)?;
        }
        "asm" => {
            run_asm_check(&root)?;
        }
        "bench" => {
            run_bench_suite(&root)?;
        }
        "fuzz" => {
            run_fuzz_smoke(&root, &args[1..])?;
        }
        "package-crates" => {
            package::package_crates(&root, &args[1..])?;
        }
        other => {
            bail!(
                "unknown xtask command `{other}` (expected `check`, `style`, `oracles`, `oracle-test`, `asm`, `bench`, `fuzz`, or `package-crates`)"
            );
        }
    }
    Ok(())
}
