# rglm

`#![no_std]` SIMD linear algebra for Rust with strict **C++ GLM 1.0.3** and **Vulkan GLSL 450/460** semantics.

## Features

- **GLSL free-function API** — polymorphic free functions (`dot`, `cross`, `normalize`, `mix`, `fract`, `modulo`, `inverse`, ...) with zero inherent math methods on types.
- **Full GLSL type set** — `f32`, `f64`, `f16` (`half`), `i8..i64`, `u8..u64`, `bool` vectors (`2..4`), all 9 matrix shapes (`2x2..4x4`), quaternions, and 16-byte aligned `Vec3A` / `Mat3A` (`rglm::aligned`).
- **Dual naming** — Rust `Vec3` / `Mat3x4` / `Quat` and GLSL `vec3` / `mat3x4` / `quat`, plus `camelCase` built-ins in `rglm::glsl_names`.
- **Swizzling** — read (`rswz!(v.bgr)`) and write (`swz!(v.xz) = uv`) across `xyzw`, `rgba`, and `stpq` with compile-time lane checks.
- **Vulkan buffer layouts** — `#[derive(Std140, Std430, Scalar)]` matching Vulkan 1.3 §15.6.4 (`feature = "derive"`).
- **SIMD backends** — `aarch64` NEON, `x86`/`x86_64` SSE2/SSE4.1/FMA, `wasm32`/`wasm64` `simd128`, `core-simd`, and `scalar-math`, with `<= 1.5 ULP` 4-wide transcendentals.

## Usage

```toml
[dependencies]
rglm = { version = "0.1", features = ["derive"] }
```

For `#![no_std]`:

```toml
[dependencies]
rglm = { version = "0.1", default-features = false, features = ["libm", "all-types"] }
```

```rust
use rglm::*;

let mut color = vec4!(0.2, 0.4, 0.8, 1.0);
swz!(color.rgb) *= 1.25;
let bgra = rswz!(color.bgra);

let view = look_at_rh(vec3!(0.0, 2.0, 5.0), vec3(0.0), vec3!(0.0, 1.0, 0.0));
let proj = perspective_vk(radians(60.0_f32), 16.0 / 9.0, 0.1, 1000.0);
let clip = proj * view * vec4!(1.0, 0.5, -2.0, 1.0);
```

## Cargo Features

| Feature | Description |
|---|---|
| `std` *(default)* | Standard library float intrinsics |
| `libm` | Pure-Rust `#![no_std]` math backend |
| `all-types` *(default)* | Enables `f16`, `f64`, and `i8`/`u8`..`i64`/`u64` vectors and matrices |
| `derive` | `#[derive(Std140, Std430, Scalar)]` and `bytemuck` integration |
| `bytemuck`, `serde`, `mint`, `arbitrary` | Ecosystem trait implementations |
| `scalar-math`, `core-simd` | Force scalar fallback or nightly `core::simd` |

## Development

```sh
cargo xtask check
cargo xtask oracle-test
cargo xtask asm
cargo xtask fuzz
cargo xtask bench
```

## License

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
