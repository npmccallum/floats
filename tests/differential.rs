//! High-density differential checks against the nightly primitive types.

#![cfg(all(feature = "casting", not(feature = "nightly")))]
#![feature(f16, f128)]

use casting::CastFrom;
use floats::{f128 as F128, f16 as F16};

#[test]
fn all_f16_values_match_nightly() {
    for bits in 0..=u16::MAX {
        let primitive = f16::from_bits(bits);
        let custom = F16::from_bits(bits);

        assert_eq!(F16::from_be_bytes(primitive.to_be_bytes()).to_bits(), bits);
        assert_eq!(F16::from_le_bytes(primitive.to_le_bytes()).to_bits(), bits);
        assert_eq!(F16::from_ne_bytes(primitive.to_ne_bytes()).to_bits(), bits);
        assert_eq!((-custom).to_bits(), (-primitive).to_bits());
        assert_eq!(custom.is_nan(), primitive.is_nan());
        assert_eq!(custom.is_infinite(), primitive.is_infinite());
        assert_eq!(custom.is_finite(), primitive.is_finite());
        assert_eq!(custom.is_sign_positive(), primitive.is_sign_positive());
        assert_eq!(custom.is_sign_negative(), primitive.is_sign_negative());

        assert_eq!(
            f32::cast_from(custom).to_bits(),
            (primitive as f32).to_bits()
        );
        assert_eq!(
            f64::cast_from(custom).to_bits(),
            (primitive as f64).to_bits()
        );
        assert_eq!(
            F128::cast_from(custom).to_bits(),
            (primitive as f128).to_bits()
        );

        macro_rules! check_integer {
            ($integer:ty) => {
                assert_eq!(
                    <$integer>::cast_from(custom),
                    primitive as $integer,
                    "f16 bits {bits:#06x} -> {}",
                    stringify!($integer)
                );
            };
        }
        check_integer!(u8);
        check_integer!(i8);
        check_integer!(u16);
        check_integer!(i16);
        check_integer!(u32);
        check_integer!(i32);
        check_integer!(u64);
        check_integer!(i64);
        check_integer!(u128);
        check_integer!(i128);
    }
}

#[test]
fn f16_narrowing_boundaries_match_nightly() {
    for bits in 0..0x7bffu16 {
        let lower = f16::from_bits(bits);
        let upper = f16::from_bits(bits + 1);

        let midpoint32 = ((lower as f32) + (upper as f32)) * 0.5;
        for value in [
            f32::from_bits(midpoint32.to_bits() - 1),
            midpoint32,
            f32::from_bits(midpoint32.to_bits() + 1),
        ] {
            assert_f32_to_f16(value);
            assert_f32_to_f16(-value);
        }

        let midpoint64 = ((lower as f64) + (upper as f64)) * 0.5;
        for value in [
            f64::from_bits(midpoint64.to_bits() - 1),
            midpoint64,
            f64::from_bits(midpoint64.to_bits() + 1),
        ] {
            assert_f64_to_f16(value);
            assert_f64_to_f16(-value);
        }
    }
}

#[test]
fn sampled_f32_values_narrow_like_nightly() {
    let mut state = 0x243f_6a88u32;
    for _ in 0..1_000_000 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        assert_f32_to_f16(f32::from_bits(state));
    }
}

