//! Compatibility tests for the manual types' `Debug` implementations.

#![cfg(not(feature = "nightly"))]
#![feature(f16, f128)]

use floats::{f128 as F128, f16 as F16};

#[test]
fn all_f16_values_debug_like_nightly() {
    for bits in 0..=u16::MAX {
        let primitive = f16::from_bits(bits);
        let custom = F16::from_bits(bits);
        assert_eq!(
            format!("{custom:?}"),
            format!("{primitive:?}"),
            "bits {bits:#06x}"
        );
    }
}

#[test]
fn f16_debug_options_match_nightly() {
    for bits in [
        0x0000, 0x8000, 0x0001, 0x03ff, 0x0400, 0x3555, 0x3c00, 0x7bff, 0x7c00, 0xfc00, 0x7c01,
        0x7e00, 0xfe00,
    ] {
        let primitive = f16::from_bits(bits);
        let custom = F16::from_bits(bits);

        assert_eq!(format!("{custom:+?}"), format!("{primitive:+?}"));
        assert_eq!(format!("{custom:12?}"), format!("{primitive:12?}"));
        assert_eq!(format!("{custom:>12?}"), format!("{primitive:>12?}"));
        assert_eq!(format!("{custom:<12?}"), format!("{primitive:<12?}"));
        assert_eq!(format!("{custom:^12?}"), format!("{primitive:^12?}"));
        assert_eq!(format!("{custom:012?}"), format!("{primitive:012?}"));
        assert_eq!(format!("{custom:.3?}"), format!("{primitive:.3?}"));
        assert_eq!(format!("{custom:+.4?}"), format!("{primitive:+.4?}"));
        assert_eq!(format!("{custom:>+10.4?}"), format!("{primitive:>+10.4?}"));
    }
}

#[test]
fn f128_debug_options_match_nightly() {
    for bits in [
        0,
        1,
        0x3fff_0000_0000_0000_0000_0000_0000_0000,
        0x7ffe_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        0x7fff_0000_0000_0000_0000_0000_0000_0000,
        0x7fff_8000_0000_0000_0000_0000_0000_0000,
        0x8000_0000_0000_0000_0000_0000_0000_0000,
        u128::MAX,
    ] {
        let primitive = f128::from_bits(bits);
        let custom = F128::from_bits(bits);

        assert_eq!(format!("{custom:?}"), format!("{primitive:?}"));
        assert_eq!(format!("{custom:+?}"), format!("{primitive:+?}"));
        assert_eq!(format!("{custom:40?}"), format!("{primitive:40?}"));
        assert_eq!(format!("{custom:>40?}"), format!("{primitive:>40?}"));
        assert_eq!(format!("{custom:040?}"), format!("{primitive:040?}"));
    }
}
