#[cfg(feature = "debug-assert")]
macro_rules! rglm_debug_assert {
    ($($arg:tt)*) => {
        debug_assert!($($arg)*);
    };
}

#[cfg(not(feature = "debug-assert"))]
macro_rules! rglm_debug_assert {
    ($($arg:tt)*) => {};
}

macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const _: () = assert!($x);
    };
}

macro_rules! const_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        const_assert!($x == $y);
    };
}
