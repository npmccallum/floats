use crate::f128;
use crate::quad::sw::{F128_ABS_MASK, F128_EXP_BIAS, F128_MANT_MASK};
use casting::CastFrom;

const F32_EXP_BIAS: i32 = 127;
const F32_INF: u32 = 0x7F80_0000;
const EXP_BIAS_DIFF: i32 = F128_EXP_BIAS - F32_EXP_BIAS; // 16256

impl CastFrom<f128> for f32 {
    #[inline]
    fn cast_from(value: f128) -> f32 {
        let bits = value.0;
        let sign = (bits >> 127) as u32;
        let exp = ((bits >> 112) & 0x7FFF) as i32;
        let mant = bits & F128_MANT_MASK;

        // Zero (signed)
        if bits & F128_ABS_MASK == 0 {
            return f32::from_bits(sign << 31);
        }

        // Infinity or NaN
        if exp == 0x7FFF {
            return f32::from_bits((sign << 31) | if mant != 0 { 0x7FC0_0000 } else { F32_INF });
        }

        // Denormalized f128 → zero (too small for f32)
        if exp == 0 {
            return f32::from_bits(sign << 31);
        }

        // Normal: rebias exponent
        let f32_exp = exp - EXP_BIAS_DIFF;

        // Overflow → infinity
        if f32_exp >= 0xFF {
            return f32::from_bits((sign << 31) | F32_INF);
        }

        // Subnormal result.
        if f32_exp <= 0 {
            let significand = mant | (1u128 << 112);
            let shift = 89 + (1 - f32_exp) as u32;
            if shift >= 128 {
                return f32::from_bits(sign << 31);
            }
            let truncated = significand >> shift;
            let remainder = significand & ((1u128 << shift) - 1);
            let halfway = 1u128 << (shift - 1);
            let rounded = truncated
                + u128::from(remainder > halfway || (remainder == halfway && truncated & 1 != 0));
            return f32::from_bits((sign << 31) | rounded as u32);
        }

        // Extract top 23 bits of 112-bit mantissa with rounding
        let f32_mant = (mant >> 89) as u32;
        let round_bit = ((mant >> 88) & 1) as u32;
        let sticky = (mant & ((1u128 << 88) - 1)) != 0;

        // Round to nearest even
        let do_round = round_bit & ((sticky as u32) | (f32_mant & 1));
        let rounded = f32_mant + do_round;

        if rounded < (1u32 << 23) {
            f32::from_bits((sign << 31) | ((f32_exp as u32) << 23) | rounded)
        } else {
            // Mantissa overflow, increment exponent
            let new_exp = f32_exp + 1;
            if new_exp >= 0xFF {
                f32::from_bits((sign << 31) | F32_INF)
            } else {
                f32::from_bits((sign << 31) | ((new_exp as u32) << 23))
            }
        }
    }
}

impl CastFrom<f32> for f128 {
    #[inline]
    fn cast_from(value: f32) -> f128 {
        const F128_INF_EXP: u128 = 0x7FFF;

        let bits = value.to_bits();
        let sign = (bits >> 31) as u128;
        let exp = ((bits >> 23) & 0xFF) as i32;
        let mant = bits & 0x7F_FFFF; // 23-bit mask

        // Zero or denormalized f32.
        if exp == 0 {
            if mant == 0 {
                return f128(sign << 127);
            }
            let msb = 31 - mant.leading_zeros();
            let f128_exp = (F128_EXP_BIAS - 149 + msb as i32) as u128;
            let fraction = (mant as u128 - (1u128 << msb)) << (112 - msb);
            return f128((sign << 127) | (f128_exp << 112) | fraction);
        }

        // Infinity or NaN
        if exp == 0xFF {
            let nan_payload = if mant != 0 { (mant as u128) << 89 } else { 0 };
            return f128((sign << 127) | (F128_INF_EXP << 112) | nan_payload);
        }

        // Normal: rebias exponent and expand mantissa
        let f128_exp = (exp + EXP_BIAS_DIFF) as u128;

        // Shift 23-bit mantissa to top of 112-bit field (shift left by 89)
        let f128_mant = (mant as u128) << 89;

        f128((sign << 127) | (f128_exp << 112) | f128_mant)
    }
}
