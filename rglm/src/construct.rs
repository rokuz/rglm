mod private {
    pub trait Target {}
}

pub trait Construct<Out: private::Target> {
    fn construct(self) -> Out;
}

trait ScalarConv<Dst>: Copy {
    fn sconv(self) -> Dst;
}

impl<T: Copy> ScalarConv<T> for T {
    #[inline(always)]
    fn sconv(self) -> T {
        self
    }
}

macro_rules! impl_num_conv {
    ($src:ty => $($dst:ty),+ $(,)?) => {
        $(
            impl ScalarConv<$dst> for $src {
                #[inline(always)]
                fn sconv(self) -> $dst {
                    self as $dst
                }
            }
        )+
    };
}

macro_rules! impl_bool_conv {
    ($num:ty, $zero:expr, $one:expr) => {
        impl ScalarConv<bool> for $num {
            #[inline(always)]
            fn sconv(self) -> bool {
                self != $zero
            }
        }

        impl ScalarConv<$num> for bool {
            #[inline(always)]
            fn sconv(self) -> $num {
                if self { $one } else { $zero }
            }
        }
    };
}

impl_bool_conv!(f32, 0.0, 1.0);
#[cfg(feature = "f64")]
impl_bool_conv!(f64, 0.0, 1.0);
#[cfg(feature = "f16")]
impl_bool_conv!(half::f16, half::f16::ZERO, half::f16::ONE);
#[cfg(feature = "i8")]
impl_bool_conv!(i8, 0, 1);
#[cfg(feature = "u8")]
impl_bool_conv!(u8, 0, 1);
#[cfg(feature = "i16")]
impl_bool_conv!(i16, 0, 1);
#[cfg(feature = "u16")]
impl_bool_conv!(u16, 0, 1);
#[cfg(feature = "i32")]
impl_bool_conv!(i32, 0, 1);
#[cfg(feature = "u32")]
impl_bool_conv!(u32, 0, 1);
#[cfg(feature = "i64")]
impl_bool_conv!(i64, 0, 1);
#[cfg(feature = "u64")]
impl_bool_conv!(u64, 0, 1);

#[cfg(feature = "f64")]
impl_num_conv!(f32 => f64);
#[cfg(feature = "f64")]
impl_num_conv!(f64 => f32);

#[cfg(feature = "f16")]
impl ScalarConv<f32> for half::f16 {
    #[inline(always)]
    fn sconv(self) -> f32 {
        self.to_f32()
    }
}

#[cfg(feature = "f16")]
impl ScalarConv<half::f16> for f32 {
    #[inline(always)]
    fn sconv(self) -> half::f16 {
        half::f16::from_f32(self)
    }
}

#[cfg(all(feature = "f16", feature = "f64"))]
impl ScalarConv<f64> for half::f16 {
    #[inline(always)]
    fn sconv(self) -> f64 {
        self.to_f64()
    }
}

#[cfg(all(feature = "f16", feature = "f64"))]
impl ScalarConv<half::f16> for f64 {
    #[inline(always)]
    fn sconv(self) -> half::f16 {
        half::f16::from_f64(self)
    }
}

macro_rules! impl_int_float_conv {
    ($int_t:ty, $feat:literal) => {
        #[cfg(feature = $feat)]
        impl_num_conv!($int_t => f32);
        #[cfg(feature = $feat)]
        impl_num_conv!(f32 => $int_t);

        #[cfg(all(feature = $feat, feature = "f64"))]
        impl_num_conv!($int_t => f64);
        #[cfg(all(feature = $feat, feature = "f64"))]
        impl_num_conv!(f64 => $int_t);

        #[cfg(all(feature = $feat, feature = "f16"))]
        impl ScalarConv<half::f16> for $int_t {
            #[inline(always)]
            fn sconv(self) -> half::f16 {
                half::f16::from_f32(self as f32)
            }
        }

        #[cfg(all(feature = $feat, feature = "f16"))]
        impl ScalarConv<$int_t> for half::f16 {
            #[inline(always)]
            fn sconv(self) -> $int_t {
                self.to_f32() as $int_t
            }
        }
    };
}

impl_int_float_conv!(i8, "i8");
impl_int_float_conv!(u8, "u8");
impl_int_float_conv!(i16, "i16");
impl_int_float_conv!(u16, "u16");
impl_int_float_conv!(i32, "i32");
impl_int_float_conv!(u32, "u32");
impl_int_float_conv!(i64, "i64");
impl_int_float_conv!(u64, "u64");

macro_rules! impl_int_pair {
    ($a:ty, $fa:literal, $b:ty, $fb:literal) => {
        #[cfg(all(feature = $fa, feature = $fb))]
        impl_num_conv!($a => $b);
        #[cfg(all(feature = $fa, feature = $fb))]
        impl_num_conv!($b => $a);
    };
}

impl_int_pair!(i8, "i8", u8, "u8");
impl_int_pair!(i8, "i8", i16, "i16");
impl_int_pair!(i8, "i8", u16, "u16");
impl_int_pair!(i8, "i8", i32, "i32");
impl_int_pair!(i8, "i8", u32, "u32");
impl_int_pair!(i8, "i8", i64, "i64");
impl_int_pair!(i8, "i8", u64, "u64");
impl_int_pair!(u8, "u8", i16, "i16");
impl_int_pair!(u8, "u8", u16, "u16");
impl_int_pair!(u8, "u8", i32, "i32");
impl_int_pair!(u8, "u8", u32, "u32");
impl_int_pair!(u8, "u8", i64, "i64");
impl_int_pair!(u8, "u8", u64, "u64");
impl_int_pair!(i16, "i16", u16, "u16");
impl_int_pair!(i16, "i16", i32, "i32");
impl_int_pair!(i16, "i16", u32, "u32");
impl_int_pair!(i16, "i16", i64, "i64");
impl_int_pair!(i16, "i16", u64, "u64");
impl_int_pair!(u16, "u16", i32, "i32");
impl_int_pair!(u16, "u16", u32, "u32");
impl_int_pair!(u16, "u16", i64, "i64");
impl_int_pair!(u16, "u16", u64, "u64");
impl_int_pair!(i32, "i32", u32, "u32");
impl_int_pair!(i32, "i32", i64, "i64");
impl_int_pair!(i32, "i32", u64, "u64");
impl_int_pair!(u32, "u32", i64, "i64");
impl_int_pair!(u32, "u32", u64, "u64");
impl_int_pair!(i64, "i64", u64, "u64");

