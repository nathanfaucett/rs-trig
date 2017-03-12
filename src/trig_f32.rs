use core::f32::{NEG_INFINITY, NAN};

use libc::c_float;


#[cfg(not(target_env = "msvc"))]
#[link_name = "m"]
extern {
    pub fn acosf(n: c_float) -> c_float;
    pub fn asinf(n: c_float) -> c_float;
    pub fn atan2f(a: c_float, b: c_float) -> c_float;
    pub fn atanf(n: c_float) -> c_float;
    pub fn coshf(n: c_float) -> c_float;
    pub fn sinhf(n: c_float) -> c_float;
    pub fn tanf(n: c_float) -> c_float;
    pub fn tanhf(n: c_float) -> c_float;
    pub fn log1pf(n: c_float) -> c_float;
}

#[cfg(target_env = "msvc")]
pub use self::shims::*;
#[cfg(target_env = "msvc")]
mod shims {
    use libc::{c_float, c_int};

    #[inline]
    pub unsafe fn acosf(n: c_float) -> c_float {
        f64::acos(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn asinf(n: c_float) -> c_float {
        f64::asin(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn atan2f(n: c_float, b: c_float) -> c_float {
        f64::atan2(n as f64, b as f64) as c_float
    }
    #[inline]
    pub unsafe fn atanf(n: c_float) -> c_float {
        f64::atan(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn coshf(n: c_float) -> c_float {
        f64::cosh(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn sinhf(n: c_float) -> c_float {
        f64::sinh(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn tanf(n: c_float) -> c_float {
        f64::tan(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn tanhf(n: c_float) -> c_float {
        f64::tanh(n as f64) as c_float
    }
    #[inline]
    pub unsafe fn log1pf(n: c_float) -> c_float {
        f64::log1p(n as f64) as c_float
    }
}

extern "rust-intrinsic"  {
    pub fn cosf32(x: f32) -> f32;
    pub fn sinf32(x: f32) -> f32;
    pub fn logf32(x: f32) -> f32;
    pub fn sqrtf32(x: f32) -> f32;
}


pub use self::acosf as acosf32;
pub use self::asinf as asinf32;
pub use self::atanf as atanf32;
pub use self::atan2f as atan2f32;
pub use self::coshf as coshf32;
pub use self::sinhf as sinhf32;
pub use self::tanf as tanf32;
pub use self::tanhf as tanhf32;


#[allow(illegal_floating_point_constant_pattern)]
#[inline]
pub unsafe fn asinhf32(n: f32) -> f32 {
    if n == NEG_INFINITY {
        NEG_INFINITY
    } else {
        logf32(n + sqrtf32((n * n) + 1.0))
    }
}
#[inline]
pub unsafe fn acoshf32(n: f32) -> f32 {
    match n {
        x if x < 1.0 => NAN,
        x => logf32(x + sqrtf32((x * x) - 1.0)),
    }
}
#[inline]
pub unsafe fn atanhf32(n: f32) -> f32 {
    0.5 * log1pf((2.0 * n) / (1.0 - n))
}
