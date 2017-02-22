use core::f32::{NEG_INFINITY, NAN};
use libc::c_float;


#[link_name = "m"]
extern {
    pub fn acos(n: c_float) -> c_float;
    pub fn asin(n: c_float) -> c_float;
    pub fn atan(n: c_float) -> c_float;
    pub fn atan2(a: c_float, b: c_float) -> c_float;
    pub fn cosh(n: c_float) -> c_float;
    pub fn sinh(n: c_float) -> c_float;
    pub fn tan(n: c_float) -> c_float;
    pub fn tanh(n: c_float) -> c_float;
}

extern "rust-intrinsic"  {
    pub fn cosf32(x: f32) -> f32;
    pub fn sinf32(x: f32) -> f32;
}


pub use self::acos as acosf32;
pub use self::asin as asinf32;
pub use self::atan as atanf32;
pub use self::atan2 as atan2f32;
pub use self::cosh as coshf32;
pub use self::sinh as sinhf32;
pub use self::tan as tanf32;
pub use self::tanh as tanhf32;


#[allow(illegal_floating_point_constant_pattern)]
#[inline]
pub unsafe fn asinhf32(n: f32) -> f32 {
    match n {
        NEG_INFINITY => NEG_INFINITY,
        x => (x + ((x * x) + 1f32).sqrt()).ln(),
    }
}
#[inline]
pub unsafe fn acoshf32(n: f32) -> f32 {
    match n {
        x if x < 1.0 => NAN,
        x => (x + ((x * x) - 1.0).sqrt()).ln(),
    }
}
#[inline]
pub unsafe fn atanhf32(n: f32) -> f32 {
    0.5 * ((2.0 * n) / (1.0 - n)).ln_1p()
}