macro_rules! impl_vec_family_construct {
    ($s:ty, $v2:ty, $v3:ty, $v4:ty) => {
        impl private::Target for $v2 {}
        impl private::Target for $v3 {}
        impl private::Target for $v4 {}

        impl Construct<$v2> for $s {
            #[inline(always)]
            fn construct(self) -> $v2 {
                <$v2>::splat(self)
            }
        }

        impl Construct<$v2> for ($s, $s) {
            #[inline(always)]
            fn construct(self) -> $v2 {
                <$v2>::new(self.0, self.1)
            }
        }

        impl Construct<$v2> for [$s; 2] {
            #[inline(always)]
            fn construct(self) -> $v2 {
                <$v2>::from_array(self)
            }
        }

        impl Construct<$v2> for $v2 {
            #[inline(always)]
            fn construct(self) -> $v2 {
                self
            }
        }

        impl Construct<$v2> for $v3 {
            #[inline(always)]
            fn construct(self) -> $v2 {
                let a = self.to_array();
                <$v2>::new(a[0], a[1])
            }
        }

        impl Construct<$v2> for $v4 {
            #[inline(always)]
            fn construct(self) -> $v2 {
                let a = self.to_array();
                <$v2>::new(a[0], a[1])
            }
        }

        impl Construct<$v3> for $s {
            #[inline(always)]
            fn construct(self) -> $v3 {
                <$v3>::splat(self)
            }
        }

        impl Construct<$v3> for ($s, $s, $s) {
            #[inline(always)]
            fn construct(self) -> $v3 {
                <$v3>::new(self.0, self.1, self.2)
            }
        }

        impl Construct<$v3> for ($v2, $s) {
            #[inline(always)]
            fn construct(self) -> $v3 {
                let a = self.0.to_array();
                <$v3>::new(a[0], a[1], self.1)
            }
        }

        impl Construct<$v3> for ($s, $v2) {
            #[inline(always)]
            fn construct(self) -> $v3 {
                let a = self.1.to_array();
                <$v3>::new(self.0, a[0], a[1])
            }
        }

        impl Construct<$v3> for [$s; 3] {
            #[inline(always)]
            fn construct(self) -> $v3 {
                <$v3>::from_array(self)
            }
        }

        impl Construct<$v3> for $v3 {
            #[inline(always)]
            fn construct(self) -> $v3 {
                self
            }
        }

        impl Construct<$v3> for $v4 {
            #[inline(always)]
            fn construct(self) -> $v3 {
                let a = self.to_array();
                <$v3>::new(a[0], a[1], a[2])
            }
        }

        impl Construct<$v4> for $s {
            #[inline(always)]
            fn construct(self) -> $v4 {
                <$v4>::splat(self)
            }
        }

        impl Construct<$v4> for ($s, $s, $s, $s) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                <$v4>::new(self.0, self.1, self.2, self.3)
            }
        }

        impl Construct<$v4> for ($v2, $s, $s) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                let a = self.0.to_array();
                <$v4>::new(a[0], a[1], self.1, self.2)
            }
        }

        impl Construct<$v4> for ($s, $v2, $s) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                let a = self.1.to_array();
                <$v4>::new(self.0, a[0], a[1], self.2)
            }
        }

        impl Construct<$v4> for ($s, $s, $v2) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                let a = self.2.to_array();
                <$v4>::new(self.0, self.1, a[0], a[1])
            }
        }

        impl Construct<$v4> for ($v2, $v2) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                let a = self.0.to_array();
                let b = self.1.to_array();
                <$v4>::new(a[0], a[1], b[0], b[1])
            }
        }

        impl Construct<$v4> for ($v3, $s) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                let a = self.0.to_array();
                <$v4>::new(a[0], a[1], a[2], self.1)
            }
        }

        impl Construct<$v4> for ($s, $v3) {
            #[inline(always)]
            fn construct(self) -> $v4 {
                let a = self.1.to_array();
                <$v4>::new(self.0, a[0], a[1], a[2])
            }
        }

        impl Construct<$v4> for [$s; 4] {
            #[inline(always)]
            fn construct(self) -> $v4 {
                <$v4>::from_array(self)
            }
        }

        impl Construct<$v4> for $v4 {
            #[inline(always)]
            fn construct(self) -> $v4 {
                self
            }
        }
    };
}

impl_vec_family_construct!(f32, crate::Vec2, crate::Vec3, crate::Vec4);
impl_vec_family_construct!(bool, crate::BVec2, crate::BVec3, crate::BVec4);

#[cfg(feature = "f64")]
impl_vec_family_construct!(f64, crate::DVec2, crate::DVec3, crate::DVec4);
#[cfg(feature = "f16")]
impl_vec_family_construct!(half::f16, crate::F16Vec2, crate::F16Vec3, crate::F16Vec4);
#[cfg(feature = "i8")]
impl_vec_family_construct!(i8, crate::I8Vec2, crate::I8Vec3, crate::I8Vec4);
#[cfg(feature = "u8")]
impl_vec_family_construct!(u8, crate::U8Vec2, crate::U8Vec3, crate::U8Vec4);
#[cfg(feature = "i16")]
impl_vec_family_construct!(i16, crate::I16Vec2, crate::I16Vec3, crate::I16Vec4);
#[cfg(feature = "u16")]
impl_vec_family_construct!(u16, crate::U16Vec2, crate::U16Vec3, crate::U16Vec4);
#[cfg(feature = "i32")]
impl_vec_family_construct!(i32, crate::IVec2, crate::IVec3, crate::IVec4);
#[cfg(feature = "u32")]
impl_vec_family_construct!(u32, crate::UVec2, crate::UVec3, crate::UVec4);
#[cfg(feature = "i64")]
impl_vec_family_construct!(i64, crate::I64Vec2, crate::I64Vec3, crate::I64Vec4);
#[cfg(feature = "u64")]
impl_vec_family_construct!(u64, crate::U64Vec2, crate::U64Vec3, crate::U64Vec4);

#[cfg(feature = "f16")]
impl Construct<crate::F16Vec2> for f32 {
    #[inline(always)]
    fn construct(self) -> crate::F16Vec2 {
        crate::F16Vec2::splat(half::f16::from_f32(self))
    }
}

#[cfg(feature = "f16")]
impl Construct<crate::F16Vec2> for (f32, f32) {
    #[inline(always)]
    fn construct(self) -> crate::F16Vec2 {
        crate::F16Vec2::new(half::f16::from_f32(self.0), half::f16::from_f32(self.1))
    }
}

#[cfg(feature = "f16")]
impl Construct<crate::F16Vec3> for f32 {
    #[inline(always)]
    fn construct(self) -> crate::F16Vec3 {
        crate::F16Vec3::splat(half::f16::from_f32(self))
    }
}

#[cfg(feature = "f16")]
impl Construct<crate::F16Vec3> for (f32, f32, f32) {
    #[inline(always)]
    fn construct(self) -> crate::F16Vec3 {
        crate::F16Vec3::new(
            half::f16::from_f32(self.0),
            half::f16::from_f32(self.1),
            half::f16::from_f32(self.2),
        )
    }
}

#[cfg(feature = "f16")]
impl Construct<crate::F16Vec4> for f32 {
    #[inline(always)]
    fn construct(self) -> crate::F16Vec4 {
        crate::F16Vec4::splat(half::f16::from_f32(self))
    }
}

