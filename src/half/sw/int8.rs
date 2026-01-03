use super::super::f16;
use casting::CastFrom;

impl CastFrom<u8> for f16 {
    #[inline]
    fn cast_from(value: u8) -> f16 {
        f16::cast_from(value as u32)
    }
}

impl CastFrom<f16> for u8 {
    #[inline]
    fn cast_from(value: f16) -> u8 {
        u32::cast_from(value).clamp(u8::MIN as u32, u8::MAX as u32) as u8
    }
}

impl CastFrom<i8> for f16 {
    #[inline]
    fn cast_from(value: i8) -> f16 {
        f16::cast_from(value as i32)
    }
}

impl CastFrom<f16> for i8 {
    #[inline]
    fn cast_from(value: f16) -> i8 {
        i32::cast_from(value).clamp(i8::MIN as i32, i8::MAX as i32) as i8
    }
}
