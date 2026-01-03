use super::super::f16;
use casting::CastFrom;

impl CastFrom<u128> for f16 {
    #[inline]
    fn cast_from(value: u128) -> f16 {
        f16::cast_from(value as f32)
    }
}

impl CastFrom<f16> for u128 {
    #[inline]
    fn cast_from(value: f16) -> u128 {
        f32::cast_from(value).clamp(u128::MIN as f32, u128::MAX as f32) as u128
    }
}

impl CastFrom<i128> for f16 {
    #[inline]
    fn cast_from(value: i128) -> f16 {
        f16::cast_from(value as f32)
    }
}

impl CastFrom<f16> for i128 {
    #[inline]
    fn cast_from(value: f16) -> i128 {
        f32::cast_from(value).clamp(i128::MIN as f32, i128::MAX as f32) as i128
    }
}
