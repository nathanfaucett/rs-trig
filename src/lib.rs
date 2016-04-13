#![no_std]


extern crate libc;


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

macro_rules! trait_trig_as {
    ($t:ident, $a:ident) => (
        impl Trig for $t {
            #[inline(always)]
            fn sin(self) -> Self {(self as $a).sin() as $t}
            #[inline(always)]
            fn asin(self) -> Self {(self as $a).asin() as $t}
            #[inline(always)]
            fn sinh(self) -> Self {(self as $a).sinh() as $t}
            #[inline(always)]
            fn asinh(self) -> Self {(self as $a).asinh() as $t}

            #[inline(always)]
            fn cos(self) -> Self {(self as $a).cos() as $t}
            #[inline(always)]
            fn acos(self) -> Self {(self as $a).acos() as $t}
            #[inline(always)]
            fn cosh(self) -> Self {(self as $a).cosh() as $t}
            #[inline(always)]
            fn acosh(self) -> Self {(self as $a).acosh() as $t}

            #[inline(always)]
            fn tan(self) -> Self {(self as $a).tan() as $t}
            #[inline(always)]
            fn atan(self) -> Self {(self as $a).atan() as $t}
            #[inline(always)]
            fn tanh(self) -> Self {(self as $a).tanh() as $t}
            #[inline(always)]
            fn atanh(self) -> Self {(self as $a).atanh() as $t}
            #[inline(always)]
            fn atan2(self, other: $t) -> Self {(self as $a).atan2(other as $a) as $t}
        }
    );
}

macro_rules! trait_trig {
    ($t:ident) => (
        impl Trig for $t {
            #[inline(always)]
            fn sin(self) -> Self {self.sin()}
            #[inline(always)]
            fn asin(self) -> Self {self.asin()}
            #[inline(always)]
            fn sinh(self) -> Self {self.sinh()}
            #[inline(always)]
            fn asinh(self) -> Self {self.asinh()}

            #[inline(always)]
            fn cos(self) -> Self {self.cos()}
            #[inline(always)]
            fn acos(self) -> Self {self.acos()}
            #[inline(always)]
            fn cosh(self) -> Self {self.cosh()}
            #[inline(always)]
            fn acosh(self) -> Self {self.acosh()}

            #[inline(always)]
            fn tan(self) -> Self {self.tan()}
            #[inline(always)]
            fn atan(self) -> Self {self.atan()}
            #[inline(always)]
            fn tanh(self) -> Self {self.tanh()}
            #[inline(always)]
            fn atanh(self) -> Self {self.atanh()}
            #[inline(always)]
            fn atan2(self, other: $t) -> Self {self.atan2(other)}
        }
    );
}

trait_trig_as!(usize, f32);
trait_trig_as!(u8, f32);
trait_trig_as!(u16, f32);
trait_trig_as!(u32, f32);
trait_trig_as!(u64, f64);

trait_trig_as!(isize, f32);
trait_trig_as!(i8, f32);
trait_trig_as!(i16, f32);
trait_trig_as!(i32, f32);
trait_trig_as!(i64, f64);

trait_trig!(f32);
trait_trig!(f64);

#[test]
fn trig() {
    assert_eq!((0f32).sin(), 0f32);
    assert_eq!((0f32).cos(), 1f32);
    assert_eq!((0f32).tan(), 0f32);
}