#[cfg(feature = "f16")]
impl Construct<crate::F16Vec4> for (f32, f32, f32, f32) {
    #[inline(always)]
    fn construct(self) -> crate::F16Vec4 {
        crate::F16Vec4::new(
            half::f16::from_f32(self.0),
            half::f16::from_f32(self.1),
            half::f16::from_f32(self.2),
            half::f16::from_f32(self.3),
        )
    }
}

impl private::Target for crate::Vec3A {}
impl private::Target for crate::BVec3A {}
impl private::Target for crate::BVec4A {}

impl Construct<crate::Vec3A> for f32 {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        crate::Vec3A::splat(self)
    }
}

impl Construct<crate::Vec3A> for (f32, f32, f32) {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        crate::Vec3A::new(self.0, self.1, self.2)
    }
}

impl Construct<crate::Vec3A> for (crate::Vec2, f32) {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        crate::Vec3A::new(self.0.x, self.0.y, self.1)
    }
}

impl Construct<crate::Vec3A> for (f32, crate::Vec2) {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        crate::Vec3A::new(self.0, self.1.x, self.1.y)
    }
}

impl Construct<crate::Vec3A> for [f32; 3] {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        crate::Vec3A::from_array(self)
    }
}

impl Construct<crate::Vec3A> for crate::Vec3A {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        self
    }
}

impl Construct<crate::Vec3A> for crate::Vec3 {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        crate::Vec3A::from(self)
    }
}

impl Construct<crate::Vec3> for crate::Vec3A {
    #[inline(always)]
    fn construct(self) -> crate::Vec3 {
        crate::Vec3::from(self)
    }
}

impl Construct<crate::Vec3A> for crate::Vec4 {
    #[inline(always)]
    fn construct(self) -> crate::Vec3A {
        let a = self.to_array();
        crate::Vec3A::new(a[0], a[1], a[2])
    }
}

impl Construct<crate::Vec2> for crate::Vec3A {
    #[inline(always)]
    fn construct(self) -> crate::Vec2 {
        crate::Vec2::new(self.x, self.y)
    }
}

impl Construct<crate::Vec4> for (crate::Vec3A, f32) {
    #[inline(always)]
    fn construct(self) -> crate::Vec4 {
        crate::Vec4::new(self.0.x, self.0.y, self.0.z, self.1)
    }
}

impl Construct<crate::Vec4> for (f32, crate::Vec3A) {
    #[inline(always)]
    fn construct(self) -> crate::Vec4 {
        crate::Vec4::new(self.0, self.1.x, self.1.y, self.1.z)
    }
}

impl Construct<crate::BVec3A> for bool {
    #[inline(always)]
    fn construct(self) -> crate::BVec3A {
        crate::BVec3A::splat(self)
    }
}

impl Construct<crate::BVec3A> for (bool, bool, bool) {
    #[inline(always)]
    fn construct(self) -> crate::BVec3A {
        crate::BVec3A::new(self.0, self.1, self.2)
    }
}

impl Construct<crate::BVec3A> for [bool; 3] {
    #[inline(always)]
    fn construct(self) -> crate::BVec3A {
        crate::BVec3A::from_array(self)
    }
}

impl Construct<crate::BVec3A> for crate::BVec3A {
    #[inline(always)]
    fn construct(self) -> crate::BVec3A {
        self
    }
}

impl Construct<crate::BVec3A> for crate::BVec3 {
    #[inline(always)]
    fn construct(self) -> crate::BVec3A {
        crate::BVec3A::from(self)
    }
}

impl Construct<crate::BVec3> for crate::BVec3A {
    #[inline(always)]
    fn construct(self) -> crate::BVec3 {
        crate::BVec3::from(self)
    }
}

impl Construct<crate::BVec4A> for bool {
    #[inline(always)]
    fn construct(self) -> crate::BVec4A {
        crate::BVec4A::splat(self)
    }
}

impl Construct<crate::BVec4A> for (bool, bool, bool, bool) {
    #[inline(always)]
    fn construct(self) -> crate::BVec4A {
        crate::BVec4A::new(self.0, self.1, self.2, self.3)
    }
}

impl Construct<crate::BVec4A> for [bool; 4] {
    #[inline(always)]
    fn construct(self) -> crate::BVec4A {
        crate::BVec4A::from_array(self)
    }
}

impl Construct<crate::BVec4A> for crate::BVec4A {
    #[inline(always)]
    fn construct(self) -> crate::BVec4A {
        self
    }
}

impl Construct<crate::BVec4A> for crate::BVec4 {
    #[inline(always)]
    fn construct(self) -> crate::BVec4A {
        crate::BVec4A::from(self)
    }
}

impl Construct<crate::BVec4> for crate::BVec4A {
    #[inline(always)]
    fn construct(self) -> crate::BVec4 {
        crate::BVec4::from(self)
    }
}

macro_rules! impl_vec_cast_pair {
    ($a2:ty, $a3:ty, $a4:ty, $b2:ty, $b3:ty, $b4:ty) => {
        impl Construct<$b2> for $a2 {
            #[inline(always)]
            fn construct(self) -> $b2 {
                <$b2>::from_array(self.to_array().map(ScalarConv::sconv))
            }
        }
        impl Construct<$a2> for $b2 {
            #[inline(always)]
            fn construct(self) -> $a2 {
                <$a2>::from_array(self.to_array().map(ScalarConv::sconv))
            }
        }
        impl Construct<$b3> for $a3 {
            #[inline(always)]
            fn construct(self) -> $b3 {
                <$b3>::from_array(self.to_array().map(ScalarConv::sconv))
            }
        }
        impl Construct<$a3> for $b3 {
            #[inline(always)]
            fn construct(self) -> $a3 {
                <$a3>::from_array(self.to_array().map(ScalarConv::sconv))
            }
        }
        impl Construct<$b4> for $a4 {
            #[inline(always)]
            fn construct(self) -> $b4 {
                <$b4>::from_array(self.to_array().map(ScalarConv::sconv))
            }
        }
        impl Construct<$a4> for $b4 {
            #[inline(always)]
            fn construct(self) -> $a4 {
                <$a4>::from_array(self.to_array().map(ScalarConv::sconv))
            }
        }
    };
}

impl_vec_cast_pair!(
    crate::Vec2,
    crate::Vec3,
    crate::Vec4,
    crate::BVec2,
    crate::BVec3,
    crate::BVec4
);

macro_rules! impl_vec_casts_with_feat {
    ($feat:literal, $v2:ty, $v3:ty, $v4:ty) => {
        #[cfg(feature = $feat)]
        impl_vec_cast_pair!(crate::Vec2, crate::Vec3, crate::Vec4, $v2, $v3, $v4);
        #[cfg(feature = $feat)]
        impl_vec_cast_pair!(crate::BVec2, crate::BVec3, crate::BVec4, $v2, $v3, $v4);
    };
}

