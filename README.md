# rglm

SIMD linear algebra for Rust — a fork of [glam](https://github.com/bitshifter/glam-rs) redesigned around Vulkan GLSL 450/460 semantics.

## Install

```toml
[dependencies]
rglm = { version = "0.1", features = ["derive"] }
```

For `#![no_std]`:

```toml
[dependencies]
rglm = { version = "0.1", default-features = false, features = ["libm", "all-types"] }
```

## Examples

### Vectors, Built-ins & Swizzling

```rust
use rglm::*;

let n = normalize(vec3!(1.0, 2.0, -2.0));
let wrapped = modulo(vec2!(-1.25, 3.75), 1.0);
let blended = mix(vec3(0.0), n, 0.5);

let mut color = vec4!(blended, 1.0);
swz!(color.rgb) *= 1.25;
let bgra = rswz!(color.bgra);
```

### Quaternions, Matrices & Vulkan Cameras

```rust
use rglm::*;

let rot = angle_axis(radians(45.0_f32), vec3!(0.0, 1.0, 0.0));
let model = mat4_cast(rot);
let view = look_at_rh(vec3!(0.0, 2.0, 5.0), vec3(0.0), vec3!(0.0, 1.0, 0.0));
let proj = perspective_vk(radians(60.0_f32), 16.0 / 9.0, 0.1, 1000.0);

let clip = proj * view * model * vec4!(1.0, 0.5, -2.0, 1.0);
let skin: mat3x4 = mat3x4(1.0);
let world_pos = skin * vec3!(1.0, 2.0, 3.0);
```

### Vulkan Buffer Layouts (`feature = "derive"`)

```rust
use rglm::*;

#[derive(Clone, Copy, Std140, Std430, Scalar)]
struct CameraBlock {
    view_proj: Mat4,
    eye: Vec3,
    exposure: f32,
}

let block = CameraBlock {
    view_proj: mat4(1.0),
    eye: vec3!(0.0, 2.0, 5.0),
    exposure: 1.2,
};

let repr = to_std140(&block);
let mut ubo = [0u8; <CameraBlock as Std140>::SIZE];
write_std140(&block, &mut ubo);
```

## Features

| Feature | Description |
|---|---|
| `std` *(default)* | Standard library scalar float intrinsics |
| `libm` | Pure-Rust `#![no_std]` scalar math backend |
| `all-types` *(default)* | `f16`, `f64`, and `i8`/`u8`..`i64`/`u64` vectors and matrices |
| `derive` | `#[derive(Std140, Std430, Scalar)]` for Vulkan UBO/SSBO/push-constant layouts |
| `bytemuck`, `serde`, `mint`, `arbitrary` | Ecosystem trait implementations |
| `scalar-math`, `core-simd` | Force scalar fallback or nightly `core::simd` |
| `debug-assert` | Runtime domain assertions (`normalize` of zero, singular `inverse`, invalid `clamp`) |

## Differences from `glam`

| Area | `glam` | `rglm` |
|---|---|---|
| API style | Inherent methods (`v.normalize()`, `a.lerp(b, t)`) | GLSL free functions only (`normalize(v)`, `mix(a, b, t)`) |
| Matrices & scalars | Square `2x2..4x4`, custom `Affine*`, `*SizeVec*` | All 9 GLSL `CxR` matrices (`2x2..4x4`) + full `f16` arithmetic; no `Affine*` / `*SizeVec*` |
| Swizzles | Read-only `Vec*Swizzles` traits (`xyzw`) | Read (`rswz!`) and write/compound-assign (`swz!`) across `xyzw`, `rgba`, `stpq` |
| Math semantics | Rust float defaults (`fract(-0.25) == -0.25`) | Strict GLSL 460 (`fract`, `modulo`, `sign(-0.0) == 0.0`, bit & pack/unpack built-ins), zero-panic `NaN` defaults |
| Elementary functions | Per-lane `sin`, `cos`, `exp`, `ln`, `powf` only | Full GLSL §8.1–8.2 set (`tan`, `asin`/`acos`/`atan`, `sinh`..`atanh`, `exp2`/`log2`), `<= 1.5 ULP` `Vec3A`/`Vec4` trig/exp/log SIMD polynomials + `gtx::fast_math` |
| Naming & constructors | `UpperCamelCase` only | Dual `Vec3` / `vec3` + `rglm::glsl_names`, variadic `vec*!` / `mat*!`, explicit `quat_wxyz` / `quat_xyzw` |
| Vulkan integration | External layout crates | Explicit `*_vk` / `*_rh_zo` projections + built-in zero-`syn` `#[derive(Std140, Std430, Scalar)]` |

## Origins & Attribution

`rglm` stands on the shoulders of established graphics math libraries:

- **[glam](https://github.com/bitshifter/glam-rs)** *(Cameron Hart & contributors)* — `rglm` originated as a hard fork of `glam` `0.34.3`, building on its SIMD backend design and Tera code generation architecture while redesigning the API around pure GLSL free functions, rectangular matrices, and strict shader semantics.
- **[OpenGL Mathematics (GLM)](https://github.com/g-truc/glm)** *(G-Truc Creation)* — primary design reference for free-function overloading, packed `vec3` layout, `gtc`/`gtx` extensions, and coordinate-system conventions; verified continuously against pinned C++ GLM `1.0.3` via differential ULP testing.
- **Vulkan GLSL 450/460 & Vulkan 1.3** *(Khronos Group)* — authoritative specification for built-in edge cases (`fract`, `modulo`, `sign`, bit/pack ops, `f16` arithmetic) and `std140`, `std430`, and `VK_EXT_scalar_block_layout` memory rules.
- **[DirectXMath](https://github.com/microsoft/DirectXMath)** *(Microsoft)* & **[Realtime Math](https://github.com/nfrechette/rtm)** *(Nicholas Frechette)* — foundational SIMD polynomial approximations and matrix/quaternion kernel techniques adapted via `glam`.

Full copyright notices are listed in [ATTRIBUTION.md](ATTRIBUTION.md).

## License

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
