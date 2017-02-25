use core::{f32, f64};

use trig_f32::*;
use trig_f64::*;


pub static TO_DEGREES_F32: f32 = 180_f32 / f32::consts::PI;
pub static TO_DEGREES_F64: f64 = 180_f64 / f64::consts::PI;

pub static TO_RADS_F32: f32 = f32::consts::PI / 180_f32;
pub static TO_RADS_F64: f64 = f64::consts::PI / 180_f64;


pub trait Trig {
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::sin(&1_f32), 0.84147096_f32);
    /// assert_eq!(Trig::sin(&1_f64), 0.8414709848078965_f64);
    /// ~~~
    fn sin(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::asin(&1_f32), 1.5707964_f32);
    /// assert_eq!(Trig::asin(&1_f64), 1.5707963267948966_f64);
    /// ~~~
    fn asin(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::sinh(&1_f32), 1.1752012_f32);
    /// assert_eq!(Trig::sinh(&1_f64), 1.1752011936438014_f64);
    /// ~~~
    fn sinh(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::asinh(&1_f32), 0.88137364_f32);
    /// assert_eq!(Trig::asinh(&1_f64), 0.8813735870195429_f64);
    /// ~~~
    fn asinh(&self) -> Self;

    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::cos(&1_f32), 0.5403023_f32);
    /// assert_eq!(Trig::cos(&1_f64), 0.5403023058681398_f64);
    /// ~~~
    fn cos(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::acos(&1_f32), 0_f32);
    /// assert_eq!(Trig::acos(&1_f64), 0_f64);
    /// ~~~
    fn acos(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::cosh(&1_f32), 1.5430806_f32);
    /// assert_eq!(Trig::cosh(&1_f64), 1.5430806348152437_f64);
    /// ~~~
    fn cosh(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::acosh(&1_f32), 0_f32);
    /// assert_eq!(Trig::acosh(&1_f64), 0_f64);
    /// ~~~
    fn acosh(&self) -> Self;

    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::tan(&1_f32), 1.5574077_f32);
    /// assert_eq!(Trig::tan(&1_f64), 1.5574077246549023_f64);
    /// ~~~
    fn tan(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::atan(&1_f32), 0.7853982_f32);
    /// assert_eq!(Trig::atan(&1_f64), 0.7853981633974483_f64);
    /// ~~~
    fn atan(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::tanh(&1_f32), 0.7615942_f32);
    /// assert_eq!(Trig::tanh(&1_f64), 0.7615941559557649_f64);
    /// ~~~
    fn tanh(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::atanh(&0_f32), 0_f32);
    /// assert_eq!(Trig::atanh(&0_f64), 0_f64);
    /// ~~~
    fn atanh(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// assert_eq!(Trig::atan2(&1_f32, &1_f32), 0.7853982_f32);
    /// assert_eq!(Trig::atan2(&1_f64, &1_f64), 0.7853981633974483_f64);
    /// ~~~
    fn atan2(&self, other: &Self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// use std::f32::consts::PI;
    /// assert_eq!(Trig::to_degrees(&PI), 180_f32);
    /// ~~~
    fn to_degrees(&self) -> Self;
    /// # Examples
    /// ~~~
    /// use trig::Trig;
    /// use std::f32::consts::PI;
    /// assert_eq!(Trig::to_radians(&180f32), PI);
    /// ~~~
    fn to_radians(&self) -> Self;
}

macro_rules! trait_trig_as_32 {
    ($t:ident) => (
        impl Trig for $t {
            #[inline(always)]
            fn sin(&self) -> Self { unsafe { sinf32(*self as f32) as $t } }
            #[inline(always)]
            fn asin(&self) -> Self { unsafe { asinf32(*self as f32) as $t } }
            #[inline(always)]
            fn sinh(&self) -> Self { unsafe { sinhf32(*self as f32) as $t } }
            #[inline(always)]
            fn asinh(&self) -> Self { unsafe { asinhf32(*self as f32) as $t } }

            #[inline(always)]
            fn cos(&self) -> Self { unsafe { cosf32(*self as f32) as $t } }
            #[inline(always)]
            fn acos(&self) -> Self { unsafe { acosf32(*self as f32) as $t } }
            #[inline(always)]
            fn cosh(&self) -> Self { unsafe { coshf32(*self as f32) as $t } }
            #[inline(always)]
            fn acosh(&self) -> Self { unsafe { acoshf32(*self as f32) as $t } }

            #[inline(always)]
            fn tan(&self) -> Self { unsafe { tanf32(*self as f32) as $t } }
            #[inline(always)]
            fn atan(&self) -> Self { unsafe { atanf32(*self as f32) as $t } }
            #[inline(always)]
            fn tanh(&self) -> Self { unsafe { tanhf32(*self as f32) as $t } }
            #[inline(always)]
            fn atanh(&self) -> Self { unsafe { atanhf32(*self as f32) as $t } }
            #[inline(always)]
            fn atan2(&self, other: &$t) -> Self {
                unsafe { atan2f32(*self as f32, *other as f32) as $t }
            }
            #[inline]
            fn to_degrees(&self) -> Self {
                *self * TO_DEGREES_F32 as $t
            }
            #[inline]
            fn to_radians(&self) -> Self {
                ((*self as f32) * TO_RADS_F32) as $t
            }
        }
    );
}

macro_rules! trait_trig_as_64 {
    ($t:ident) => (
        impl Trig for $t {
            #[inline(always)]
            fn sin(&self) -> Self { unsafe { sinf64(*self as f64) as $t } }
            #[inline(always)]
            fn asin(&self) -> Self { unsafe { asinf64(*self as f64) as $t } }
            #[inline(always)]
            fn sinh(&self) -> Self { unsafe { sinhf64(*self as f64) as $t } }
            #[inline(always)]
            fn asinh(&self) -> Self { unsafe { asinhf64(*self as f64) as $t } }

            #[inline(always)]
            fn cos(&self) -> Self { unsafe { cosf64(*self as f64) as $t } }
            #[inline(always)]
            fn acos(&self) -> Self { unsafe { acosf64(*self as f64) as $t } }
            #[inline(always)]
            fn cosh(&self) -> Self { unsafe { coshf64(*self as f64) as $t } }
            #[inline(always)]
            fn acosh(&self) -> Self { unsafe { acoshf64(*self as f64) as $t } }

            #[inline(always)]
            fn tan(&self) -> Self { unsafe { tanf64(*self as f64) as $t } }
            #[inline(always)]
            fn atan(&self) -> Self { unsafe { atanf64(*self as f64) as $t } }
            #[inline(always)]
            fn tanh(&self) -> Self { unsafe { tanhf64(*self as f64) as $t } }
            #[inline(always)]
            fn atanh(&self) -> Self { unsafe { atanhf64(*self as f64) as $t } }
            #[inline(always)]
            fn atan2(&self, other: &$t) -> Self {
                unsafe { atan2f64(*self as f64, *other as f64) as $t }
            }
            #[inline]
            fn to_degrees(&self) -> Self {
                *self * TO_DEGREES_F64 as $t
            }
            #[inline]
            fn to_radians(&self) -> Self {
                ((*self as f64) * TO_RADS_F64) as $t
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
    fn sin(&self) -> Self { unsafe { sinf32(*self) } }
    #[inline(always)]
    fn asin(&self) -> Self { unsafe { asinf32(*self) } }
    #[inline(always)]
    fn sinh(&self) -> Self { unsafe { sinhf32(*self) } }
    #[inline(always)]
    fn asinh(&self) -> Self { unsafe { asinhf32(*self) } }

    #[inline(always)]
    fn cos(&self) -> Self { unsafe { cosf32(*self) } }
    #[inline(always)]
    fn acos(&self) -> Self { unsafe { acosf32(*self) } }
    #[inline(always)]
    fn cosh(&self) -> Self { unsafe { coshf32(*self) } }
    #[inline(always)]
    fn acosh(&self) -> Self { unsafe { acoshf32(*self) } }

    #[inline(always)]
    fn tan(&self) -> Self { unsafe { tanf32(*self) } }
    #[inline(always)]
    fn atan(&self) -> Self { unsafe { atanf32(*self) } }
    #[inline(always)]
    fn tanh(&self) -> Self { unsafe { tanhf32(*self) } }
    #[inline(always)]
    fn atanh(&self) -> Self { unsafe { atanhf32(*self) } }
    #[inline(always)]
    fn atan2(&self, other: &f32) -> Self { unsafe { atan2f32(*self, *other) } }
    #[inline]
    fn to_degrees(&self) -> Self {
        *self * TO_DEGREES_F32
    }
    #[inline]
    fn to_radians(&self) -> Self {
        *self * TO_RADS_F32
    }
}

impl Trig for f64 {
    #[inline(always)]
    fn sin(&self) -> Self { unsafe { sinf64(*self) } }
    #[inline(always)]
    fn asin(&self) -> Self { unsafe { asinf64(*self) } }
    #[inline(always)]
    fn sinh(&self) -> Self { unsafe { sinhf64(*self) } }
    #[inline(always)]
    fn asinh(&self) -> Self { unsafe { asinhf64(*self) } }

    #[inline(always)]
    fn cos(&self) -> Self { unsafe { cosf64(*self) } }
    #[inline(always)]
    fn acos(&self) -> Self { unsafe { acosf64(*self) } }
    #[inline(always)]
    fn cosh(&self) -> Self { unsafe { coshf64(*self) } }
    #[inline(always)]
    fn acosh(&self) -> Self { unsafe { acoshf64(*self) } }

    #[inline(always)]
    fn tan(&self) -> Self { unsafe { tanf64(*self) } }
    #[inline(always)]
    fn atan(&self) -> Self { unsafe { atanf64(*self) } }
    #[inline(always)]
    fn tanh(&self) -> Self { unsafe { tanhf64(*self) } }
    #[inline(always)]
    fn atanh(&self) -> Self { unsafe { atanhf64(*self) } }
    #[inline(always)]
    fn atan2(&self, other: &f64) -> Self { unsafe { atan2f64(*self, *other) } }
    #[inline]
    fn to_degrees(&self) -> Self {
        *self * TO_DEGREES_F64
    }
    #[inline]
    fn to_radians(&self) -> Self {
        *self * TO_RADS_F64
    }
}
