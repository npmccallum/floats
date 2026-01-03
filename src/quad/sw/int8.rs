use crate::f128;
use crate::quad::sw::{F128_INF, F128_MANT_MASK};
use casting::CastFrom;

impl CastFrom<f128> for i8 {
    #[inline]
    fn cast_from(value: f128) -> i8 {
        let bits = value.0;
        let sign = (bits >> 127) != 0;
        let exp = ((bits >> 112) & 0x7FFF) as i32;
        let mant = bits & F128_MANT_MASK;

        if exp == 0 {
            return 0;
        }

        if exp == 0x7FFF && mant != 0 {
            return 0; // NaN
        }

        if exp == 0x7FFF {
            return if sign { i8::MIN } else { i8::MAX };
        }

        let unbiased_exp = exp - 16383;
        if unbiased_exp < 0 {
            return 0;
        }

        // For i8, max unbiased exp is 6 (since 2^7 needs 7 bits, but i8 is 7+sign)
        if unbiased_exp > 6 {
            return if sign { i8::MIN } else { i8::MAX };
        }

        let significand = mant | (1u128 << 112);
        let shift = 112 - unbiased_exp;

        let int_val = if shift >= 0 {
            (significand >> shift) as i8
        } else {
            0
        };

        if sign {
            -int_val
        } else {
            int_val
        }
    }
}

impl CastFrom<i8> for f128 {
    #[inline]
    fn cast_from(value: i8) -> f128 {
        if value == 0 {
            return f128(0);
        }

        let (sign, abs) = if value < 0 {
            (1u128 << 127, (value as i16).unsigned_abs() as u128)
        } else {
            (0u128, value as u128)
        };

        let lz = abs.leading_zeros();
        let msb_pos = 127 - lz;
        let exp = 16383 + msb_pos as i32;

        if exp > 32766 {
            return f128(sign | F128_INF);
        }

        let shift = msb_pos.saturating_sub(112);
        let mant_bits = if msb_pos >= 112 {
            abs >> shift
        } else {
            abs << (112 - msb_pos)
        };

        let round_bit = if shift > 0 {
            (abs >> (shift - 1)) & 1
        } else {
            0
        };
        let rounded = (mant_bits & F128_MANT_MASK) + round_bit;

        if rounded >= (1u128 << 112) {
            f128(sign | ((exp as u128 + 1) << 112))
        } else {
            f128(sign | ((exp as u128) << 112) | rounded)
        }
    }
}

impl CastFrom<u8> for f128 {
    #[inline]
    fn cast_from(value: u8) -> f128 {
        if value == 0 {
            return f128(0);
        }

        let abs = value as u128;
        let lz = abs.leading_zeros();
        let msb_pos = 127 - lz;
        let exp = 16383 + msb_pos as i32;

        if exp > 32766 {
            return f128(F128_INF);
        }

        let shift = msb_pos.saturating_sub(112);
        let mant_bits = if msb_pos >= 112 {
            abs >> shift
        } else {
            abs << (112 - msb_pos)
        };

        let round_bit = if shift > 0 {
            (abs >> (shift - 1)) & 1
        } else {
            0
        };
        let rounded = (mant_bits & F128_MANT_MASK) + round_bit;

        if rounded >= (1u128 << 112) {
            f128((exp as u128 + 1) << 112)
        } else {
            f128(((exp as u128) << 112) | rounded)
        }
    }
}

impl CastFrom<f128> for u8 {
    #[inline]
    fn cast_from(value: f128) -> u8 {
        let bits = value.0;
        let sign = bits >> 127;
        let exp = ((bits >> 112) & 0x7FFF) as i32;
        let mant = bits & F128_MANT_MASK;

        if sign != 0 {
            return 0;
        }

        if exp == 0 {
            return 0;
        }

        if exp == 0x7FFF && mant != 0 {
            return 0;
        }

        if exp == 0x7FFF {
            return u8::MAX;
        }

        let unbiased_exp = exp - 16383;
        if unbiased_exp < 0 {
            return 0;
        }

        if unbiased_exp > 7 {
            return u8::MAX;
        }

        let significand = mant | (1u128 << 112);
        let shift = 112 - unbiased_exp;

        if shift >= 0 {
            (significand >> shift) as u8
        } else {
            (significand << -shift) as u8
        }
    }
}
