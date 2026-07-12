//! Shared helper traits for tests

#[cfg(not(feature = "nightly"))]
mod bitable;
#[cfg(not(feature = "nightly"))]
mod custom;

#[cfg(not(feature = "nightly"))]
pub use bitable::Bitable;
#[cfg(not(feature = "nightly"))]
pub use custom::Customized;
