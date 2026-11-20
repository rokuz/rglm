# AGENTS.md — rglm

> A SIMD-accelerated Rust math library faithful to C++ GLM and Vulkan GLSL 450/460.
> Hard fork of `glam` 0.34.3 (`edition = "2024"`, MSRV `1.93`, `#![no_std]`).

If `AGENTS.local.md` exists in the workspace root, read and follow its rules in addition to this document.

## Layout

| Path | What |
|---|---|
| `rglm/` | core `#![no_std]` math library: vectors, matrices (`2x2..4x4`), quaternions, swizzles (`rswz!`/`swz!`), GLSL 460 built-ins, `gtc`/`gtx`, Vulkan buffer layouts |
| `rglm-derive/` | zero-`syn` procedural macro crate providing `#[derive(Std140, Std430, Scalar)]` |
| `codegen/` | Tera template generator producing vector, matrix, quaternion, mask, and built-in trait implementations |
| `fuzz/` | `cargo-fuzz` targets (`builtins_no_panic`, `mat_quat_algebra`, `layout_roundtrip`) |
| `xtask/` | workspace automation: style/lint verification, C++ GLM differential ULP oracle, assembly spill checks, fuzzing, benchmarks |
| `oracles.toml` | pinned C++ GLM commit (`1.0.3`) for differential ULP testing |

## Architecture & Lint Conventions

- **Free functions only.** Never add public inherent methods (`pub fn`) to `Vec*`, `Mat*`, `Quat*`, or `BVec*` structs (`cargo xtask style`). Structs expose only fields/deref, associated constants, indexing, standard operators, and conversion traits; all math and conversions live in free functions.
- **Doc-comments only (`cargo xtask style`).** Do not add `//`, `/* */`, `{# #}`, `#` (in `.toml`), or internal `///` comments in code, templates, tests, or configs. Only concise ASCII `///` doc-comments on public free functions (`pub fn`, `pub const fn`) and exported public macros (`#[macro_export] macro_rules!`) are permitted, documenting mathematical behavior, coordinate conventions, and edge cases.
- **Idiomatic, high-assurance Rust.**
  - `unwrap`, `expect`, `panic!`, `unreachable!`, `unimplemented!`, and `todo!` are denied across the entire workspace, including tests and tools (`clippy.toml`). Tests return `Result` or use `assert!`/`assert_eq!`.
  - Use `#[expect(..., reason = "...")]` instead of `#[allow(...)]` (`allow_attributes = "deny"`).
  - `unsafe_code` is denied by default and permitted only with an explicit `#[expect(unsafe_code, reason = "...")]` on isolated SIMD intrinsics and verified `#[repr(C)]` / `#[repr(transparent)]` layout casts.
  - Pure math functions and constructors must carry `#[must_use]`.
  - Default builds never panic on mathematical domain inputs (`normalize(vec3(0.0))` yields `NaN`); optional domain assertions are gated behind `feature = "debug-assert"`.

## Commands

```sh
cargo codegen
cargo codegen --check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p rglm --no-default-features --features libm,all-types
cargo test -p rglm --features scalar-math,all-types
cargo deny check
cargo xtask check
cargo xtask oracles
cargo xtask oracle-test
cargo xtask asm
cargo xtask fuzz
cargo xtask bench
```
