use crate::f128;
use crate::quad::sw::{F128_ABS_MASK, F128_EXP_BIAS, F128_MANT_MASK};
use casting::CastFrom;

const F64_EXP_BIAS: i32 = 1023;
const F64_INF: u64 = 0x7FF0_0000_0000_0000;
const F64_QNAN: u64 = 0x7FF8_0000_0000_0000;
const EXP_BIAS_DIFF: i32 = F128_EXP_BIAS - F64_EXP_BIAS; // 15360

impl CastFrom<f128> for f64 {
    #[inline]
    fn cast_from(value: f128) -> f64 {
        let bits = value.0;
        let sign = (bits >> 127) as u64;
        let exp = ((bits >> 112) & 0x7FFF) as i32;
        let mant = bits & F128_MANT_MASK;

        // Zero (signed)
        if bits & F128_ABS_MASK == 0 {
            return f64::from_bits(sign << 63);
        }

        // Infinity or NaN
        if exp == 0x7FFF {
            return f64::from_bits((sign << 63) | if mant != 0 { F64_QNAN } else { F64_INF });
        }

        // Denormalized f128 → zero (too small for f64)
        if exp == 0 {
            return f64::from_bits(sign << 63);
        }

        // Normal: rebias exponent
        let f64_exp = exp - EXP_BIAS_DIFF;

        // Overflow → infinity
        if f64_exp >= 0x7FF {
            return f64::from_bits((sign << 63) | F64_INF);
        }

        // Underflow → zero (f128 denormals too small for f64)
        if f64_exp <= 0 {
            return f64::from_bits(sign << 63);
        }

        // Extract top 52 bits of 112-bit mantissa with rounding
        let f64_mant = (mant >> 60) as u64;
        let round_bit = ((mant >> 59) & 1) as u64;
        let sticky = (mant & ((1u128 << 59) - 1)) != 0;

        // Round to nearest even
        let do_round = round_bit & ((sticky as u64) | (f64_mant & 1));
        let rounded = f64_mant + do_round;

        if rounded < (1u64 << 52) {
            f64::from_bits((sign << 63) | ((f64_exp as u64) << 52) | rounded)
        } else {
            // Mantissa overflow, increment exponent
            let new_exp = f64_exp + 1;
            if new_exp >= 0x7FF {
                f64::from_bits((sign << 63) | F64_INF)
            } else {
                f64::from_bits((sign << 63) | ((new_exp as u64) << 52))
            }
        }
    }
}

impl CastFrom<f64> for f128 {
    #[inline]
    fn cast_from(value: f64) -> f128 {
        const F128_INF_EXP: u128 = 0x7FFF;

        let bits = value.to_bits();
        let sign = (bits >> 63) as u128;
        let exp = ((bits >> 52) & 0x7FF) as i32;
        let mant = bits & 0xF_FFFF_FFFF_FFFF; // 52-bit mask

        // Zero or denormalized f64 → f128 zero (denormals too small)
        if exp == 0 {
            return f128(sign << 127);
        }

        // Infinity or NaN
        if exp == 0x7FF {
            let nan_payload = if mant != 0 { (mant as u128) << 60 } else { 0 };
            return f128((sign << 127) | (F128_INF_EXP << 112) | nan_payload);
        }

        // Normal: rebias exponent and expand mantissa
        let f128_exp = (exp + EXP_BIAS_DIFF) as u128;

        // Shift 52-bit mantissa to top of 112-bit field (shift left by 60)
        let f128_mant = (mant as u128) << 60;

        f128((sign << 127) | (f128_exp << 112) | f128_mant)
    }
}
