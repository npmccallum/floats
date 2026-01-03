#![cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]

use super::super::f16;
use casting::CastFrom;

impl CastFrom<u64> for f16 {
    #[inline]
    fn cast_from(value: u64) -> f16 {
        f16::cast_from(value as f32)
    }
}

impl CastFrom<f16> for u64 {
    #[inline]
    fn cast_from(value: f16) -> u64 {
        f32::cast_from(value).clamp(u64::MIN as f32, u64::MAX as f32) as u64
    }
}

impl CastFrom<i64> for f16 {
    #[inline]
    fn cast_from(value: i64) -> f16 {
        f16::cast_from(value as f32)
    }
}

impl CastFrom<f16> for i64 {
    #[inline]
    fn cast_from(value: f16) -> i64 {
        f32::cast_from(value).clamp(i64::MIN as f32, i64::MAX as f32) as i64
    }
}
