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
        let bits = value.to_bits();
        let sign = ((bits >> 48) & 0x8000) as u16;
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let mant = bits & 0x000f_ffff_ffff_ffff;

        if exp == 0 {
            return f16(sign);
        }

        if exp == 0x7ff {
            if mant == 0 {
                return f16(sign | 0x7c00);
            }

            // Preserve the high payload bits, as nightly does. The quiet bit is
            // forced on so that a payload living entirely in the truncated low
            // bits cannot turn the NaN into an infinity.
            let payload = (mant >> 42) as u16 & 0x03FF;
            return f16(sign | 0x7c00 | 0x0200 | payload);
        }

        let f16_exp = exp - 1008;
        if f16_exp > 30 {
            return f16(sign | 0x7c00);
        }

        let significand = mant | (1u64 << 52);
        if f16_exp <= 0 {
            let shift = (43 - f16_exp) as u32;
            if shift > 53 {
                return f16(sign);
            }
            let truncated = significand >> shift;
            let remainder = significand & ((1u64 << shift) - 1);
            let halfway = 1u64 << (shift - 1);
            let rounded = truncated
                + u64::from(remainder > halfway || (remainder == halfway && truncated & 1 != 0));
            return f16(sign | rounded as u16);
        }

        let truncated = significand >> 42;
        let remainder = significand & ((1u64 << 42) - 1);
        let halfway = 1u64 << 41;
        let rounded = truncated
            + u64::from(remainder > halfway || (remainder == halfway && truncated & 1 != 0));

        if rounded >= 0x800 {
            if f16_exp >= 30 {
                f16(sign | 0x7c00)
            } else {
                f16(sign | (((f16_exp + 1) as u16) << 10))
            }
        } else {
            f16(sign | ((f16_exp as u16) << 10) | (rounded as u16 & 0x03ff))
        }
    }
}