impl_vec_casts_with_feat!("f64", crate::DVec2, crate::DVec3, crate::DVec4);
impl_vec_casts_with_feat!("f16", crate::F16Vec2, crate::F16Vec3, crate::F16Vec4);
impl_vec_casts_with_feat!("i8", crate::I8Vec2, crate::I8Vec3, crate::I8Vec4);
impl_vec_casts_with_feat!("u8", crate::U8Vec2, crate::U8Vec3, crate::U8Vec4);
impl_vec_casts_with_feat!("i16", crate::I16Vec2, crate::I16Vec3, crate::I16Vec4);
impl_vec_casts_with_feat!("u16", crate::U16Vec2, crate::U16Vec3, crate::U16Vec4);
impl_vec_casts_with_feat!("i32", crate::IVec2, crate::IVec3, crate::IVec4);
impl_vec_casts_with_feat!("u32", crate::UVec2, crate::UVec3, crate::UVec4);
impl_vec_casts_with_feat!("i64", crate::I64Vec2, crate::I64Vec3, crate::I64Vec4);
impl_vec_casts_with_feat!("u64", crate::U64Vec2, crate::U64Vec3, crate::U64Vec4);

#[cfg(all(feature = "i32", feature = "u32"))]
impl_vec_cast_pair!(
    crate::IVec2,
    crate::IVec3,
    crate::IVec4,
    crate::UVec2,
    crate::UVec3,
    crate::UVec4
);

#[cfg(all(feature = "f64", feature = "i32"))]
impl_vec_cast_pair!(
    crate::DVec2,
    crate::DVec3,
    crate::DVec4,
    crate::IVec2,
    crate::IVec3,
    crate::IVec4
);

#[cfg(all(feature = "f64", feature = "u32"))]
impl_vec_cast_pair!(
    crate::DVec2,
    crate::DVec3,
    crate::DVec4,
    crate::UVec2,
    crate::UVec3,
    crate::UVec4
);

#[cfg(all(feature = "f64", feature = "f16"))]
impl_vec_cast_pair!(
    crate::DVec2,
    crate::DVec3,
    crate::DVec4,
    crate::F16Vec2,
    crate::F16Vec3,
    crate::F16Vec4
);

trait MatGrid: Copy {
    type S: Copy;
    const COLS: usize;
    const ROWS: usize;
    const ZERO_S: Self::S;
    const ONE_S: Self::S;
    fn cell(&self, col: usize, row: usize) -> Self::S;
    fn from_fn(f: impl FnMut(usize, usize) -> Self::S) -> Self;
}

macro_rules! impl_mat_grid_and_base {
    ($mat_t:ty, $s:ty, $col_t:ty, $cols:literal, $rows:literal, $len:literal, $zero:expr, $one:expr, $col_tup:tt, $col_construct:expr, $flat_tup:tt, $flat_construct:expr) => {
        impl private::Target for $mat_t {}

        impl MatGrid for $mat_t {
            type S = $s;
            const COLS: usize = $cols;
            const ROWS: usize = $rows;
            const ZERO_S: $s = $zero;
            const ONE_S: $s = $one;

            #[inline(always)]
            fn cell(&self, col: usize, row: usize) -> $s {
                self.col(col).to_array()[row]
            }

            #[inline(always)]
            fn from_fn(mut f: impl FnMut(usize, usize) -> $s) -> Self {
                let mut arr = [$zero; $len];
                let mut c = 0;
                while c < $cols {
                    let mut r = 0;
                    while r < $rows {
                        arr[c * $rows + r] = f(c, r);
                        r += 1;
                    }
                    c += 1;
                }
                Self::from_cols_array(arr)
            }
        }

        impl Construct<$mat_t> for $s {
            #[inline(always)]
            fn construct(self) -> $mat_t {
                <$mat_t>::diagonal(self)
            }
        }

        impl Construct<$mat_t> for [$s; $len] {
            #[inline(always)]
            fn construct(self) -> $mat_t {
                <$mat_t>::from_cols_array(self)
            }
        }

        impl Construct<$mat_t> for [[$s; $rows]; $cols] {
            #[inline(always)]
            fn construct(self) -> $mat_t {
                <$mat_t>::from_cols_array_2d(self)
            }
        }

        impl Construct<$mat_t> for $col_tup {
            #[inline(always)]
            fn construct(self) -> $mat_t {
                ($col_construct)(self)
            }
        }

        impl Construct<$mat_t> for $flat_tup {
            #[inline(always)]
            fn construct(self) -> $mat_t {
                ($flat_construct)(self)
            }
        }

        impl Construct<$mat_t> for $mat_t {
            #[inline(always)]
            fn construct(self) -> $mat_t {
                self
            }
        }
    };
}

macro_rules! impl_mat_family_base {
    ($s:ty, $zero:expr, $one:expr, $v2:ty, $v3:ty, $v4:ty, $m2:ty, $m2x3:ty, $m2x4:ty, $m3x2:ty, $m3:ty, $m3x4:ty, $m4x2:ty, $m4x3:ty, $m4:ty) => {
        impl_mat_grid_and_base!(
            $m2,
            $s,
            $v2,
            2,
            2,
            4,
            $zero,
            $one,
            ($v2, $v2),
            |c: ($v2, $v2)| <$m2>::from_cols(c.0, c.1),
            ($s, $s, $s, $s),
            |t: ($s, $s, $s, $s)| <$m2>::from_cols_array([t.0, t.1, t.2, t.3])
        );
        impl_mat_grid_and_base!(
            $m2x3,
            $s,
            $v3,
            2,
            3,
            6,
            $zero,
            $one,
            ($v3, $v3),
            |c: ($v3, $v3)| <$m2x3>::from_cols(c.0, c.1),
            ($s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s)| <$m2x3>::from_cols_array([t.0, t.1, t.2, t.3, t.4, t.5])
        );
        impl_mat_grid_and_base!(
            $m2x4,
            $s,
            $v4,
            2,
            4,
            8,
            $zero,
            $one,
            ($v4, $v4),
            |c: ($v4, $v4)| <$m2x4>::from_cols(c.0, c.1),
            ($s, $s, $s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s, $s, $s)| <$m2x4>::from_cols_array([
                t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7
            ])
        );
        impl_mat_grid_and_base!(
            $m3x2,
            $s,
            $v2,
            3,
            2,
            6,
            $zero,
            $one,
            ($v2, $v2, $v2),
            |c: ($v2, $v2, $v2)| <$m3x2>::from_cols(c.0, c.1, c.2),
            ($s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s)| <$m3x2>::from_cols_array([t.0, t.1, t.2, t.3, t.4, t.5])
        );
        impl_mat_grid_and_base!(
            $m3,
            $s,
            $v3,
            3,
            3,
            9,
            $zero,
            $one,
            ($v3, $v3, $v3),
            |c: ($v3, $v3, $v3)| <$m3>::from_cols(c.0, c.1, c.2),
            ($s, $s, $s, $s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s, $s, $s, $s)| <$m3>::from_cols_array([
                t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8
            ])
        );
        impl_mat_grid_and_base!(
            $m3x4,
            $s,
            $v4,
            3,
            4,
            12,
            $zero,
            $one,
            ($v4, $v4, $v4),
            |c: ($v4, $v4, $v4)| <$m3x4>::from_cols(c.0, c.1, c.2),
            ($s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s)| <$m3x4>::from_cols_array([
                t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8, t.9, t.10, t.11
            ])
        );
        impl_mat_grid_and_base!(
            $m4x2,
            $s,
            $v2,
            4,
            2,
            8,
            $zero,
            $one,
            ($v2, $v2, $v2, $v2),
            |c: ($v2, $v2, $v2, $v2)| <$m4x2>::from_cols(c.0, c.1, c.2, c.3),
            ($s, $s, $s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s, $s, $s)| <$m4x2>::from_cols_array([
                t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7
            ])
        );
        impl_mat_grid_and_base!(
            $m4x3,
            $s,
            $v3,
            4,
            3,
            12,
            $zero,
            $one,
            ($v3, $v3, $v3, $v3),
            |c: ($v3, $v3, $v3, $v3)| <$m4x3>::from_cols(c.0, c.1, c.2, c.3),
            ($s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s),
            |t: ($s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s)| <$m4x3>::from_cols_array([
                t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8, t.9, t.10, t.11
            ])
        );
        impl_mat_grid_and_base!(
            $m4,
            $s,
            $v4,
            4,
            4,
            16,
            $zero,
            $one,
            ($v4, $v4, $v4, $v4),
            |c: ($v4, $v4, $v4, $v4)| <$m4>::from_cols(c.0, c.1, c.2, c.3),
            (
                $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s, $s
            ),
            |t: (
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s,
                $s
            )| <$m4>::from_cols_array([
                t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8, t.9, t.10, t.11, t.12, t.13, t.14,
                t.15
            ])
        );
    };
}

