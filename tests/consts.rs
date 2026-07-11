//! Compatibility tests for f16 and f128 constants/classification.
//!
//! These tests verify that our f16/f128 constants and classification methods
//! behave identically to the nightly standard library types.

#![feature(f16, f128)]

mod common;

use common::{Bitable, Customized};

#[rstest::rstest]
#[case::radix(<f16 as Customized>::Custom::RADIX as i64, f16::RADIX as i64)]
#[case::mantissa_digits(
    <f16 as Customized>::Custom::MANTISSA_DIGITS as i64,
    f16::MANTISSA_DIGITS as i64
)]
#[case::digits(<f16 as Customized>::Custom::DIGITS as i64, f16::DIGITS as i64)]
#[case::min_exp(<f16 as Customized>::Custom::MIN_EXP as i64, f16::MIN_EXP as i64)]
#[case::max_exp(<f16 as Customized>::Custom::MAX_EXP as i64, f16::MAX_EXP as i64)]
#[case::min_10_exp(
    <f16 as Customized>::Custom::MIN_10_EXP as i64,
    f16::MIN_10_EXP as i64
)]
#[case::max_10_exp(
    <f16 as Customized>::Custom::MAX_10_EXP as i64,
    f16::MAX_10_EXP as i64
)]
fn f16_integer_consts_match_primitive(#[case] custom: i64, #[case] primitive: i64) {
    assert_eq!(custom, primitive);
}

#[rstest::rstest]
#[case::epsilon(<f16 as Customized>::Custom::EPSILON, f16::EPSILON)]
#[case::min(<f16 as Customized>::Custom::MIN, f16::MIN)]
#[case::min_positive(
    <f16 as Customized>::Custom::MIN_POSITIVE,
    f16::MIN_POSITIVE
)]
#[case::max(<f16 as Customized>::Custom::MAX, f16::MAX)]
#[case::nan(<f16 as Customized>::Custom::NAN, f16::NAN)]
#[case::infinity(<f16 as Customized>::Custom::INFINITY, f16::INFINITY)]
#[case::neg_infinity(
    <f16 as Customized>::Custom::NEG_INFINITY,
    f16::NEG_INFINITY
)]
fn f16_bit_consts_match_primitive(
    #[case] custom: <f16 as Customized>::Custom,
    #[case] primitive: f16,
) {
    assert_bits_eq(custom, primitive);
}

#[rstest::rstest]
#[case::nan(f16::NAN)]
#[case::signaling_nan(f16::from_bits(0x7c01))]
#[case::neg_nan(f16::from_bits(0xfe00))]
#[case::infinity(f16::INFINITY)]
#[case::neg_infinity(f16::NEG_INFINITY)]
#[case::max(f16::MAX)]
#[case::min(f16::MIN)]
#[case::min_positive(f16::MIN_POSITIVE)]
#[case::subnormal(f16::from_bits(0x0001))]
#[case::neg_subnormal(f16::from_bits(0x8001))]
#[case::zero(0.0f16)]
#[case::neg_zero(-0.0f16)]
#[case::one(1.0f16)]
#[case::neg_one(-1.0f16)]
fn f16_classification_matches_primitive(#[case] primitive: f16) {
    let custom = <<f16 as Customized>::Custom as Bitable>::from_bits(primitive.to_bits());

    assert_eq!(custom.is_nan(), primitive.is_nan());
    assert_eq!(custom.is_infinite(), primitive.is_infinite());
    assert_eq!(custom.is_finite(), primitive.is_finite());
    assert_eq!(custom.is_sign_positive(), primitive.is_sign_positive());
    assert_eq!(custom.is_sign_negative(), primitive.is_sign_negative());
}

#[rstest::rstest]
#[case::radix(<f128 as Customized>::Custom::RADIX as i64, f128::RADIX as i64)]
#[case::mantissa_digits(
    <f128 as Customized>::Custom::MANTISSA_DIGITS as i64,
    f128::MANTISSA_DIGITS as i64
)]
#[case::digits(<f128 as Customized>::Custom::DIGITS as i64, f128::DIGITS as i64)]
#[case::min_exp(
    <f128 as Customized>::Custom::MIN_EXP as i64,
    f128::MIN_EXP as i64
)]
#[case::max_exp(
    <f128 as Customized>::Custom::MAX_EXP as i64,
    f128::MAX_EXP as i64
)]
#[case::min_10_exp(
    <f128 as Customized>::Custom::MIN_10_EXP as i64,
    f128::MIN_10_EXP as i64
)]
#[case::max_10_exp(
    <f128 as Customized>::Custom::MAX_10_EXP as i64,
    f128::MAX_10_EXP as i64
)]
fn f128_integer_consts_match_primitive(#[case] custom: i64, #[case] primitive: i64) {
    assert_eq!(custom, primitive);
}

#[rstest::rstest]
#[case::epsilon(<f128 as Customized>::Custom::EPSILON, f128::EPSILON)]
#[case::min(<f128 as Customized>::Custom::MIN, f128::MIN)]
#[case::min_positive(
    <f128 as Customized>::Custom::MIN_POSITIVE,
    f128::MIN_POSITIVE
)]
#[case::max(<f128 as Customized>::Custom::MAX, f128::MAX)]
#[case::nan(<f128 as Customized>::Custom::NAN, f128::NAN)]
#[case::infinity(<f128 as Customized>::Custom::INFINITY, f128::INFINITY)]
#[case::neg_infinity(
    <f128 as Customized>::Custom::NEG_INFINITY,
    f128::NEG_INFINITY
)]
fn f128_bit_consts_match_primitive(
    #[case] custom: <f128 as Customized>::Custom,
    #[case] primitive: f128,
) {
    assert_bits_eq(custom, primitive);
}

#[rstest::rstest]
#[case::nan(f128::NAN)]
#[case::signaling_nan(f128::from_bits(0x7fff0000000000000000000000000001))]
#[case::neg_nan(f128::from_bits(0xffff8000000000000000000000000000))]
#[case::infinity(f128::INFINITY)]
#[case::neg_infinity(f128::NEG_INFINITY)]
#[case::max(f128::MAX)]
#[case::min(f128::MIN)]
#[case::min_positive(f128::MIN_POSITIVE)]
#[case::subnormal(f128::from_bits(0x00000000000000000000000000000001))]
#[case::neg_subnormal(f128::from_bits(0x80000000000000000000000000000001))]
#[case::zero(0.0f128)]
#[case::neg_zero(-0.0f128)]
#[case::one(1.0f128)]
#[case::neg_one(-1.0f128)]
fn f128_classification_matches_primitive(#[case] primitive: f128) {
    let custom = <<f128 as Customized>::Custom as Bitable>::from_bits(primitive.to_bits());

    assert_eq!(custom.is_nan(), primitive.is_nan());
    assert_eq!(custom.is_infinite(), primitive.is_infinite());
    assert_eq!(custom.is_finite(), primitive.is_finite());
    assert_eq!(custom.is_sign_positive(), primitive.is_sign_positive());
    assert_eq!(custom.is_sign_negative(), primitive.is_sign_negative());
}

fn assert_bits_eq<T, U>(custom: T, primitive: U)
where
    T: Bitable<Bits = U::Bits>,
    U: Bitable,
    U::Bits: core::fmt::Debug + PartialEq,
{
    assert_eq!(custom.to_bits(), primitive.to_bits());
}
