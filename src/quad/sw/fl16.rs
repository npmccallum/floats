use crate::f128;
use crate::f16;
use crate::quad::sw::{F128_ABS_MASK, F128_EXP_BIAS, F128_MANT_MASK};
use casting::CastFrom;

const F16_EXP_BIAS: i32 = 15;
const F16_INF: u16 = 0x7C00;
const EXP_BIAS_DIFF: i32 = F128_EXP_BIAS - F16_EXP_BIAS; // 16368

impl CastFrom<f128> for f16 {
    #[inline]
    fn cast_from(value: f128) -> f16 {
        let bits = value.0;
        let sign = (bits >> 127) as u16;
        let exp = ((bits >> 112) & 0x7FFF) as i32;
        let mant = bits & F128_MANT_MASK;

        // Zero (signed)
        if bits & F128_ABS_MASK == 0 {
            return f16::from_bits(sign << 15);
        }

        // Infinity or NaN
        if exp == 0x7FFF {
            return f16::from_bits((sign << 15) | if mant != 0 { 0x7E00 } else { F16_INF });
        }

        // Denormalized f128 → zero (too small for f16)
        if exp == 0 {
            return f16::from_bits(sign << 15);
        }

        // Normal: rebias exponent
        let f16_exp = exp - EXP_BIAS_DIFF;

        // Overflow → infinity
        if f16_exp >= 0x1F {
            return f16::from_bits((sign << 15) | F16_INF);
        }

        // Underflow → zero
        if f16_exp <= 0 {
            return f16::from_bits(sign << 15);
        }

        // Extract top 10 bits of 112-bit mantissa with rounding
        let f16_mant = (mant >> 102) as u16;
        let round_bit = ((mant >> 101) & 1) as u16;
        let sticky = (mant & ((1u128 << 101) - 1)) != 0;

        // Round to nearest even
        let do_round = round_bit & ((sticky as u16) | (f16_mant & 1));
        let rounded = f16_mant + do_round;

        if rounded < (1u16 << 10) {
            f16::from_bits((sign << 15) | ((f16_exp as u16) << 10) | rounded)
        } else {
            // Mantissa overflow, increment exponent
            let new_exp = f16_exp + 1;
            if new_exp >= 0x1F {
                f16::from_bits((sign << 15) | F16_INF)
            } else {
                f16::from_bits((sign << 15) | ((new_exp as u16) << 10))
            }
        }
    }
}

impl CastFrom<f16> for f128 {
    #[inline]
    fn cast_from(value: f16) -> f128 {
        const F128_INF_EXP: u128 = 0x7FFF;

        let bits = value.to_bits();
        let sign = (bits >> 15) as u128;
        let exp = ((bits >> 10) & 0x1F) as i32;
        let mant = bits & 0x3FF; // 10-bit mask

        // Zero or denormalized f16 → f128 zero
        if exp == 0 {
            return f128(sign << 127);
        }

        // Infinity or NaN
        if exp == 0x1F {
            let nan_payload = if mant != 0 { (mant as u128) << 102 } else { 0 };
            return f128((sign << 127) | (F128_INF_EXP << 112) | nan_payload);
        }

        // Normal: rebias exponent and expand mantissa
        let f128_exp = (exp + EXP_BIAS_DIFF) as u128;

        // Shift 10-bit mantissa to top of 112-bit field (shift left by 102)
        let f128_mant = (mant as u128) << 102;

        f128((sign << 127) | (f128_exp << 112) | f128_mant)
    }
}