impl_mat_family_base!(
    f32,
    0.0,
    1.0,
    crate::Vec2,
    crate::Vec3,
    crate::Vec4,
    crate::Mat2,
    crate::Mat2x3,
    crate::Mat2x4,
    crate::Mat3x2,
    crate::Mat3,
    crate::Mat3x4,
    crate::Mat4x2,
    crate::Mat4x3,
    crate::Mat4
);

impl_mat_grid_and_base!(
    crate::Mat2x3A,
    f32,
    crate::Vec3A,
    2,
    3,
    6,
    0.0,
    1.0,
    (crate::Vec3A, crate::Vec3A),
    |c: (crate::Vec3A, crate::Vec3A)| crate::Mat2x3A::from_cols(c.0, c.1),
    (f32, f32, f32, f32, f32, f32),
    |t: (f32, f32, f32, f32, f32, f32)| crate::Mat2x3A::from_cols_array([
        t.0, t.1, t.2, t.3, t.4, t.5
    ])
);

impl_mat_grid_and_base!(
    crate::Mat3A,
    f32,
    crate::Vec3A,
    3,
    3,
    9,
    0.0,
    1.0,
    (crate::Vec3A, crate::Vec3A, crate::Vec3A),
    |c: (crate::Vec3A, crate::Vec3A, crate::Vec3A)| crate::Mat3A::from_cols(c.0, c.1, c.2),
    (f32, f32, f32, f32, f32, f32, f32, f32, f32),
    |t: (f32, f32, f32, f32, f32, f32, f32, f32, f32)| crate::Mat3A::from_cols_array([
        t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8
    ])
);

impl_mat_grid_and_base!(
    crate::Mat4x3A,
    f32,
    crate::Vec3A,
    4,
    3,
    12,
    0.0,
    1.0,
    (crate::Vec3A, crate::Vec3A, crate::Vec3A, crate::Vec3A),
    |c: (crate::Vec3A, crate::Vec3A, crate::Vec3A, crate::Vec3A)| crate::Mat4x3A::from_cols(
        c.0, c.1, c.2, c.3
    ),
    (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32),
    |t: (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)| {
        crate::Mat4x3A::from_cols_array([
            t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8, t.9, t.10, t.11,
        ])
    }
);

#[cfg(feature = "f64")]
impl_mat_family_base!(
    f64,
    0.0,
    1.0,
    crate::DVec2,
    crate::DVec3,
    crate::DVec4,
    crate::DMat2,
    crate::DMat2x3,
    crate::DMat2x4,
    crate::DMat3x2,
    crate::DMat3,
    crate::DMat3x4,
    crate::DMat4x2,
    crate::DMat4x3,
    crate::DMat4
);

#[cfg(feature = "f16")]
impl_mat_family_base!(
    half::f16,
    half::f16::ZERO,
    half::f16::ONE,
    crate::F16Vec2,
    crate::F16Vec3,
    crate::F16Vec4,
    crate::F16Mat2,
    crate::F16Mat2x3,
    crate::F16Mat2x4,
    crate::F16Mat3x2,
    crate::F16Mat3,
    crate::F16Mat3x4,
    crate::F16Mat4x2,
    crate::F16Mat4x3,
    crate::F16Mat4
);

#[inline]
fn resize_cast_mat<Src, Dst>(src: Src) -> Dst
where
    Src: MatGrid,
    Dst: MatGrid,
    Src::S: ScalarConv<Dst::S>,
{
    Dst::from_fn(|c, r| {
        if c < Src::COLS && r < Src::ROWS {
            src.cell(c, r).sconv()
        } else if c == r {
            Dst::ONE_S
        } else {
            Dst::ZERO_S
        }
    })
}

macro_rules! impl_mat_resize_group {
    ($head:ty, $($tail:ty),+ $(,)?) => {
        $(
            impl Construct<$tail> for $head {
                #[inline]
                fn construct(self) -> $tail {
                    resize_cast_mat(self)
                }
            }
            impl Construct<$head> for $tail {
                #[inline]
                fn construct(self) -> $head {
                    resize_cast_mat(self)
                }
            }
        )+
        impl_mat_resize_group!($($tail),+);
    };
    ($last:ty) => {};
}

impl_mat_resize_group!(
    crate::Mat2,
    crate::Mat2x3,
    crate::Mat2x3A,
    crate::Mat2x4,
    crate::Mat3x2,
    crate::Mat3,
    crate::Mat3A,
    crate::Mat3x4,
    crate::Mat4x2,
    crate::Mat4x3,
    crate::Mat4x3A,
    crate::Mat4,
);

#[cfg(feature = "f64")]
impl_mat_resize_group!(
    crate::DMat2,
    crate::DMat2x3,
    crate::DMat2x4,
    crate::DMat3x2,
    crate::DMat3,
    crate::DMat3x4,
    crate::DMat4x2,
    crate::DMat4x3,
    crate::DMat4,
);

#[cfg(feature = "f16")]
impl_mat_resize_group!(
    crate::F16Mat2,
    crate::F16Mat2x3,
    crate::F16Mat2x4,
    crate::F16Mat3x2,
    crate::F16Mat3,
    crate::F16Mat3x4,
    crate::F16Mat4x2,
    crate::F16Mat4x3,
    crate::F16Mat4,
);

