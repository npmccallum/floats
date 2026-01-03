#![cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "f16c")
)))]

use crate::f16;

use casting::CastFrom;

const F16_INF: u16 = 0x7C00;

/// Lookup table for f16 exponent -> f32 exponent conversion.
/// Index is f16 exponent (0-31), value is f32 exponent shifted to position.
/// Special cases: 0 = denormal/zero, 31 = inf/nan
const F16_TO_F32_EXP: [u32; 32] = {
    let mut table = [0u32; 32];
    let mut i = 1u32;
    while i < 31 {
        // f16 exp i maps to f32 exp (i + 112), shifted to bit position 23
        // 112 = f32 bias (127) - f16 bias (15)
        table[i as usize] = (i + 112) << 23;
        i += 1;
    }
    // exp 31 = infinity/NaN
    table[31] = 0x7F800000;
    table
};

// Only provide software implementation if no hardware acceleration is available

impl CastFrom<f16> for f32 {
    #[inline]
    fn cast_from(value: f16) -> f32 {
        let bits = value.0 as u32;
        let sign = (bits & 0x8000) << 16;
        let exp = (bits >> 10) & 0x1F;
        let mant = bits & 0x3FF;

        // Fast path: normal numbers and infinity/NaN (exponent 1-31)
        if exp != 0 {
            return f32::from_bits(sign | F16_TO_F32_EXP[exp as usize] | (mant << 13));
        }

        // Zero or denormalized
        if mant == 0 {
            return f32::from_bits(sign);
        }

        // Denormalized: normalize by finding the leading 1 bit
        let shift = mant.leading_zeros() - 22;
        let normalized_mant = ((mant << (shift + 1)) & 0x3FF) << 13;
        let f32_exp = (113u32 - shift) << 23;
        f32::from_bits(sign | f32_exp | normalized_mant)
    }
}

impl CastFrom<f32> for f16 {
    #[inline]
    fn cast_from(value: f32) -> Self {
        let bits = value.to_bits();
        let sign = ((bits >> 16) & 0x8000) as u16;
        let f32_exp = (bits >> 23) & 0xFF;
        let f32_mant = bits & 0x7FFFFF;

        // Zero or denormal f32 → zero
        if f32_exp == 0 {
            return Self(sign);
        }

        // Infinity or NaN
        if f32_exp == 255 {
            // Preserve NaN payload (use quiet NaN bit)
            return Self(sign | F16_INF | if f32_mant != 0 { 0x0200 } else { 0 });
        }

        // Rebias exponent
        let f16_exp = f32_exp as i32 - 112;

        // Overflow → infinity (exp > 30, since 31 is reserved for inf/nan)
        if f16_exp > 30 {
            return Self(sign | F16_INF);
        }

        // Denormalized result
        if f16_exp <= 0 {
            let shift = (14 - f16_exp) as u32;
            return if shift >= 24 {
                Self(sign)
            } else {
                Self(sign | (((f32_mant | 0x800000) >> shift) as u16))
            };
        }

        // Normal case with rounding
        let mant = (f32_mant >> 13) as u16;
        let round = ((f32_mant >> 12) & 1) as u16;
        let rounded = mant + round;

        if rounded < 0x400 {
            Self(sign | ((f16_exp as u16) << 10) | rounded)
        } else if f16_exp >= 30 {
            Self(sign | F16_INF)
        } else {
            Self(sign | (((f16_exp + 1) as u16) << 10))
        }
    }
}
