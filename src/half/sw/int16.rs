use super::super::f16;
use casting::CastFrom;

impl CastFrom<u16> for f16 {
    #[inline]
    fn cast_from(value: u16) -> f16 {
        f16::cast_from(value as u32)
    }
}

impl CastFrom<f16> for u16 {
    #[inline]
    fn cast_from(value: f16) -> u16 {
        u32::cast_from(value).clamp(u16::MIN as u32, u16::MAX as u32) as u16
    }
}

impl CastFrom<i16> for f16 {
    #[inline]
    fn cast_from(value: i16) -> f16 {
        f16::cast_from(value as i32)
    }
}

impl CastFrom<f16> for i16 {
    #[inline]
    fn cast_from(value: f16) -> i16 {
        i32::cast_from(value).clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }
}
