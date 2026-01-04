#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(f16, f128))]
#![no_std]

#[cfg(not(feature = "nightly"))]
mod half;

#[cfg(not(feature = "nightly"))]
mod quad;

#[cfg(not(feature = "nightly"))]
pub use half::f16;

#[cfg(not(feature = "nightly"))]
pub use quad::f128;

#[cfg(feature = "nightly")]
pub use f16;

#[cfg(feature = "nightly")]
pub use f128;
