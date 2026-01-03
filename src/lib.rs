#![doc = include_str!("../README.md")]
#![no_std]

mod half;
mod quad;

pub use half::f16;
pub use quad::f128;