macro_rules! impl_mat_cross_family {
    ([$($a:ty),+], $b_group:tt) => {
        $(
            impl_mat_cross_family!(@single $a => $b_group);
        )+
    };
    (@single $a:ty => [$($b:ty),+]) => {
        $(
            impl Construct<$b> for $a {
                #[inline]
                fn construct(self) -> $b {
                    resize_cast_mat(self)
                }
            }
            impl Construct<$a> for $b {
                #[inline]
                fn construct(self) -> $a {
                    resize_cast_mat(self)
                }
            }
        )+
    };
}

#[cfg(feature = "f64")]
impl_mat_cross_family!(
    [
        crate::Mat2,
        crate::Mat2x3,
        crate::Mat2x3A,
        crate::Mat2x4,
        crate::Mat3x2,
        crate::Mat3,
        crate::Mat3A,
        crate::Mat3x4,
        crate::Mat4x2,
        crate::Mat4x3,
        crate::Mat4x3A,
        crate::Mat4
    ],
    [
        crate::DMat2,
        crate::DMat2x3,
        crate::DMat2x4,
        crate::DMat3x2,
        crate::DMat3,
        crate::DMat3x4,
        crate::DMat4x2,
        crate::DMat4x3,
        crate::DMat4
    ]
);

#[cfg(feature = "f16")]
impl_mat_cross_family!(
    [
        crate::Mat2,
        crate::Mat2x3,
        crate::Mat2x3A,
        crate::Mat2x4,
        crate::Mat3x2,
        crate::Mat3,
        crate::Mat3A,
        crate::Mat3x4,
        crate::Mat4x2,
        crate::Mat4x3,
        crate::Mat4x3A,
        crate::Mat4
    ],
    [
        crate::F16Mat2,
        crate::F16Mat2x3,
        crate::F16Mat2x4,
        crate::F16Mat3x2,
        crate::F16Mat3,
        crate::F16Mat3x4,
        crate::F16Mat4x2,
        crate::F16Mat4x3,
        crate::F16Mat4
    ]
);

#[cfg(all(feature = "f64", feature = "f16"))]
impl_mat_cross_family!(
    [
        crate::DMat2,
        crate::DMat2x3,
        crate::DMat2x4,
        crate::DMat3x2,
        crate::DMat3,
        crate::DMat3x4,
        crate::DMat4x2,
        crate::DMat4x3,
        crate::DMat4
    ],
    [
        crate::F16Mat2,
        crate::F16Mat2x3,
        crate::F16Mat2x4,
        crate::F16Mat3x2,
        crate::F16Mat3,
        crate::F16Mat3x4,
        crate::F16Mat4x2,
        crate::F16Mat4x3,
        crate::F16Mat4
    ]
);

#[path = "construct_macros.rs"]
mod generated_ctors;
pub use generated_ctors::*;

impl From<crate::Mat2x3A> for crate::Mat2x3 {
    #[inline(always)]
    fn from(m: crate::Mat2x3A) -> Self {
        Self::from_cols(crate::Vec3::from(m.x_axis), crate::Vec3::from(m.y_axis))
    }
}

impl From<crate::Mat2x3> for crate::Mat2x3A {
    #[inline(always)]
    fn from(m: crate::Mat2x3) -> Self {
        Self::from_cols(crate::Vec3A::from(m.x_axis), crate::Vec3A::from(m.y_axis))
    }
}

impl From<crate::Mat3A> for crate::Mat3 {
    #[inline(always)]
    fn from(m: crate::Mat3A) -> Self {
        Self::from_cols(
            crate::Vec3::from(m.x_axis),
            crate::Vec3::from(m.y_axis),
            crate::Vec3::from(m.z_axis),
        )
    }
}

impl From<crate::Mat3> for crate::Mat3A {
    #[inline(always)]
    fn from(m: crate::Mat3) -> Self {
        Self::from_cols(
            crate::Vec3A::from(m.x_axis),
            crate::Vec3A::from(m.y_axis),
            crate::Vec3A::from(m.z_axis),
        )
    }
}

impl From<crate::Mat4x3A> for crate::Mat4x3 {
    #[inline(always)]
    fn from(m: crate::Mat4x3A) -> Self {
        Self::from_cols(
            crate::Vec3::from(m.x_axis),
            crate::Vec3::from(m.y_axis),
            crate::Vec3::from(m.z_axis),
            crate::Vec3::from(m.w_axis),
        )
    }
}

impl From<crate::Mat4x3> for crate::Mat4x3A {
    #[inline(always)]
    fn from(m: crate::Mat4x3) -> Self {
        Self::from_cols(
            crate::Vec3A::from(m.x_axis),
            crate::Vec3A::from(m.y_axis),
            crate::Vec3A::from(m.z_axis),
            crate::Vec3A::from(m.w_axis),
        )
    }
}

/// Constructs a single-precision quaternion from scalar `(w, x, y, z)` components (C++ GLM scalar order).
#[inline(always)]
#[must_use]
pub const fn quat_wxyz(w: f32, x: f32, y: f32, z: f32) -> crate::Quat {
    crate::Quat::from_wxyz(w, x, y, z)
}

/// Constructs a single-precision quaternion from `(x, y, z, w)` components matching `[x, y, z, w]` memory order.
#[inline(always)]
#[must_use]
pub const fn quat_xyzw(x: f32, y: f32, z: f32, w: f32) -> crate::Quat {
    crate::Quat::from_xyzw(x, y, z, w)
}

/// Returns the multiplicative identity single-precision quaternion `(x=0, y=0, z=0, w=1)`.
#[inline(always)]
#[must_use]
pub const fn quat_identity() -> crate::Quat {
    crate::Quat::IDENTITY
}

/// Constructs a double-precision quaternion from scalar `(w, x, y, z)` components (C++ GLM scalar order).
#[cfg(feature = "f64")]
#[inline(always)]
#[must_use]
pub const fn dquat_wxyz(w: f64, x: f64, y: f64, z: f64) -> crate::DQuat {
    crate::DQuat::from_wxyz(w, x, y, z)
}

/// Constructs a double-precision quaternion from `(x, y, z, w)` components matching `[x, y, z, w]` memory order.
#[cfg(feature = "f64")]
#[inline(always)]
#[must_use]
pub const fn dquat_xyzw(x: f64, y: f64, z: f64, w: f64) -> crate::DQuat {
    crate::DQuat::from_xyzw(x, y, z, w)
}

/// Returns the multiplicative identity double-precision quaternion `(x=0, y=0, z=0, w=1)`.
#[cfg(feature = "f64")]
#[inline(always)]
#[must_use]
pub const fn dquat_identity() -> crate::DQuat {
    crate::DQuat::IDENTITY
}

/// Constructs a half-precision quaternion from scalar `(w, x, y, z)` components (C++ GLM scalar order).
#[cfg(feature = "f16")]
#[inline(always)]
#[must_use]
pub const fn f16quat_wxyz(
    w: half::f16,
    x: half::f16,
    y: half::f16,
    z: half::f16,
) -> crate::F16Quat {
    crate::F16Quat::from_wxyz(w, x, y, z)
}