// Windows nightly's primitive f128 lowering can terminate the process for
// structured subnormal inputs. Keep its curated coverage in `cast.rs`; macOS
// and Linux provide the reliable full-exponent oracle for this corpus.
#[cfg(not(target_os = "windows"))]
#[test]
fn structured_f128_values_match_nightly() {
    const MANTISSA_MASK: u128 = (1u128 << 112) - 1;
    const MANTISSAS: [u128; 12] = [
        0,
        1,
        1 << 55,
        1 << 111,
        (1 << 111) - 1,
        (1 << 111) + 1,
        0x0000_5555_5555_5555_5555_5555_5555_5555,
        0x0000_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa,
        0x0000_8000_0000_0000_0000_0000_0000_0001,
        0x0000_7fff_ffff_ffff_ffff_ffff_ffff_ffff,
        MANTISSA_MASK - 1,
        MANTISSA_MASK,
    ];

    for sign in [0, 1u128 << 127] {
        for exponent in 0..=0x7fffu128 {
            for mantissa in MANTISSAS {
                let bits = sign | (exponent << 112) | mantissa;
                let primitive = f128::from_bits(bits);
                let custom = F128::from_bits(bits);

                assert_eq!(
                    f64::cast_from(custom).to_bits(),
                    (primitive as f64).to_bits()
                );
                assert_eq!(
                    f32::cast_from(custom).to_bits(),
                    (primitive as f32).to_bits()
                );
                assert_eq!(
                    F16::cast_from(custom).to_bits(),
                    (primitive as f16).to_bits()
                );

                macro_rules! check_integer {
                    ($integer:ty) => {
                        assert_eq!(
                            <$integer>::cast_from(custom),
                            primitive as $integer,
                            "f128 bits {bits:#034x} -> {}",
                            stringify!($integer)
                        );
                    };
                }
                check_integer!(u8);
                check_integer!(i8);
                check_integer!(u16);
                check_integer!(i16);
                check_integer!(u32);
                check_integer!(i32);
                check_integer!(u64);
                check_integer!(i64);
                check_integer!(u128);
                check_integer!(i128);
            }
        }
    }
}

// Windows nightly also terminates the process while lowering some direct
// wide-integer-to-f128 casts. Its curated cases remain covered by `cast.rs`.
#[cfg(not(target_os = "windows"))]
#[test]
fn wide_integer_precision_boundaries_match_nightly() {
    const VALUES: [u128; 18] = [
        0,
        1,
        (1u128 << 112) - 1,
        1u128 << 112,
        (1u128 << 112) + 1,
        (1u128 << 112) + 2,
        (1u128 << 112) + 3,
        (1u128 << 113) - 3,
        (1u128 << 113) - 2,
        (1u128 << 113) - 1,
        1u128 << 113,
        (1u128 << 113) + 1,
        (1u128 << 113) + 2,
        (1u128 << 113) + 3,
        (1u128 << 127) - 1,
        1u128 << 127,
        u128::MAX - 1,
        u128::MAX,
    ];

    for value in VALUES {
        assert_eq!(F128::cast_from(value).to_bits(), (value as f128).to_bits());
        let signed = value as i128;
        assert_eq!(
            F128::cast_from(signed).to_bits(),
            (signed as f128).to_bits()
        );
    }
}

// Direct 128-bit integer-to-f16 lowering depends on compiler-rt support. Linux
// provides a reliable primitive oracle; macOS needs the shim in `cast.rs`, and
// the Windows nightly lowering can terminate the process for a broad corpus.
#[cfg(target_os = "linux")]
#[test]
fn sampled_wide_integers_narrow_like_nightly() {
    let mut state = 0x9e37_79b9_7f4a_7c15_243f_6a88_85a3_08d3u128;
    for _ in 0..1_000_000 {
        state ^= state << 23;
        state ^= state >> 17;
        state ^= state << 26;

        assert_eq!(F16::cast_from(state).to_bits(), (state as f16).to_bits());
        let signed = state as i128;
        assert_eq!(F16::cast_from(signed).to_bits(), (signed as f16).to_bits());
    }
}

fn assert_f32_to_f16(value: f32) {
    assert_eq!(
        F16::cast_from(value).to_bits(),
        (value as f16).to_bits(),
        "f32 bits {:#010x}",
        value.to_bits()
    );
}

fn assert_f64_to_f16(value: f64) {
    assert_eq!(
        F16::cast_from(value).to_bits(),
        (value as f16).to_bits(),
        "f64 bits {:#018x}",
        value.to_bits()
    );
}
