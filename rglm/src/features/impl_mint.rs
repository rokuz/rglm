macro_rules! impl_mint_vec2 {
    ($ty:ty, $scalar:ty) => {
        impl From<mint::Vector2<$scalar>> for $ty {
            #[inline]
            fn from(v: mint::Vector2<$scalar>) -> Self {
                Self::from([v.x, v.y])
            }
        }
        impl From<$ty> for mint::Vector2<$scalar> {
            #[inline]
            fn from(v: $ty) -> Self {
                let [x, y]: [$scalar; 2] = v.into();
                Self { x, y }
            }
        }
    };
}

macro_rules! impl_mint_vec3 {
    ($ty:ty, $scalar:ty) => {
        impl From<mint::Vector3<$scalar>> for $ty {
            #[inline]
            fn from(v: mint::Vector3<$scalar>) -> Self {
                Self::from([v.x, v.y, v.z])
            }
        }
        impl From<$ty> for mint::Vector3<$scalar> {
            #[inline]
            fn from(v: $ty) -> Self {
                let [x, y, z]: [$scalar; 3] = v.into();
                Self { x, y, z }
            }
        }
    };
}

macro_rules! impl_mint_vec4 {
    ($ty:ty, $scalar:ty) => {
        impl From<mint::Vector4<$scalar>> for $ty {
            #[inline]
            fn from(v: mint::Vector4<$scalar>) -> Self {
                Self::from([v.x, v.y, v.z, v.w])
            }
        }
        impl From<$ty> for mint::Vector4<$scalar> {
            #[inline]
            fn from(v: $ty) -> Self {
                let [x, y, z, w]: [$scalar; 4] = v.into();
                Self { x, y, z, w }
            }
        }
    };
}

impl_mint_vec2!(crate::Vec2, f32);
impl_mint_vec3!(crate::Vec3, f32);
impl_mint_vec3!(crate::Vec3A, f32);
impl_mint_vec4!(crate::Vec4, f32);

#[cfg(feature = "f64")]
impl_mint_vec2!(crate::DVec2, f64);
#[cfg(feature = "f64")]
impl_mint_vec3!(crate::DVec3, f64);
#[cfg(feature = "f64")]
impl_mint_vec4!(crate::DVec4, f64);

#[cfg(feature = "i32")]
impl_mint_vec2!(crate::IVec2, i32);
#[cfg(feature = "i32")]
impl_mint_vec3!(crate::IVec3, i32);
#[cfg(feature = "i32")]
impl_mint_vec4!(crate::IVec4, i32);

#[cfg(feature = "u32")]
impl_mint_vec2!(crate::UVec2, u32);
#[cfg(feature = "u32")]
impl_mint_vec3!(crate::UVec3, u32);
#[cfg(feature = "u32")]
impl_mint_vec4!(crate::UVec4, u32);

impl From<mint::Quaternion<f32>> for crate::Quat {
    #[inline]
    fn from(q: mint::Quaternion<f32>) -> Self {
        Self::from([q.v.x, q.v.y, q.v.z, q.s])
    }
}

impl From<crate::Quat> for mint::Quaternion<f32> {
    #[inline]
    fn from(q: crate::Quat) -> Self {
        let [x, y, z, s]: [f32; 4] = q.into();
        Self {
            v: mint::Vector3 { x, y, z },
            s,
        }
    }
}
