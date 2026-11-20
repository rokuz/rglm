use crate::{
    Mat2, Mat2x3, Mat2x3A, Mat2x4, Mat3, Mat3A, Mat3x2, Mat3x4, Mat4, Mat4x2, Mat4x3, Mat4x3A,
    Quat, Vec2, Vec3, Vec3A, Vec4,
};

#[cfg(feature = "f64")]
use crate::{
    DMat2, DMat2x3, DMat2x4, DMat3, DMat3x2, DMat3x4, DMat4, DMat4x2, DMat4x3, DQuat, DVec2, DVec3,
    DVec4,
};

#[cfg(feature = "f16")]
use crate::{
    F16Mat2, F16Mat2x3, F16Mat2x4, F16Mat3, F16Mat3x2, F16Mat3x4, F16Mat4, F16Mat4x2, F16Mat4x3,
    F16Quat, F16Vec2, F16Vec3, F16Vec4, f16,
};

mod sealed {
    pub trait MatrixCompMult {
        fn glsl_matrix_comp_mult(self, y: Self) -> Self;
    }

    pub trait Transpose<Out> {
        fn glsl_transpose(self) -> Out;
    }

    pub trait Determinant<Out> {
        fn glsl_determinant(self) -> Out;
    }

    pub trait Inverse {
        fn glsl_inverse(self) -> Self;
    }

    pub trait OuterProduct<R, Out> {
        fn glsl_outer_product(self, r: R) -> Out;
    }
}

macro_rules! impl_matrix_common {
    ($($mat:ty => $trans:ty),+ $(,)?) => {
        $(
            impl sealed::MatrixCompMult for $mat {
                #[inline(always)]
                fn glsl_matrix_comp_mult(self, y: Self) -> Self {
                    self.matrix_comp_mult(y)
                }
            }

            impl sealed::Transpose<$trans> for $mat {
                #[inline(always)]
                fn glsl_transpose(self) -> $trans {
                    self.transpose()
                }
            }
        )+
    };
}

macro_rules! impl_square_matrix {
    ($($mat:ty => $scalar:ty),+ $(,)?) => {
        $(
            impl sealed::Determinant<$scalar> for $mat {
                #[inline(always)]
                fn glsl_determinant(self) -> $scalar {
                    self.determinant()
                }
            }

            impl sealed::Inverse for $mat {
                #[inline(always)]
                fn glsl_inverse(self) -> Self {
                    self.inverse()
                }
            }
        )+
    };
}

macro_rules! impl_outer_product {
    ($c_vec:ty, $r_vec:ty => $out_mat:ty; $($r_comp:ident),+) => {
        impl sealed::OuterProduct<$r_vec, $out_mat> for $c_vec {
            #[inline(always)]
            fn glsl_outer_product(self, r: $r_vec) -> $out_mat {
                <$out_mat>::from_cols($(self * r.$r_comp),+)
            }
        }
    };
}

impl_matrix_common!(
    Mat2 => Mat2,
    Mat2x3 => Mat3x2,
    Mat2x3A => Mat3x2,
    Mat2x4 => Mat4x2,
    Mat3x2 => Mat2x3,
    Mat3 => Mat3,
    Mat3A => Mat3A,
    Mat3x4 => Mat4x3,
    Mat4x2 => Mat2x4,
    Mat4x3 => Mat3x4,
    Mat4x3A => Mat3x4,
    Mat4 => Mat4,
);

impl_square_matrix!(
    Mat2 => f32,
    Mat3 => f32,
    Mat3A => f32,
    Mat4 => f32,
);

impl sealed::Inverse for Quat {
    #[inline(always)]
    fn glsl_inverse(self) -> Self {
        self.inverse()
    }
}

impl_outer_product!(Vec2, Vec2 => Mat2; x, y);
impl_outer_product!(Vec3, Vec2 => Mat2x3; x, y);
impl_outer_product!(Vec3A, Vec2 => Mat2x3A; x, y);
impl_outer_product!(Vec4, Vec2 => Mat2x4; x, y);
impl_outer_product!(Vec2, Vec3 => Mat3x2; x, y, z);
impl_outer_product!(Vec3, Vec3 => Mat3; x, y, z);
impl_outer_product!(Vec3A, Vec3A => Mat3A; x, y, z);
impl_outer_product!(Vec4, Vec3 => Mat3x4; x, y, z);
impl_outer_product!(Vec2, Vec4 => Mat4x2; x, y, z, w);
impl_outer_product!(Vec3, Vec4 => Mat4x3; x, y, z, w);
impl_outer_product!(Vec3A, Vec4 => Mat4x3A; x, y, z, w);
impl_outer_product!(Vec4, Vec4 => Mat4; x, y, z, w);

