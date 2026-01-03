//! Generic PartialEq tests for f16 and f128
//!
//! These tests verify that our f16/f128 PartialEq implementations
//! behave identically to the nightly standard library types.

#![feature(f16, f128)]

mod common;

use common::{Bitable, Customized};

/// Generic PartialEq test
#[rstest::rstest]
// f16 NaN patterns
#[case(f16::NAN, f16::NAN)]
#[case(f16::from_bits(0x7C01), f16::from_bits(0x7C01))]
#[case(f16::from_bits(0x7C01), f16::from_bits(0x7E00))]
#[case(f16::from_bits(0x7C01), f16::from_bits(0x7D00))]
#[case(f16::from_bits(0xFC01), f16::from_bits(0xFC01))]
#[case(f16::from_bits(0x7C01), f16::from_bits(0xFC01))]
#[case(f16::from_bits(0x7C01), 1.0f16)]
#[case(f16::from_bits(0x7C01), f16::INFINITY)]
#[case(f16::from_bits(0x7C01), 0.0f16)]
// f16 Zero equality
#[case(0.0f16, -0.0f16)]
#[case(0.0f16, 0.0f16)]
#[case(-0.0f16, -0.0f16)]
// f16 Infinity equality
#[case(f16::INFINITY, f16::INFINITY)]
#[case(f16::NEG_INFINITY, f16::NEG_INFINITY)]
#[case(f16::INFINITY, f16::NEG_INFINITY)]
#[case(f16::INFINITY, 1.0f16)]
#[case(f16::NEG_INFINITY, 1.0f16)]
// f16 Regular values
#[case(1.0f16, 1.0f16)]
#[case(1.0f16, 2.0f16)]
#[case(1.0f16, -1.0f16)]
#[case(f16::MAX, f16::MAX)]
#[case(f16::MIN, f16::MIN)]
#[case(f16::MAX, f16::MIN)]
#[case(f16::MAX, f16::from_bits(f16::MAX.to_bits() - 1))]
// f128 NaN patterns
#[case(f128::NAN, f128::NAN)]
#[case(
    f128::from_bits(0x7FFF0000000000000000000000000001),
    f128::from_bits(0x7FFF0000000000000000000000000001)
)]
#[case(
    f128::from_bits(0x7FFF0000000000000000000000000001),
    f128::from_bits(0x7FFF8000000000000000000000000000)
)]
#[case(
    f128::from_bits(0x7FFF8000000000000000000000000000),
    f128::from_bits(0x7FFF4000000000000000000000000000)
)]
#[case(
    f128::from_bits(0xFFFF0000000000000000000000000001),
    f128::from_bits(0xFFFF0000000000000000000000000001)
)]
#[case(
    f128::from_bits(0x7FFF0000000000000000000000000001),
    f128::from_bits(0xFFFF0000000000000000000000000001)
)]
#[case(f128::from_bits(0x7FFF0000000000000000000000000001), 1.0f128)]
#[case(f128::from_bits(0x7FFF0000000000000000000000000001), f128::INFINITY)]
#[case(f128::from_bits(0x7FFF0000000000000000000000000001), 0.0f128)]
// f128 Zero equality
#[case(0.0f128, -0.0f128)]
#[case(0.0f128, 0.0f128)]
#[case(-0.0f128, -0.0f128)]
// f128 Infinity equality
#[case(f128::INFINITY, f128::INFINITY)]
#[case(f128::NEG_INFINITY, f128::NEG_INFINITY)]
#[case(f128::INFINITY, f128::NEG_INFINITY)]
#[case(f128::INFINITY, 1.0f128)]
#[case(f128::NEG_INFINITY, 1.0f128)]
// f128 Regular values
#[case(1.0f128, 1.0f128)]
#[case(1.0f128, 2.0f128)]
#[case(1.0f128, -1.0f128)]
#[case(f128::MAX, f128::MAX)]
#[case(f128::MIN, f128::MIN)]
#[case(f128::MAX, f128::MIN)]
#[case(f128::MAX, f128::from_bits(f128::MAX.to_bits() - 1))]
fn test_partial_eq<T>(#[case] a: T, #[case] b: T)
where
    T: PartialEq + Copy + core::fmt::Debug + Bitable + Customized,
    T::Custom: PartialEq + Bitable<Bits = T::Bits>,
{
    let ca = T::Custom::from_bits(a.to_bits());
    let cb = T::Custom::from_bits(b.to_bits());
    assert_eq!(a == b, ca == cb);
}
