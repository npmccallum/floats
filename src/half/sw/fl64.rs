#![cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]

use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for f64 {
    #[inline]
    fn cast_from(value: f16) -> f64 {
        f32::cast_from(value) as f64
    }
}

impl CastFrom<f64> for f16 {
    #[inline]
    fn cast_from(value: f64) -> f16 {
        f16::cast_from(value as f32)
    }
}
