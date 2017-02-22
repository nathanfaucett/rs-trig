use core::f64::{NEG_INFINITY, NAN};
use libc::c_double;


#[link_name = "m"]
extern {
    pub fn acos(n: c_double) -> c_double;
    pub fn asin(n: c_double) -> c_double;
    pub fn atan(n: c_double) -> c_double;
    pub fn atan2(a: c_double, b: c_double) -> c_double;
    pub fn cosh(n: c_double) -> c_double;
    pub fn sinh(n: c_double) -> c_double;
    pub fn tan(n: c_double) -> c_double;
    pub fn tanh(n: c_double) -> c_double;
}

extern "rust-intrinsic"  {
    pub fn cosf64(x: f64) -> f64;
    pub fn sinf64(x: f64) -> f64;
}


pub use self::acos as acosf64;
pub use self::asin as asinf64;
pub use self::atan as atanf64;
pub use self::atan2 as atan2f64;
pub use self::cosh as coshf64;
pub use self::sinh as sinhf64;
pub use self::tan as tanf64;
pub use self::tanh as tanhf64;


#[allow(illegal_floating_point_constant_pattern)]
#[inline]
pub unsafe fn asinhf64(n: f64) -> f64 {
    match n {
        NEG_INFINITY => NEG_INFINITY,
        x => (x + ((x * x) + 1f64).sqrt()).ln(),
    }
}
#[inline]
pub unsafe fn acoshf64(n: f64) -> f64 {
    match n {
        x if x < 1.0 => NAN,
        x => (x + ((x * x) - 1.0).sqrt()).ln(),
    }
}
#[inline]
pub unsafe fn atanhf64(n: f64) -> f64 {
    0.5 * ((2.0 * n) / (1.0 - n)).ln_1p()
}
