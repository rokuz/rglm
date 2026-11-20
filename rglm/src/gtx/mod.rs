pub mod fast_math;
pub mod norm;
pub mod safe_ops;

pub use fast_math::{
    FastGeomVec, FastMathUnary, fast_cos, fast_exp, fast_inversesqrt, fast_length, fast_log,
    fast_normalize, fast_sin,
};
pub use norm::{
    Distance2Op, Length2Op, VecNormOp, distance2, l_max_norm, l1_norm, l2_norm, length2, lx_norm,
};
pub use safe_ops::{
    SafeInverseOp, SafeLengthClampOp, SafeNormalizeOp, clamp_length, clamp_length_max,
    normalize_or, normalize_or_zero, try_inverse, try_normalize,
};
