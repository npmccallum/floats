use crate::f128;
use crate::quad::sw::{F128_INF, F128_MANT_MASK};
use casting::CastFrom;

impl CastFrom<f128> for i128 {
    #[inline]
    fn cast_from(value: f128) -> i128 {
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
            return if sign { i128::MIN } else { i128::MAX };
        }

        let unbiased_exp = exp - 16383;
        if unbiased_exp < 0 {
            return 0;
        }

        // For i128, max unbiased exp is 126 (since 2^127 needs 127 bits, but i128 is 127+sign)
        if unbiased_exp > 126 {
            return if sign { i128::MIN } else { i128::MAX };
        }

        let significand = mant | (1u128 << 112);
        let shift = 112 - unbiased_exp;

        let int_val = if shift >= 0 {
            significand >> shift
        } else {
            significand << -shift
        };

        if sign {
            -(int_val as i128)
        } else {
            int_val as i128
        }
    }
}

impl CastFrom<i128> for f128 {
    #[inline]
    fn cast_from(value: i128) -> f128 {
        if value == 0 {
            return f128(0);
        }

        let (sign, abs) = if value < 0 {
            if value == i128::MIN {
                (1u128 << 127, 1u128 << 127)
            } else {
                (1u128 << 127, value.unsigned_abs())
            }
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

impl CastFrom<u128> for f128 {
    #[inline]
    fn cast_from(value: u128) -> f128 {
        if value == 0 {
            return f128(0);
        }

        let abs = value;
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

impl CastFrom<f128> for u128 {
    #[inline]
    fn cast_from(value: f128) -> u128 {
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
            return u128::MAX;
        }

        let unbiased_exp = exp - 16383;
        if unbiased_exp < 0 {
            return 0;
        }

        if unbiased_exp > 127 {
            return u128::MAX;
        }

        let significand = mant | (1u128 << 112);
        let shift = 112 - unbiased_exp;

        if shift >= 0 {
            significand >> shift
        } else {
            significand << -shift
        }
    }
}
