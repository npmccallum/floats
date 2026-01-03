#![cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]

use super::super::f16;
use casting::CastFrom;

impl CastFrom<u32> for f16 {
    #[inline]
    fn cast_from(value: u32) -> f16 {
        f16::cast_from(value as f32)
    }
}

impl CastFrom<f16> for u32 {
    #[inline]
    fn cast_from(value: f16) -> u32 {
        f32::cast_from(value).clamp(u32::MIN as f32, u32::MAX as f32) as u32
    }
}

impl CastFrom<i32> for f16 {
    #[inline]
    fn cast_from(value: i32) -> f16 {
        f16::cast_from(value as f32)
    }
}

impl CastFrom<f16> for i32 {
    #[inline]
    fn cast_from(value: f16) -> i32 {
        f32::cast_from(value).clamp(i32::MIN as f32, i32::MAX as f32) as i32
    }
}
