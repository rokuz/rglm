/// Constructs `Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn vec2<A: crate::Construct<crate::Vec2>>(args: A) -> crate::Vec2 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`vec2`].
#[macro_export]
macro_rules! vec2 {
    ($a:expr $(,)?) => {
        $crate::vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::vec2(($a, $($rest),+))
    };
}

/// Constructs `Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn vec3<A: crate::Construct<crate::Vec3>>(args: A) -> crate::Vec3 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`vec3`].
#[macro_export]
macro_rules! vec3 {
    ($a:expr $(,)?) => {
        $crate::vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::vec3(($a, $($rest),+))
    };
}

/// Constructs `Vec3A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn vec3a<A: crate::Construct<crate::Vec3A>>(args: A) -> crate::Vec3A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`vec3a`].
#[macro_export]
macro_rules! vec3a {
    ($a:expr $(,)?) => {
        $crate::vec3a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::vec3a(($a, $($rest),+))
    };
}

/// Constructs `Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn vec4<A: crate::Construct<crate::Vec4>>(args: A) -> crate::Vec4 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`vec4`].
#[macro_export]
macro_rules! vec4 {
    ($a:expr $(,)?) => {
        $crate::vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::vec4(($a, $($rest),+))
    };
}

/// Constructs `BVec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn bvec2<A: crate::Construct<crate::BVec2>>(args: A) -> crate::BVec2 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`bvec2`].
#[macro_export]
macro_rules! bvec2 {
    ($a:expr $(,)?) => {
        $crate::bvec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::bvec2(($a, $($rest),+))
    };
}

/// Constructs `BVec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn bvec3<A: crate::Construct<crate::BVec3>>(args: A) -> crate::BVec3 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`bvec3`].
#[macro_export]
macro_rules! bvec3 {
    ($a:expr $(,)?) => {
        $crate::bvec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::bvec3(($a, $($rest),+))
    };
}

/// Constructs `BVec3A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn bvec3a<A: crate::Construct<crate::BVec3A>>(args: A) -> crate::BVec3A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`bvec3a`].
#[macro_export]
macro_rules! bvec3a {
    ($a:expr $(,)?) => {
        $crate::bvec3a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::bvec3a(($a, $($rest),+))
    };
}

/// Constructs `BVec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn bvec4<A: crate::Construct<crate::BVec4>>(args: A) -> crate::BVec4 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`bvec4`].
#[macro_export]
macro_rules! bvec4 {
    ($a:expr $(,)?) => {
        $crate::bvec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::bvec4(($a, $($rest),+))
    };
}

/// Constructs `BVec4A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn bvec4a<A: crate::Construct<crate::BVec4A>>(args: A) -> crate::BVec4A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`bvec4a`].
#[macro_export]
macro_rules! bvec4a {
    ($a:expr $(,)?) => {
        $crate::bvec4a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::bvec4a(($a, $($rest),+))
    };
}

/// Constructs `Mat2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat2<A: crate::Construct<crate::Mat2>>(args: A) -> crate::Mat2 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat2`].
#[macro_export]
macro_rules! mat2 {
    ($a:expr $(,)?) => {
        $crate::mat2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat2(($a, $($rest),+))
    };
}

/// Constructs `Mat2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat2x2<A: crate::Construct<crate::Mat2>>(args: A) -> crate::Mat2 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat2x2`].
#[macro_export]
macro_rules! mat2x2 {
    ($a:expr $(,)?) => {
        $crate::mat2x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat2x2(($a, $($rest),+))
    };
}

/// Constructs `Mat2x3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat2x3<A: crate::Construct<crate::Mat2x3>>(args: A) -> crate::Mat2x3 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat2x3`].
#[macro_export]
macro_rules! mat2x3 {
    ($a:expr $(,)?) => {
        $crate::mat2x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat2x3(($a, $($rest),+))
    };
}

/// Constructs `Mat2x3A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat2x3a<A: crate::Construct<crate::Mat2x3A>>(args: A) -> crate::Mat2x3A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat2x3a`].
#[macro_export]
macro_rules! mat2x3a {
    ($a:expr $(,)?) => {
        $crate::mat2x3a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat2x3a(($a, $($rest),+))
    };
}

/// Constructs `Mat2x4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat2x4<A: crate::Construct<crate::Mat2x4>>(args: A) -> crate::Mat2x4 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat2x4`].
#[macro_export]
macro_rules! mat2x4 {
    ($a:expr $(,)?) => {
        $crate::mat2x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat2x4(($a, $($rest),+))
    };
}

