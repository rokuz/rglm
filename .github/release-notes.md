rglm @VERSION@: a SIMD linear algebra library for Rust faithful to C++ GLM and Vulkan GLSL 450/460,
`#![no_std]`, with GLSL free functions, all nine `CxR` matrices, writable swizzles and
`#[derive(Std140, Std430, Scalar)]` for Vulkan buffer layouts.

## Files

| File | What |
|---|---|
| `rglm-@VERSION@.crate` | `cargo package` archive of `rglm`, the library |
| `rglm-derive-@VERSION@.crate` | `cargo package` archive of `rglm-derive`, the procedural macros behind the `derive` feature |
| `SHA256SUMS` | SHA-256 of every file: `sha256sum -c SHA256SUMS` |

## Rust

The crates are on [crates.io](https://crates.io/crates/rglm):

```toml
[dependencies]
rglm = { version = "@VERSION@", features = ["derive"] }
```

For `#![no_std]`:

```toml
[dependencies]
rglm = { version = "@VERSION@", default-features = false, features = ["libm", "all-types"] }
```

```rust
use rglm::*;

let rot = angle_axis(radians(45.0_f32), vec3!(0.0, 1.0, 0.0));
let view = look_at_rh(vec3!(0.0, 2.0, 5.0), vec3(0.0), vec3!(0.0, 1.0, 0.0));
let proj = perspective_vk(radians(60.0_f32), 16.0 / 9.0, 0.1, 1000.0);
let clip = proj * view * mat4_cast(rot) * vec4!(1.0, 0.5, -2.0, 1.0);
```

The features and the differences from `glam` are listed in the
[README](https://github.com/rokuz/rglm/blob/v@VERSION@/README.md).
