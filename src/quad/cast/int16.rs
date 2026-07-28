use super::{F128_INF, F128_MANT_MASK};
use crate::f128;
use casting::CastFrom;

impl CastFrom<f128> for i16 {
    #[inline]
    fn cast_from(value: f128) -> i16 {
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
            return if sign { i16::MIN } else { i16::MAX };
        }

        let unbiased_exp = exp - 16383;
        if unbiased_exp < 0 {
            return 0;
        }

        // For i16, max unbiased exp is 14 (since 2^15 needs 15 bits, but i16 is 15+sign)
        if unbiased_exp > 14 {
            return if sign { i16::MIN } else { i16::MAX };
        }

        let significand = mant | (1u128 << 112);
        let shift = 112 - unbiased_exp;

        let int_val = if shift >= 0 {
            (significand >> shift) as i16
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

impl CastFrom<i16> for f128 {
    #[inline]
    fn cast_from(value: i16) -> f128 {
        if value == 0 {
            return f128(0);
        }

        let (sign, abs) = if value < 0 {
            (1u128 << 127, (value as i32).unsigned_abs() as u128)
        } else {
            (0u128, value as u128)
        };

        let lz = abs.leading_zeros();
        let msb_pos = 127 - lz;
        let exp = 16383 + msb_pos as i32;

        if exp > 32766 {
            return f128(sign | F128_INF);
        }

        // `abs` is at most 16 significant bits, so `msb_pos <= 15` always --
        // strictly less than the 112-bit mantissa, and thus always exact: no
        // rounding is ever needed, and the wide-shift/round/carry machinery
        // `int128.rs` needs for sources up to 128 bits never applies here.
        let mant_bits = (abs << (112 - msb_pos)) & F128_MANT_MASK;
        f128(sign | ((exp as u128) << 112) | mant_bits)
    }
}

impl CastFrom<u16> for f128 {
    #[inline]
    fn cast_from(value: u16) -> f128 {
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

        // See the `i16` impl above: `msb_pos <= 15` always, so this is exact.
        let mant_bits = (abs << (112 - msb_pos)) & F128_MANT_MASK;
        f128(((exp as u128) << 112) | mant_bits)
    }
}

impl CastFrom<f128> for u16 {
    #[inline]
    fn cast_from(value: f128) -> u16 {
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
            return u16::MAX;
        }

        let unbiased_exp = exp - 16383;
        if unbiased_exp < 0 {
            return 0;
        }

        if unbiased_exp > 15 {
            return u16::MAX;
        }

        let significand = mant | (1u128 << 112);
        let shift = 112 - unbiased_exp;

        if shift >= 0 {
            (significand >> shift) as u16
        } else {
            (significand << -shift) as u16
        }
    }
}