/// Constructs `Mat3x2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat3x2<A: crate::Construct<crate::Mat3x2>>(args: A) -> crate::Mat3x2 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat3x2`].
#[macro_export]
macro_rules! mat3x2 {
    ($a:expr $(,)?) => {
        $crate::mat3x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat3x2(($a, $($rest),+))
    };
}

/// Constructs `Mat3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat3<A: crate::Construct<crate::Mat3>>(args: A) -> crate::Mat3 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat3`].
#[macro_export]
macro_rules! mat3 {
    ($a:expr $(,)?) => {
        $crate::mat3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat3(($a, $($rest),+))
    };
}

/// Constructs `Mat3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat3x3<A: crate::Construct<crate::Mat3>>(args: A) -> crate::Mat3 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat3x3`].
#[macro_export]
macro_rules! mat3x3 {
    ($a:expr $(,)?) => {
        $crate::mat3x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat3x3(($a, $($rest),+))
    };
}

/// Constructs `Mat3A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat3a<A: crate::Construct<crate::Mat3A>>(args: A) -> crate::Mat3A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat3a`].
#[macro_export]
macro_rules! mat3a {
    ($a:expr $(,)?) => {
        $crate::mat3a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat3a(($a, $($rest),+))
    };
}

/// Constructs `Mat3A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat3x3a<A: crate::Construct<crate::Mat3A>>(args: A) -> crate::Mat3A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat3x3a`].
#[macro_export]
macro_rules! mat3x3a {
    ($a:expr $(,)?) => {
        $crate::mat3x3a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat3x3a(($a, $($rest),+))
    };
}

/// Constructs `Mat3x4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat3x4<A: crate::Construct<crate::Mat3x4>>(args: A) -> crate::Mat3x4 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat3x4`].
#[macro_export]
macro_rules! mat3x4 {
    ($a:expr $(,)?) => {
        $crate::mat3x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat3x4(($a, $($rest),+))
    };
}

/// Constructs `Mat4x2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat4x2<A: crate::Construct<crate::Mat4x2>>(args: A) -> crate::Mat4x2 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat4x2`].
#[macro_export]
macro_rules! mat4x2 {
    ($a:expr $(,)?) => {
        $crate::mat4x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat4x2(($a, $($rest),+))
    };
}

/// Constructs `Mat4x3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat4x3<A: crate::Construct<crate::Mat4x3>>(args: A) -> crate::Mat4x3 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat4x3`].
#[macro_export]
macro_rules! mat4x3 {
    ($a:expr $(,)?) => {
        $crate::mat4x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat4x3(($a, $($rest),+))
    };
}

/// Constructs `Mat4x3A` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat4x3a<A: crate::Construct<crate::Mat4x3A>>(args: A) -> crate::Mat4x3A {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat4x3a`].
#[macro_export]
macro_rules! mat4x3a {
    ($a:expr $(,)?) => {
        $crate::mat4x3a($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat4x3a(($a, $($rest),+))
    };
}

/// Constructs `Mat4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat4<A: crate::Construct<crate::Mat4>>(args: A) -> crate::Mat4 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat4`].
#[macro_export]
macro_rules! mat4 {
    ($a:expr $(,)?) => {
        $crate::mat4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat4(($a, $($rest),+))
    };
}

/// Constructs `Mat4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn mat4x4<A: crate::Construct<crate::Mat4>>(args: A) -> crate::Mat4 {
    args.construct()
}

