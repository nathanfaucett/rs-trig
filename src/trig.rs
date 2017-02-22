use trig_f32::*;
use trig_f64::*;


pub trait Trig {
    fn sin(self) -> Self;
    fn asin(self) -> Self;
    fn sinh(self) -> Self;
    fn asinh(self) -> Self;

    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn cosh(self) -> Self;
    fn acosh(self) -> Self;

    fn tan(self) -> Self;
    fn atan(self) -> Self;
    fn tanh(self) -> Self;
    fn atanh(self) -> Self;
    fn atan2(self, other: Self) -> Self;
}

macro_rules! trait_trig_as_32 {
    ($t:ident) => (
        impl Trig for $t {
            #[inline(always)]
            fn sin(self) -> Self { unsafe { sinf32(self as f32) as $t } }
            #[inline(always)]
            fn asin(self) -> Self { unsafe { asinf32(self as f32) as $t } }
            #[inline(always)]
            fn sinh(self) -> Self { unsafe { sinhf32(self as f32) as $t } }
            #[inline(always)]
            fn asinh(self) -> Self { unsafe { asinhf32(self as f32) as $t } }

            #[inline(always)]
            fn cos(self) -> Self { unsafe { cosf32(self as f32) as $t } }
            #[inline(always)]
            fn acos(self) -> Self { unsafe { acosf32(self as f32) as $t } }
            #[inline(always)]
            fn cosh(self) -> Self { unsafe { coshf32(self as f32) as $t } }
            #[inline(always)]
            fn acosh(self) -> Self { unsafe { acoshf32(self as f32) as $t } }

            #[inline(always)]
            fn tan(self) -> Self { unsafe { tanf32(self as f32) as $t } }
            #[inline(always)]
            fn atan(self) -> Self { unsafe { atanf32(self as f32) as $t } }
            #[inline(always)]
            fn tanh(self) -> Self { unsafe { tanhf32(self as f32) as $t } }
            #[inline(always)]
            fn atanh(self) -> Self { unsafe { atanhf32(self as f32) as $t } }
            #[inline(always)]
            fn atan2(self, other: $t) -> Self {
                unsafe { atan2f32(self as f32, other as f32) as $t }
            }
        }
    );
}

macro_rules! trait_trig_as_64 {
    ($t:ident) => (
        impl Trig for $t {
            #[inline(always)]
            fn sin(self) -> Self { unsafe { sinf64(self as f64) as $t } }
            #[inline(always)]
            fn asin(self) -> Self { unsafe { asinf64(self as f64) as $t } }
            #[inline(always)]
            fn sinh(self) -> Self { unsafe { sinhf64(self as f64) as $t } }
            #[inline(always)]
            fn asinh(self) -> Self { unsafe { asinhf64(self as f64) as $t } }

            #[inline(always)]
            fn cos(self) -> Self { unsafe { cosf64(self as f64) as $t } }
            #[inline(always)]
            fn acos(self) -> Self { unsafe { acosf64(self as f64) as $t } }
            #[inline(always)]
            fn cosh(self) -> Self { unsafe { coshf64(self as f64) as $t } }
            #[inline(always)]
            fn acosh(self) -> Self { unsafe { acoshf64(self as f64) as $t } }

            #[inline(always)]
            fn tan(self) -> Self { unsafe { tanf64(self as f64) as $t } }
            #[inline(always)]
            fn atan(self) -> Self { unsafe { atanf64(self as f64) as $t } }
            #[inline(always)]
            fn tanh(self) -> Self { unsafe { tanhf64(self as f64) as $t } }
            #[inline(always)]
            fn atanh(self) -> Self { unsafe { atanhf64(self as f64) as $t } }
            #[inline(always)]
            fn atan2(self, other: $t) -> Self {
                unsafe { atan2f64(self as f64, other as f64) as $t }
            }
        }
    );
}


trait_trig_as_64!(usize);
trait_trig_as_32!(u8);
trait_trig_as_32!(u16);
trait_trig_as_64!(u32);
trait_trig_as_64!(u64);

trait_trig_as_64!(isize);
trait_trig_as_32!(i8);
trait_trig_as_32!(i16);
trait_trig_as_64!(i32);
trait_trig_as_64!(i64);


impl Trig for f32 {
    #[inline(always)]
    fn sin(self) -> Self { unsafe { sinf32(self) } }
    #[inline(always)]
    fn asin(self) -> Self { unsafe { asinf32(self) } }
    #[inline(always)]
    fn sinh(self) -> Self { unsafe { sinhf32(self) } }
    #[inline(always)]
    fn asinh(self) -> Self { unsafe { asinhf32(self) } }

    #[inline(always)]
    fn cos(self) -> Self { unsafe { cosf32(self) } }
    #[inline(always)]
    fn acos(self) -> Self { unsafe { acosf32(self) } }
    #[inline(always)]
    fn cosh(self) -> Self { unsafe { coshf32(self) } }
    #[inline(always)]
    fn acosh(self) -> Self { unsafe { acoshf32(self) } }

    #[inline(always)]
    fn tan(self) -> Self { unsafe { tanf32(self) } }
    #[inline(always)]
    fn atan(self) -> Self { unsafe { atanf32(self) } }
    #[inline(always)]
    fn tanh(self) -> Self { unsafe { tanhf32(self) } }
    #[inline(always)]
    fn atanh(self) -> Self { unsafe { atanhf32(self) } }
    #[inline(always)]
    fn atan2(self, other: f32) -> Self { unsafe { atan2f32(self, other) } }
}

impl Trig for f64 {
    #[inline(always)]
    fn sin(self) -> Self { unsafe { sinf64(self) } }
    #[inline(always)]
    fn asin(self) -> Self { unsafe { asinf64(self) } }
    #[inline(always)]
    fn sinh(self) -> Self { unsafe { sinhf64(self) } }
    #[inline(always)]
    fn asinh(self) -> Self { unsafe { asinhf64(self) } }

    #[inline(always)]
    fn cos(self) -> Self { unsafe { cosf64(self) } }
    #[inline(always)]
    fn acos(self) -> Self { unsafe { acosf64(self) } }
    #[inline(always)]
    fn cosh(self) -> Self { unsafe { coshf64(self) } }
    #[inline(always)]
    fn acosh(self) -> Self { unsafe { acoshf64(self) } }

    #[inline(always)]
    fn tan(self) -> Self { unsafe { tanf64(self) } }
    #[inline(always)]
    fn atan(self) -> Self { unsafe { atanf64(self) } }
    #[inline(always)]
    fn tanh(self) -> Self { unsafe { tanhf64(self) } }
    #[inline(always)]
    fn atanh(self) -> Self { unsafe { atanhf64(self) } }
    #[inline(always)]
    fn atan2(self, other: f64) -> Self { unsafe { atan2f64(self, other) } }
}

#[test]
fn trig() {
    assert_eq!((0).sin(), 0);
    assert_eq!((0).cos(), 1);
    assert_eq!((0).tan(), 0);

    assert_eq!((0f32).sin(), 0f32);
    assert_eq!((0f32).cos(), 1f32);
    assert_eq!((0f32).tan(), 0f32);

    assert_eq!((0f64).sin(), 0f64);
    assert_eq!((0f64).cos(), 1f64);
    assert_eq!((0f64).tan(), 0f64);
}