/// Constructs a half-precision quaternion from `(x, y, z, w)` components matching `[x, y, z, w]` memory order.
#[cfg(feature = "f16")]
#[inline(always)]
#[must_use]
pub const fn f16quat_xyzw(
    x: half::f16,
    y: half::f16,
    z: half::f16,
    w: half::f16,
) -> crate::F16Quat {
    crate::F16Quat::from_xyzw(x, y, z, w)
}

/// Returns the multiplicative identity half-precision quaternion `(x=0, y=0, z=0, w=1)`.
#[cfg(feature = "f16")]
#[inline(always)]
#[must_use]
pub const fn f16quat_identity() -> crate::F16Quat {
    crate::F16Quat::IDENTITY
}

pub trait AngleAxisOp<Axis, Out> {
    fn angle_axis_op(self, axis: Axis) -> Out;
}

impl AngleAxisOp<crate::Vec3, crate::Quat> for f32 {
    #[inline]
    fn angle_axis_op(self, axis: crate::Vec3) -> crate::Quat {
        crate::Quat::from_axis_angle(axis, self)
    }
}

impl AngleAxisOp<crate::Vec3A, crate::Quat> for f32 {
    #[inline]
    fn angle_axis_op(self, axis: crate::Vec3A) -> crate::Quat {
        crate::Quat::from_axis_angle(crate::Vec3::from(axis), self)
    }
}

#[cfg(feature = "f64")]
impl AngleAxisOp<crate::DVec3, crate::DQuat> for f64 {
    #[inline]
    fn angle_axis_op(self, axis: crate::DVec3) -> crate::DQuat {
        crate::DQuat::from_axis_angle(axis, self)
    }
}

#[cfg(feature = "f16")]
impl AngleAxisOp<crate::F16Vec3, crate::F16Quat> for half::f16 {
    #[inline]
    fn angle_axis_op(self, axis: crate::F16Vec3) -> crate::F16Quat {
        crate::F16Quat::from_axis_angle(axis, self)
    }
}

/// Builds a rotation quaternion from a rotation `angle` in radians around a unit `axis` (`glm::angleAxis`).
#[inline]
#[must_use]
pub fn angle_axis<S, V, Q>(angle: S, axis: V) -> Q
where
    S: AngleAxisOp<V, Q>,
{
    angle.angle_axis_op(axis)
}

pub trait QuatCastOp {
    type Out;
    fn quat_cast_op(self) -> Self::Out;
}

impl QuatCastOp for crate::Mat3 {
    type Out = crate::Quat;
    #[inline]
    fn quat_cast_op(self) -> crate::Quat {
        crate::Quat::from_mat3(self)
    }
}

impl QuatCastOp for crate::Mat3A {
    type Out = crate::Quat;
    #[inline]
    fn quat_cast_op(self) -> crate::Quat {
        crate::Quat::from_mat3(crate::Mat3::from(self))
    }
}

impl QuatCastOp for crate::Mat4 {
    type Out = crate::Quat;
    #[inline]
    fn quat_cast_op(self) -> crate::Quat {
        crate::Quat::from_mat3(mat3(self))
    }
}

#[cfg(feature = "f64")]
impl QuatCastOp for crate::DMat3 {
    type Out = crate::DQuat;
    #[inline]
    fn quat_cast_op(self) -> crate::DQuat {
        crate::DQuat::from_mat3(self)
    }
}

#[cfg(feature = "f64")]
impl QuatCastOp for crate::DMat4 {
    type Out = crate::DQuat;
    #[inline]
    fn quat_cast_op(self) -> crate::DQuat {
        crate::DQuat::from_mat3(dmat3(self))
    }
}

#[cfg(feature = "f16")]
impl QuatCastOp for crate::F16Mat3 {
    type Out = crate::F16Quat;
    #[inline]
    fn quat_cast_op(self) -> crate::F16Quat {
        crate::F16Quat::from_mat3(self)
    }
}

#[cfg(feature = "f16")]
impl QuatCastOp for crate::F16Mat4 {
    type Out = crate::F16Quat;
    #[inline]
    fn quat_cast_op(self) -> crate::F16Quat {
        crate::F16Quat::from_mat3(f16mat3(self))
    }
}

/// Converts a 3x3 or 4x4 orthonormal rotation matrix into a unit quaternion (`glm::quat_cast`).
#[inline]
#[must_use]
pub fn quat_cast<M: QuatCastOp>(m: M) -> M::Out {
    m.quat_cast_op()
}

pub trait MatCastQuatOp {
    type Mat3Out;
    type Mat4Out;
    fn mat3_cast_op(self) -> Self::Mat3Out;
    fn mat4_cast_op(self) -> Self::Mat4Out;
}

impl MatCastQuatOp for crate::Quat {
    type Mat3Out = crate::Mat3;
    type Mat4Out = crate::Mat4;
    #[inline]
    fn mat3_cast_op(self) -> crate::Mat3 {
        self.to_mat3()
    }
    #[inline]
    fn mat4_cast_op(self) -> crate::Mat4 {
        self.to_mat4()
    }
}

#[cfg(feature = "f64")]
impl MatCastQuatOp for crate::DQuat {
    type Mat3Out = crate::DMat3;
    type Mat4Out = crate::DMat4;
    #[inline]
    fn mat3_cast_op(self) -> crate::DMat3 {
        self.to_mat3()
    }
    #[inline]
    fn mat4_cast_op(self) -> crate::DMat4 {
        self.to_mat4()
    }
}

#[cfg(feature = "f16")]
impl MatCastQuatOp for crate::F16Quat {
    type Mat3Out = crate::F16Mat3;
    type Mat4Out = crate::F16Mat4;
    #[inline]
    fn mat3_cast_op(self) -> crate::F16Mat3 {
        self.to_mat3()
    }
    #[inline]
    fn mat4_cast_op(self) -> crate::F16Mat4 {
        self.to_mat4()
    }
}

/// Converts a unit quaternion into a 3x3 rotation matrix (`glm::mat3_cast`).
#[inline]
#[must_use]
pub fn mat3_cast<Q: MatCastQuatOp>(q: Q) -> Q::Mat3Out {
    q.mat3_cast_op()
}

/// Converts a unit quaternion into a 4x4 rotation matrix (`glm::mat4_cast`).
#[inline]
#[must_use]
pub fn mat4_cast<Q: MatCastQuatOp>(q: Q) -> Q::Mat4Out {
    q.mat4_cast_op()
}

pub trait FlatArray: Copy {
    type Scalar: Copy;
    const LEN: usize;
    type Array: Copy;
    fn into_flat_array(self) -> Self::Array;
    fn from_flat_slice(slice: &[Self::Scalar]) -> Self;
    fn write_flat_slice(self, slice: &mut [Self::Scalar]);
}