#[cfg(feature = "f64")]
impl_matrix_common!(
    DMat2 => DMat2,
    DMat2x3 => DMat3x2,
    DMat2x4 => DMat4x2,
    DMat3x2 => DMat2x3,
    DMat3 => DMat3,
    DMat3x4 => DMat4x3,
    DMat4x2 => DMat2x4,
    DMat4x3 => DMat3x4,
    DMat4 => DMat4,
);

#[cfg(feature = "f64")]
impl_square_matrix!(
    DMat2 => f64,
    DMat3 => f64,
    DMat4 => f64,
);

#[cfg(feature = "f64")]
impl sealed::Inverse for DQuat {
    #[inline(always)]
    fn glsl_inverse(self) -> Self {
        self.inverse()
    }
}

#[cfg(feature = "f64")]
impl_outer_product!(DVec2, DVec2 => DMat2; x, y);
#[cfg(feature = "f64")]
impl_outer_product!(DVec3, DVec2 => DMat2x3; x, y);
#[cfg(feature = "f64")]
impl_outer_product!(DVec4, DVec2 => DMat2x4; x, y);
#[cfg(feature = "f64")]
impl_outer_product!(DVec2, DVec3 => DMat3x2; x, y, z);
#[cfg(feature = "f64")]
impl_outer_product!(DVec3, DVec3 => DMat3; x, y, z);
#[cfg(feature = "f64")]
impl_outer_product!(DVec4, DVec3 => DMat3x4; x, y, z);
#[cfg(feature = "f64")]
impl_outer_product!(DVec2, DVec4 => DMat4x2; x, y, z, w);
#[cfg(feature = "f64")]
impl_outer_product!(DVec3, DVec4 => DMat4x3; x, y, z, w);
#[cfg(feature = "f64")]
impl_outer_product!(DVec4, DVec4 => DMat4; x, y, z, w);

#[cfg(feature = "f16")]
impl_matrix_common!(
    F16Mat2 => F16Mat2,
    F16Mat2x3 => F16Mat3x2,
    F16Mat2x4 => F16Mat4x2,
    F16Mat3x2 => F16Mat2x3,
    F16Mat3 => F16Mat3,
    F16Mat3x4 => F16Mat4x3,
    F16Mat4x2 => F16Mat2x4,
    F16Mat4x3 => F16Mat3x4,
    F16Mat4 => F16Mat4,
);

#[cfg(feature = "f16")]
impl_square_matrix!(
    F16Mat2 => f16,
    F16Mat3 => f16,
    F16Mat4 => f16,
);

#[cfg(feature = "f16")]
impl sealed::Inverse for F16Quat {
    #[inline(always)]
    fn glsl_inverse(self) -> Self {
        self.inverse()
    }
}

#[cfg(feature = "f16")]
impl_outer_product!(F16Vec2, F16Vec2 => F16Mat2; x, y);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec3, F16Vec2 => F16Mat2x3; x, y);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec4, F16Vec2 => F16Mat2x4; x, y);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec2, F16Vec3 => F16Mat3x2; x, y, z);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec3, F16Vec3 => F16Mat3; x, y, z);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec4, F16Vec3 => F16Mat3x4; x, y, z);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec2, F16Vec4 => F16Mat4x2; x, y, z, w);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec3, F16Vec4 => F16Mat4x3; x, y, z, w);
#[cfg(feature = "f16")]
impl_outer_product!(F16Vec4, F16Vec4 => F16Mat4; x, y, z, w);

/// Computes the component-wise product of two matrices `x` and `y` (`result[i][j] = x[i][j] * y[i][j]`).
#[inline(always)]
#[must_use]
pub fn matrix_comp_mult<M: sealed::MatrixCompMult>(x: M, y: M) -> M {
    x.glsl_matrix_comp_mult(y)
}

/// Computes the outer product `c * r^T` of column vector `c` and row vector `r`.
#[inline(always)]
#[must_use]
pub fn outer_product<C, R, Out>(c: C, r: R) -> Out
where
    C: sealed::OuterProduct<R, Out>,
{
    c.glsl_outer_product(r)
}

/// Returns the transpose of matrix `m`, swapping columns and rows (`MatCxR -> MatRxC`).
#[inline(always)]
#[must_use]
pub fn transpose<M, Out>(m: M) -> Out
where
    M: sealed::Transpose<Out>,
{
    m.glsl_transpose()
}

/// Computes the scalar determinant of square matrix `m`.
#[inline(always)]
#[must_use]
pub fn determinant<M, Out>(m: M) -> Out
where
    M: sealed::Determinant<Out>,
{
    m.glsl_determinant()
}

/// Computes the multiplicative inverse of a square matrix or quaternion `x`.
#[inline(always)]
#[must_use]
pub fn inverse<T: sealed::Inverse>(x: T) -> T {
    x.glsl_inverse()
}