/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`mat4x4`].
#[macro_export]
macro_rules! mat4x4 {
    ($a:expr $(,)?) => {
        $crate::mat4x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::mat4x4(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DVec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dvec2<A: crate::Construct<crate::DVec2>>(args: A) -> crate::DVec2 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dvec2`].
#[macro_export]
macro_rules! dvec2 {
    ($a:expr $(,)?) => {
        $crate::dvec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dvec2(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DVec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dvec3<A: crate::Construct<crate::DVec3>>(args: A) -> crate::DVec3 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dvec3`].
#[macro_export]
macro_rules! dvec3 {
    ($a:expr $(,)?) => {
        $crate::dvec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dvec3(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DVec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dvec4<A: crate::Construct<crate::DVec4>>(args: A) -> crate::DVec4 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dvec4`].
#[macro_export]
macro_rules! dvec4 {
    ($a:expr $(,)?) => {
        $crate::dvec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dvec4(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat2<A: crate::Construct<crate::DMat2>>(args: A) -> crate::DMat2 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat2`].
#[macro_export]
macro_rules! dmat2 {
    ($a:expr $(,)?) => {
        $crate::dmat2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat2(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat2x2<A: crate::Construct<crate::DMat2>>(args: A) -> crate::DMat2 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat2x2`].
#[macro_export]
macro_rules! dmat2x2 {
    ($a:expr $(,)?) => {
        $crate::dmat2x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat2x2(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat2x3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat2x3<A: crate::Construct<crate::DMat2x3>>(args: A) -> crate::DMat2x3 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat2x3`].
#[macro_export]
macro_rules! dmat2x3 {
    ($a:expr $(,)?) => {
        $crate::dmat2x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat2x3(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat2x4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat2x4<A: crate::Construct<crate::DMat2x4>>(args: A) -> crate::DMat2x4 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat2x4`].
#[macro_export]
macro_rules! dmat2x4 {
    ($a:expr $(,)?) => {
        $crate::dmat2x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat2x4(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat3x2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat3x2<A: crate::Construct<crate::DMat3x2>>(args: A) -> crate::DMat3x2 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat3x2`].
#[macro_export]
macro_rules! dmat3x2 {
    ($a:expr $(,)?) => {
        $crate::dmat3x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat3x2(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat3<A: crate::Construct<crate::DMat3>>(args: A) -> crate::DMat3 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat3`].
#[macro_export]
macro_rules! dmat3 {
    ($a:expr $(,)?) => {
        $crate::dmat3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat3(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat3x3<A: crate::Construct<crate::DMat3>>(args: A) -> crate::DMat3 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat3x3`].
#[macro_export]
macro_rules! dmat3x3 {
    ($a:expr $(,)?) => {
        $crate::dmat3x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat3x3(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat3x4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat3x4<A: crate::Construct<crate::DMat3x4>>(args: A) -> crate::DMat3x4 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat3x4`].
#[macro_export]
macro_rules! dmat3x4 {
    ($a:expr $(,)?) => {
        $crate::dmat3x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat3x4(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat4x2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat4x2<A: crate::Construct<crate::DMat4x2>>(args: A) -> crate::DMat4x2 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat4x2`].
#[macro_export]
macro_rules! dmat4x2 {
    ($a:expr $(,)?) => {
        $crate::dmat4x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat4x2(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat4x3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat4x3<A: crate::Construct<crate::DMat4x3>>(args: A) -> crate::DMat4x3 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat4x3`].
#[macro_export]
macro_rules! dmat4x3 {
    ($a:expr $(,)?) => {
        $crate::dmat4x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat4x3(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat4<A: crate::Construct<crate::DMat4>>(args: A) -> crate::DMat4 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat4`].
#[macro_export]
macro_rules! dmat4 {
    ($a:expr $(,)?) => {
        $crate::dmat4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat4(($a, $($rest),+))
    };
}

#[cfg(feature = "f64")]
/// Constructs `DMat4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn dmat4x4<A: crate::Construct<crate::DMat4>>(args: A) -> crate::DMat4 {
    args.construct()
}

#[cfg(feature = "f64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`dmat4x4`].
#[macro_export]
macro_rules! dmat4x4 {
    ($a:expr $(,)?) => {
        $crate::dmat4x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::dmat4x4(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16vec2<A: crate::Construct<crate::F16Vec2>>(args: A) -> crate::F16Vec2 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16vec2`].
#[macro_export]
macro_rules! f16vec2 {
    ($a:expr $(,)?) => {
        $crate::f16vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16vec3<A: crate::Construct<crate::F16Vec3>>(args: A) -> crate::F16Vec3 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16vec3`].
#[macro_export]
macro_rules! f16vec3 {
    ($a:expr $(,)?) => {
        $crate::f16vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16vec4<A: crate::Construct<crate::F16Vec4>>(args: A) -> crate::F16Vec4 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16vec4`].
#[macro_export]
macro_rules! f16vec4 {
    ($a:expr $(,)?) => {
        $crate::f16vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16vec4(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat2<A: crate::Construct<crate::F16Mat2>>(args: A) -> crate::F16Mat2 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat2`].
#[macro_export]
macro_rules! f16mat2 {
    ($a:expr $(,)?) => {
        $crate::f16mat2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat2(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat2x2<A: crate::Construct<crate::F16Mat2>>(args: A) -> crate::F16Mat2 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat2x2`].
#[macro_export]
macro_rules! f16mat2x2 {
    ($a:expr $(,)?) => {
        $crate::f16mat2x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat2x2(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat2x3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat2x3<A: crate::Construct<crate::F16Mat2x3>>(args: A) -> crate::F16Mat2x3 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat2x3`].
#[macro_export]
macro_rules! f16mat2x3 {
    ($a:expr $(,)?) => {
        $crate::f16mat2x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat2x3(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat2x4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat2x4<A: crate::Construct<crate::F16Mat2x4>>(args: A) -> crate::F16Mat2x4 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat2x4`].
#[macro_export]
macro_rules! f16mat2x4 {
    ($a:expr $(,)?) => {
        $crate::f16mat2x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat2x4(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat3x2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat3x2<A: crate::Construct<crate::F16Mat3x2>>(args: A) -> crate::F16Mat3x2 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat3x2`].
#[macro_export]
macro_rules! f16mat3x2 {
    ($a:expr $(,)?) => {
        $crate::f16mat3x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat3x2(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat3<A: crate::Construct<crate::F16Mat3>>(args: A) -> crate::F16Mat3 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat3`].
#[macro_export]
macro_rules! f16mat3 {
    ($a:expr $(,)?) => {
        $crate::f16mat3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat3(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat3x3<A: crate::Construct<crate::F16Mat3>>(args: A) -> crate::F16Mat3 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat3x3`].
#[macro_export]
macro_rules! f16mat3x3 {
    ($a:expr $(,)?) => {
        $crate::f16mat3x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat3x3(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat3x4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat3x4<A: crate::Construct<crate::F16Mat3x4>>(args: A) -> crate::F16Mat3x4 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat3x4`].
#[macro_export]
macro_rules! f16mat3x4 {
    ($a:expr $(,)?) => {
        $crate::f16mat3x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat3x4(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat4x2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat4x2<A: crate::Construct<crate::F16Mat4x2>>(args: A) -> crate::F16Mat4x2 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat4x2`].
#[macro_export]
macro_rules! f16mat4x2 {
    ($a:expr $(,)?) => {
        $crate::f16mat4x2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat4x2(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat4x3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat4x3<A: crate::Construct<crate::F16Mat4x3>>(args: A) -> crate::F16Mat4x3 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat4x3`].
#[macro_export]
macro_rules! f16mat4x3 {
    ($a:expr $(,)?) => {
        $crate::f16mat4x3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat4x3(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat4<A: crate::Construct<crate::F16Mat4>>(args: A) -> crate::F16Mat4 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat4`].
#[macro_export]
macro_rules! f16mat4 {
    ($a:expr $(,)?) => {
        $crate::f16mat4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat4(($a, $($rest),+))
    };
}

#[cfg(feature = "f16")]
/// Constructs `F16Mat4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn f16mat4x4<A: crate::Construct<crate::F16Mat4>>(args: A) -> crate::F16Mat4 {
    args.construct()
}

#[cfg(feature = "f16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`f16mat4x4`].
#[macro_export]
macro_rules! f16mat4x4 {
    ($a:expr $(,)?) => {
        $crate::f16mat4x4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::f16mat4x4(($a, $($rest),+))
    };
}

#[cfg(feature = "i8")]
/// Constructs `I8Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i8vec2<A: crate::Construct<crate::I8Vec2>>(args: A) -> crate::I8Vec2 {
    args.construct()
}

#[cfg(feature = "i8")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i8vec2`].
#[macro_export]
macro_rules! i8vec2 {
    ($a:expr $(,)?) => {
        $crate::i8vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i8vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "i8")]
/// Constructs `I8Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i8vec3<A: crate::Construct<crate::I8Vec3>>(args: A) -> crate::I8Vec3 {
    args.construct()
}

#[cfg(feature = "i8")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i8vec3`].
#[macro_export]
macro_rules! i8vec3 {
    ($a:expr $(,)?) => {
        $crate::i8vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i8vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "i8")]
/// Constructs `I8Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i8vec4<A: crate::Construct<crate::I8Vec4>>(args: A) -> crate::I8Vec4 {
    args.construct()
}

#[cfg(feature = "i8")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i8vec4`].
#[macro_export]
macro_rules! i8vec4 {
    ($a:expr $(,)?) => {
        $crate::i8vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i8vec4(($a, $($rest),+))
    };
}

#[cfg(feature = "u8")]
/// Constructs `U8Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u8vec2<A: crate::Construct<crate::U8Vec2>>(args: A) -> crate::U8Vec2 {
    args.construct()
}

#[cfg(feature = "u8")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u8vec2`].
#[macro_export]
macro_rules! u8vec2 {
    ($a:expr $(,)?) => {
        $crate::u8vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u8vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "u8")]
/// Constructs `U8Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u8vec3<A: crate::Construct<crate::U8Vec3>>(args: A) -> crate::U8Vec3 {
    args.construct()
}

#[cfg(feature = "u8")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u8vec3`].
#[macro_export]
macro_rules! u8vec3 {
    ($a:expr $(,)?) => {
        $crate::u8vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u8vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "u8")]
/// Constructs `U8Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u8vec4<A: crate::Construct<crate::U8Vec4>>(args: A) -> crate::U8Vec4 {
    args.construct()
}

#[cfg(feature = "u8")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u8vec4`].
#[macro_export]
macro_rules! u8vec4 {
    ($a:expr $(,)?) => {
        $crate::u8vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u8vec4(($a, $($rest),+))
    };
}

#[cfg(feature = "i16")]
/// Constructs `I16Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i16vec2<A: crate::Construct<crate::I16Vec2>>(args: A) -> crate::I16Vec2 {
    args.construct()
}

#[cfg(feature = "i16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i16vec2`].
#[macro_export]
macro_rules! i16vec2 {
    ($a:expr $(,)?) => {
        $crate::i16vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i16vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "i16")]
/// Constructs `I16Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i16vec3<A: crate::Construct<crate::I16Vec3>>(args: A) -> crate::I16Vec3 {
    args.construct()
}

#[cfg(feature = "i16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i16vec3`].
#[macro_export]
macro_rules! i16vec3 {
    ($a:expr $(,)?) => {
        $crate::i16vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i16vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "i16")]
/// Constructs `I16Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i16vec4<A: crate::Construct<crate::I16Vec4>>(args: A) -> crate::I16Vec4 {
    args.construct()
}

#[cfg(feature = "i16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i16vec4`].
#[macro_export]
macro_rules! i16vec4 {
    ($a:expr $(,)?) => {
        $crate::i16vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i16vec4(($a, $($rest),+))
    };
}

#[cfg(feature = "u16")]
/// Constructs `U16Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u16vec2<A: crate::Construct<crate::U16Vec2>>(args: A) -> crate::U16Vec2 {
    args.construct()
}

#[cfg(feature = "u16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u16vec2`].
#[macro_export]
macro_rules! u16vec2 {
    ($a:expr $(,)?) => {
        $crate::u16vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u16vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "u16")]
/// Constructs `U16Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u16vec3<A: crate::Construct<crate::U16Vec3>>(args: A) -> crate::U16Vec3 {
    args.construct()
}

#[cfg(feature = "u16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u16vec3`].
#[macro_export]
macro_rules! u16vec3 {
    ($a:expr $(,)?) => {
        $crate::u16vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u16vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "u16")]
/// Constructs `U16Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u16vec4<A: crate::Construct<crate::U16Vec4>>(args: A) -> crate::U16Vec4 {
    args.construct()
}

#[cfg(feature = "u16")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u16vec4`].
#[macro_export]
macro_rules! u16vec4 {
    ($a:expr $(,)?) => {
        $crate::u16vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u16vec4(($a, $($rest),+))
    };
}

#[cfg(feature = "i32")]
/// Constructs `IVec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn ivec2<A: crate::Construct<crate::IVec2>>(args: A) -> crate::IVec2 {
    args.construct()
}

#[cfg(feature = "i32")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`ivec2`].
#[macro_export]
macro_rules! ivec2 {
    ($a:expr $(,)?) => {
        $crate::ivec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::ivec2(($a, $($rest),+))
    };
}

#[cfg(feature = "i32")]
/// Constructs `IVec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn ivec3<A: crate::Construct<crate::IVec3>>(args: A) -> crate::IVec3 {
    args.construct()
}

#[cfg(feature = "i32")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`ivec3`].
#[macro_export]
macro_rules! ivec3 {
    ($a:expr $(,)?) => {
        $crate::ivec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::ivec3(($a, $($rest),+))
    };
}

#[cfg(feature = "i32")]
/// Constructs `IVec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn ivec4<A: crate::Construct<crate::IVec4>>(args: A) -> crate::IVec4 {
    args.construct()
}

#[cfg(feature = "i32")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`ivec4`].
#[macro_export]
macro_rules! ivec4 {
    ($a:expr $(,)?) => {
        $crate::ivec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::ivec4(($a, $($rest),+))
    };
}

#[cfg(feature = "u32")]
/// Constructs `UVec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn uvec2<A: crate::Construct<crate::UVec2>>(args: A) -> crate::UVec2 {
    args.construct()
}

#[cfg(feature = "u32")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`uvec2`].
#[macro_export]
macro_rules! uvec2 {
    ($a:expr $(,)?) => {
        $crate::uvec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::uvec2(($a, $($rest),+))
    };
}

#[cfg(feature = "u32")]
/// Constructs `UVec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn uvec3<A: crate::Construct<crate::UVec3>>(args: A) -> crate::UVec3 {
    args.construct()
}

#[cfg(feature = "u32")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`uvec3`].
#[macro_export]
macro_rules! uvec3 {
    ($a:expr $(,)?) => {
        $crate::uvec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::uvec3(($a, $($rest),+))
    };
}

#[cfg(feature = "u32")]
/// Constructs `UVec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn uvec4<A: crate::Construct<crate::UVec4>>(args: A) -> crate::UVec4 {
    args.construct()
}

#[cfg(feature = "u32")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`uvec4`].
#[macro_export]
macro_rules! uvec4 {
    ($a:expr $(,)?) => {
        $crate::uvec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::uvec4(($a, $($rest),+))
    };
}

#[cfg(feature = "i64")]
/// Constructs `I64Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i64vec2<A: crate::Construct<crate::I64Vec2>>(args: A) -> crate::I64Vec2 {
    args.construct()
}

#[cfg(feature = "i64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i64vec2`].
#[macro_export]
macro_rules! i64vec2 {
    ($a:expr $(,)?) => {
        $crate::i64vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i64vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "i64")]
/// Constructs `I64Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i64vec3<A: crate::Construct<crate::I64Vec3>>(args: A) -> crate::I64Vec3 {
    args.construct()
}

#[cfg(feature = "i64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i64vec3`].
#[macro_export]
macro_rules! i64vec3 {
    ($a:expr $(,)?) => {
        $crate::i64vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i64vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "i64")]
/// Constructs `I64Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn i64vec4<A: crate::Construct<crate::I64Vec4>>(args: A) -> crate::I64Vec4 {
    args.construct()
}

#[cfg(feature = "i64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`i64vec4`].
#[macro_export]
macro_rules! i64vec4 {
    ($a:expr $(,)?) => {
        $crate::i64vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::i64vec4(($a, $($rest),+))
    };
}

#[cfg(feature = "u64")]
/// Constructs `U64Vec2` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u64vec2<A: crate::Construct<crate::U64Vec2>>(args: A) -> crate::U64Vec2 {
    args.construct()
}

#[cfg(feature = "u64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u64vec2`].
#[macro_export]
macro_rules! u64vec2 {
    ($a:expr $(,)?) => {
        $crate::u64vec2($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u64vec2(($a, $($rest),+))
    };
}

#[cfg(feature = "u64")]
/// Constructs `U64Vec3` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u64vec3<A: crate::Construct<crate::U64Vec3>>(args: A) -> crate::U64Vec3 {
    args.construct()
}

#[cfg(feature = "u64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u64vec3`].
#[macro_export]
macro_rules! u64vec3 {
    ($a:expr $(,)?) => {
        $crate::u64vec3($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u64vec3(($a, $($rest),+))
    };
}

#[cfg(feature = "u64")]
/// Constructs `U64Vec4` from a scalar splat, tuple, array, or compatible vector/matrix.
#[inline(always)]
#[must_use]
pub fn u64vec4<A: crate::Construct<crate::U64Vec4>>(args: A) -> crate::U64Vec4 {
    args.construct()
}

#[cfg(feature = "u64")]
/// Variadic GLSL-style constructor forwarding 1-or-more arguments to [`u64vec4`].
#[macro_export]
macro_rules! u64vec4 {
    ($a:expr $(,)?) => {
        $crate::u64vec4($a)
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {
        $crate::u64vec4(($a, $($rest),+))
    };
}
