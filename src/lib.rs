#![feature(intrinsics)]
#![no_std]


extern crate libc;


mod trig_f32;
mod trig_f64;
mod trig;


pub use trig_f32::*;
pub use trig_f64::*;
pub use trig::Trig;