macro_rules! impl_flat_vec_quat {
    ($($ty:ty => [$s:ty; $n:literal]),+ $(,)?) => {
        $(
            impl FlatArray for $ty {
                type Scalar = $s;
                const LEN: usize = $n;
                type Array = [$s; $n];

                #[inline(always)]
                fn into_flat_array(self) -> [$s; $n] {
                    self.to_array()
                }

                #[inline]
                fn from_flat_slice(slice: &[$s]) -> Self {
                    let mut arr = [slice[0]; $n];
                    arr.copy_from_slice(&slice[..$n]);
                    Self::from_array(arr)
                }

                #[inline]
                fn write_flat_slice(self, slice: &mut [$s]) {
                    slice[..$n].copy_from_slice(&self.to_array());
                }
            }
        )+
    };
}

macro_rules! impl_flat_mat {
    ($($ty:ty => [$s:ty; $n:literal; $c:literal x $r:literal]),+ $(,)?) => {
        $(
            impl FlatArray for $ty {
                type Scalar = $s;
                const LEN: usize = $n;
                type Array = [$s; $n];

                #[inline(always)]
                fn into_flat_array(self) -> [$s; $n] {
                    self.to_cols_array()
                }

                #[inline]
                fn from_flat_slice(slice: &[$s]) -> Self {
                    let mut arr = [slice[0]; $n];
                    arr.copy_from_slice(&slice[..$n]);
                    Self::from_cols_array(arr)
                }

                #[inline]
                fn write_flat_slice(self, slice: &mut [$s]) {
                    slice[..$n].copy_from_slice(&self.to_cols_array());
                }
            }
        )+
    };
}

impl_flat_vec_quat!(
    crate::Vec2 => [f32; 2],
    crate::Vec3 => [f32; 3],
    crate::Vec3A => [f32; 3],
    crate::Vec4 => [f32; 4],
    crate::BVec2 => [bool; 2],
    crate::BVec3 => [bool; 3],
    crate::BVec3A => [bool; 3],
    crate::BVec4 => [bool; 4],
    crate::BVec4A => [bool; 4],
    crate::Quat => [f32; 4],
);

impl_flat_mat!(
    crate::Mat2 => [f32; 4; 2 x 2],
    crate::Mat2x3 => [f32; 6; 2 x 3],
    crate::Mat2x3A => [f32; 6; 2 x 3],
    crate::Mat2x4 => [f32; 8; 2 x 4],
    crate::Mat3x2 => [f32; 6; 3 x 2],
    crate::Mat3 => [f32; 9; 3 x 3],
    crate::Mat3A => [f32; 9; 3 x 3],
    crate::Mat3x4 => [f32; 12; 3 x 4],
    crate::Mat4x2 => [f32; 8; 4 x 2],
    crate::Mat4x3 => [f32; 12; 4 x 3],
    crate::Mat4x3A => [f32; 12; 4 x 3],
    crate::Mat4 => [f32; 16; 4 x 4],
);

#[cfg(feature = "f64")]
impl_flat_vec_quat!(
    crate::DVec2 => [f64; 2],
    crate::DVec3 => [f64; 3],
    crate::DVec4 => [f64; 4],
    crate::DQuat => [f64; 4],
);

#[cfg(feature = "f64")]
impl_flat_mat!(
    crate::DMat2 => [f64; 4; 2 x 2],
    crate::DMat2x3 => [f64; 6; 2 x 3],
    crate::DMat2x4 => [f64; 8; 2 x 4],
    crate::DMat3x2 => [f64; 6; 3 x 2],
    crate::DMat3 => [f64; 9; 3 x 3],
    crate::DMat3x4 => [f64; 12; 3 x 4],
    crate::DMat4x2 => [f64; 8; 4 x 2],
    crate::DMat4x3 => [f64; 12; 4 x 3],
    crate::DMat4 => [f64; 16; 4 x 4],
);

#[cfg(feature = "f16")]
impl_flat_vec_quat!(
    crate::F16Vec2 => [half::f16; 2],
    crate::F16Vec3 => [half::f16; 3],
    crate::F16Vec4 => [half::f16; 4],
    crate::F16Quat => [half::f16; 4],
);

#[cfg(feature = "f16")]
impl_flat_mat!(
    crate::F16Mat2 => [half::f16; 4; 2 x 2],
    crate::F16Mat2x3 => [half::f16; 6; 2 x 3],
    crate::F16Mat2x4 => [half::f16; 8; 2 x 4],
    crate::F16Mat3x2 => [half::f16; 6; 3 x 2],
    crate::F16Mat3 => [half::f16; 9; 3 x 3],
    crate::F16Mat3x4 => [half::f16; 12; 3 x 4],
    crate::F16Mat4x2 => [half::f16; 8; 4 x 2],
    crate::F16Mat4x3 => [half::f16; 12; 4 x 3],
    crate::F16Mat4 => [half::f16; 16; 4 x 4],
);

#[cfg(feature = "i8")]
impl_flat_vec_quat!(crate::I8Vec2 => [i8; 2], crate::I8Vec3 => [i8; 3], crate::I8Vec4 => [i8; 4]);
#[cfg(feature = "u8")]
impl_flat_vec_quat!(crate::U8Vec2 => [u8; 2], crate::U8Vec3 => [u8; 3], crate::U8Vec4 => [u8; 4]);
#[cfg(feature = "i16")]
impl_flat_vec_quat!(crate::I16Vec2 => [i16; 2], crate::I16Vec3 => [i16; 3], crate::I16Vec4 => [i16; 4]);
#[cfg(feature = "u16")]
impl_flat_vec_quat!(crate::U16Vec2 => [u16; 2], crate::U16Vec3 => [u16; 3], crate::U16Vec4 => [u16; 4]);
#[cfg(feature = "i32")]
impl_flat_vec_quat!(crate::IVec2 => [i32; 2], crate::IVec3 => [i32; 3], crate::IVec4 => [i32; 4]);
#[cfg(feature = "u32")]
impl_flat_vec_quat!(crate::UVec2 => [u32; 2], crate::UVec3 => [u32; 3], crate::UVec4 => [u32; 4]);
#[cfg(feature = "i64")]
impl_flat_vec_quat!(crate::I64Vec2 => [i64; 2], crate::I64Vec3 => [i64; 3], crate::I64Vec4 => [i64; 4]);
#[cfg(feature = "u64")]
impl_flat_vec_quat!(crate::U64Vec2 => [u64; 2], crate::U64Vec3 => [u64; 3], crate::U64Vec4 => [u64; 4]);

/// Converts a vector, quaternion, or matrix into its flat column-major scalar array.
#[inline(always)]
#[must_use]
pub fn to_array<T: FlatArray>(val: T) -> T::Array {
    val.into_flat_array()
}

/// Reads a vector, quaternion, or column-major matrix from the leading elements of `slice`.
#[inline]
#[must_use]
pub fn from_slice<T: FlatArray>(slice: &[T::Scalar]) -> T {
    T::from_flat_slice(slice)
}

/// Writes the flat scalar components of `val` into the leading elements of `slice`.
#[inline]
pub fn write_to_slice<T: FlatArray>(val: T, slice: &mut [T::Scalar]) {
    val.write_flat_slice(slice);
}
