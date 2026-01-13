//! Generic cast tests for f16 and f128
//!
//! These tests verify that our f16/f128 CastFrom/CastInto implementations
//! behave identically to the nightly standard library types.

#![cfg(feature = "casting")]
#![feature(f16, f128)]

// Workaround for missing compiler-rt symbols on aarch64 macOS
// Double-casting strategy: go through f64 as an intermediate type
#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn __floattihf(a: i128) -> f16 {
    (a as f64) as f16
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn __floatuntihf(a: u128) -> f16 {
    (a as f64) as f16
}

mod common;

use common::Bitable;

use std::fmt::Debug;

use casting::CastInto;

use crate::common::Customized;

/// Test casting FROM types that have custom equivalents (f16, f128)
#[rstest::rstest]
// f16 -> f32 cases
#[case::f16_f32_neg_nan(-f16::NAN, -f32::NAN)]
#[case::f16_f32_neg_inf(f16::NEG_INFINITY, f32::NEG_INFINITY)]
#[case::f16_f32_min(f16::MIN, f16::MIN as f32)]
#[case::f16_f32_neg_subnormal(-2.0f16.powi(f16::MANTISSA_DIGITS as i32), -2.0f32.powi(f16::MANTISSA_DIGITS as i32))]
#[case::f16_f32_neg_subnormal_plus_one(-2.0f16.powi(f16::MANTISSA_DIGITS as i32) + 1.0, -(2.0f32.powi(f16::MANTISSA_DIGITS as i32)) + 1.0)]
#[case::f16_f32_neg_one(-1.0f16, -1.0f32)]
#[case::f16_f32_neg_zero(-0.0f16, -0.0f32)]
#[case::f16_f32_zero(0.0f16, 0.0f32)]
#[case::f16_f32_one(1.0f16, 1.0f32)]
#[case::f16_f32_subnormal_minus_one(2.0f16.powi(f16::MANTISSA_DIGITS as i32) - 1.0, (2.0f32.powi(f16::MANTISSA_DIGITS as i32)) - 1.0)]
#[case::f16_f32_subnormal(2.0f16.powi(f16::MANTISSA_DIGITS as i32), 2.0f32.powi(f16::MANTISSA_DIGITS as i32))]
#[case::f16_f32_max(f16::MAX, f16::MAX as f32)]
#[case::f16_f32_inf(f16::INFINITY, f32::INFINITY)]
#[case::f16_f32_nan(f16::NAN, f32::NAN)]
// f16 -> f64 cases
#[case::f16_f64_neg_nan(-f16::NAN, -f64::NAN)]
#[case::f16_f64_neg_inf(f16::NEG_INFINITY, f64::NEG_INFINITY)]
#[case::f16_f64_min(f16::MIN, f16::MIN as f64)]
#[case::f16_f64_neg_subnormal(-2.0f16.powi(f16::MANTISSA_DIGITS as i32), -2.0f64.powi(f16::MANTISSA_DIGITS as i32))]
#[case::f16_f64_neg_subnormal_plus_one(-2.0f16.powi(f16::MANTISSA_DIGITS as i32) + 1.0, -(2.0f64.powi(f16::MANTISSA_DIGITS as i32)) + 1.0)]
#[case::f16_f64_neg_one(-1.0f16, -1.0f64)]
#[case::f16_f64_neg_zero(-0.0f16, -0.0f64)]
#[case::f16_f64_zero(0.0f16, 0.0f64)]
#[case::f16_f64_one(1.0f16, 1.0f64)]
#[case::f16_f64_subnormal_minus_one(2.0f16.powi(f16::MANTISSA_DIGITS as i32) - 1.0, (2.0f64.powi(f16::MANTISSA_DIGITS as i32)) - 1.0)]
#[case::f16_f64_subnormal(2.0f16.powi(f16::MANTISSA_DIGITS as i32), 2.0f64.powi(f16::MANTISSA_DIGITS as i32))]
#[case::f16_f64_max(f16::MAX, f16::MAX as f64)]
#[case::f16_f64_inf(f16::INFINITY, f64::INFINITY)]
#[case::f16_f64_nan(f16::NAN, f64::NAN)]
// f16 -> u8 cases
#[case::f16_u8_min(u8::MIN as f16, u8::MIN)]
#[case::f16_u8_max(u8::MAX as f16, u8::MAX)]
#[case::f16_u8_default(u8::default() as f16, u8::default())]
#[case::f16_u8_one(1.0f16, 1u8)]
// f16 -> i8 cases
#[case::f16_i8_min(i8::MIN as f16, i8::MIN)]
#[case::f16_i8_max(i8::MAX as f16, i8::MAX)]
#[case::f16_i8_default(i8::default() as f16, i8::default())]
#[case::f16_i8_one(1.0f16, 1i8)]
// f16 -> u16 cases
#[case::f16_u16_min(u16::MIN as f16, u16::MIN)]
#[case::f16_u16_max(u16::MAX as f16, u16::MAX)]
#[case::f16_u16_default(u16::default() as f16, u16::default())]
#[case::f16_u16_one(1.0f16, 1u16)]
// f16 -> i16 cases
#[case::f16_i16_min(i16::MIN as f16, i16::MIN)]
#[case::f16_i16_max(i16::MAX as f16, i16::MAX)]
#[case::f16_i16_default(i16::default() as f16, i16::default())]
#[case::f16_i16_one(1.0f16, 1i16)]
// f16 -> u32 cases
#[case::f16_u32_min(u32::MIN as f16, u32::MIN)]
#[case::f16_u32_max(u32::MAX as f16, u32::MAX)]
#[case::f16_u32_default(u32::default() as f16, u32::default())]
#[case::f16_u32_one(1.0f16, 1u32)]
// f16 -> i32 cases
#[case::f16_i32_min(i32::MIN as f16, i32::MIN)]
#[case::f16_i32_max(i32::MAX as f16, i32::MAX)]
#[case::f16_i32_default(i32::default() as f16, i32::default())]
#[case::f16_i32_one(1.0f16, 1i32)]
// f16 -> u64 cases
#[case::f16_u64_min(u64::MIN as f16, u64::MIN)]
#[case::f16_u64_max(u64::MAX as f16, u64::MAX)]
#[case::f16_u64_default(u64::default() as f16, u64::default())]
#[case::f16_u64_one(1.0f16, 1u64)]
// f16 -> i64 cases
#[case::f16_i64_min(i64::MIN as f16, i64::MIN)]
#[case::f16_i64_max(i64::MAX as f16, i64::MAX)]
#[case::f16_i64_default(i64::default() as f16, i64::default())]
#[case::f16_i64_one(1.0f16, 1i64)]
// f16 -> u128 cases
#[case::f16_u128_min(u128::MIN as f16, u128::MIN)]
#[case::f16_u128_max(u128::MAX as f16, u128::MAX)] // breaks when optimization is off
#[case::f16_u128_default(u128::default() as f16, u128::default())]
#[case::f16_u128_one(1.0f16, 1u128)]
// f16 -> i128 cases
#[case::f16_i128_min(i128::MIN as f16, i128::MIN)] // breaks when optimization is off
#[case::f16_i128_max(i128::MAX as f16, i128::MAX)] // breaks when optimization is off
#[case::f16_i128_default(i128::default() as f16, i128::default())]
#[case::f16_i128_one(1.0f16, 1i128)]
// f128 -> f64 cases
#[case::f128_f64_neg_nan(-f128::NAN, -f64::NAN)]
#[case::f128_f64_neg_inf(f128::NEG_INFINITY, f64::NEG_INFINITY)]
#[case::f128_f64_min(f64::MIN as f128, f64::MIN)]
#[case::f128_f64_neg_subnormal(-2.0f128.powi(f64::MANTISSA_DIGITS as i32), -2.0f64.powi(f64::MANTISSA_DIGITS as i32))]
#[case::f128_f64_neg_subnormal_plus_one(-2.0f128.powi(f64::MANTISSA_DIGITS as i32) + 1.0, -2.0f64.powi(f64::MANTISSA_DIGITS as i32) + 1.0)]
#[case::f128_f64_neg_one(-1.0f128, -1.0f64)]
#[case::f128_f64_neg_zero(-0.0f128, -0.0f64)]
#[case::f128_f64_zero(0.0f128, 0.0f64)]
#[case::f128_f64_one(1.0f128, 1.0f64)]
#[case::f128_f64_subnormal_minus_one(2.0f128.powi(f64::MANTISSA_DIGITS as i32) - 1.0, 2.0f64.powi(f64::MANTISSA_DIGITS as i32) - 1.0)]
#[case::f128_f64_subnormal(2.0f128.powi(f64::MANTISSA_DIGITS as i32), 2.0f64.powi(f64::MANTISSA_DIGITS as i32))]
#[case::f128_f64_max(f64::MAX as f128, f64::MAX)]
#[case::f128_f64_inf(f128::INFINITY, f64::INFINITY)]
#[case::f128_f64_nan(f128::NAN, f64::NAN)]
// f128 -> f32 cases
#[case::f128_f32_neg_nan(-f128::NAN, -f32::NAN)]
#[case::f128_f32_neg_inf(f128::NEG_INFINITY, f32::NEG_INFINITY)]
#[case::f128_f32_min(f32::MIN as f128, f32::MIN)]
#[case::f128_f32_neg_one(-1.0f128, -1.0f32)]
#[case::f128_f32_neg_zero(-0.0f128, -0.0f32)]
#[case::f128_f32_zero(0.0f128, 0.0f32)]
#[case::f128_f32_one(1.0f128, 1.0f32)]
#[case::f128_f32_max(f32::MAX as f128, f32::MAX)]
#[case::f128_f32_inf(f128::INFINITY, f32::INFINITY)]
#[case::f128_f32_nan(f128::NAN, f32::NAN)]
// f128 -> u8 cases
#[case::f128_u8_min(u8::MIN as f128, u8::MIN)]
#[case::f128_u8_max(u8::MAX as f128, u8::MAX)]
#[case::f128_u8_default(u8::default() as f128, u8::default())]
#[case::f128_u8_one(1.0f128, 1u8)]
// f128 -> i8 cases
#[case::f128_i8_min(i8::MIN as f128, i8::MIN)]
#[case::f128_i8_max(i8::MAX as f128, i8::MAX)]
#[case::f128_i8_default(i8::default() as f128, i8::default())]
#[case::f128_i8_one(1.0f128, 1i8)]
// f128 -> u16 cases
#[case::f128_u16_min(u16::MIN as f128, u16::MIN)]
#[case::f128_u16_max(u16::MAX as f128, u16::MAX)]
#[case::f128_u16_default(u16::default() as f128, u16::default())]
#[case::f128_u16_one(1.0f128, 1u16)]
// f128 -> i16 cases
#[case::f128_i16_min(i16::MIN as f128, i16::MIN)]
#[case::f128_i16_max(i16::MAX as f128, i16::MAX)]
#[case::f128_i16_default(i16::default() as f128, i16::default())]
#[case::f128_i16_one(1.0f128, 1i16)]
// f128 -> u32 cases
#[case::f128_u32_min(u32::MIN as f128, u32::MIN)]
#[case::f128_u32_max(u32::MAX as f128, u32::MAX)]
#[case::f128_u32_default(u32::default() as f128, u32::default())]
#[case::f128_u32_one(1.0f128, 1u32)]
// f128 -> i32 cases
#[case::f128_i32_min(i32::MIN as f128, i32::MIN)]
#[case::f128_i32_max(i32::MAX as f128, i32::MAX)]
#[case::f128_i32_default(i32::default() as f128, i32::default())]
#[case::f128_i32_one(1.0f128, 1i32)]
// f128 -> u64 cases
#[case::f128_u64_min(u64::MIN as f128, u64::MIN)]
#[case::f128_u64_max(u64::MAX as f128, u64::MAX)]
#[case::f128_u64_default(u64::default() as f128, u64::default())]
#[case::f128_u64_one(1.0f128, 1u64)]
// f128 -> i64 cases
#[case::f128_i64_min(i64::MIN as f128, i64::MIN)]
#[case::f128_i64_max(i64::MAX as f128, i64::MAX)]
#[case::f128_i64_default(i64::default() as f128, i64::default())]
#[case::f128_i64_one(1.0f128, 1i64)]
// f128 -> u128 cases
#[case::f128_u128_min(u128::MIN as f128, u128::MIN)]
#[case::f128_u128_max(u128::MAX as f128, u128::MAX)] // breaks when optimization is off
#[case::f128_u128_default(u128::default() as f128, u128::default())]
#[case::f128_u128_one(1.0f128, 1u128)]
// f128 -> i128 cases
#[case::f128_i128_min(i128::MIN as f128, i128::MIN)] // breaks when optimization is off
#[case::f128_i128_max(i128::MAX as f128, i128::MAX)] // breaks when optimization is off
#[case::f128_i128_default(i128::default() as f128, i128::default())]
#[case::f128_i128_one(1.0f128, 1i128)]
fn test_cast_from<F, I>(#[case] from: F, #[case] into: I)
where
    F: Copy + Bitable + CastInto<I> + Customized,
    I: Debug + Copy + Bitable<Bits: Debug + PartialEq>,
    F::Custom: Copy + Bitable<Bits = F::Bits> + CastInto<I>,
{
    let cust = F::Custom::from_bits(from.to_bits());

    let std = from.cast_into();
    let our = cust.cast_into();

    // If the outputs here are different, it means we have an implementation bug.
    eprintln!("std: {:?}, our: {:?}", std, our);

    // Fails when the test case inputs are bad or the platform is broken.
    assert_eq!(std.to_bits(), into.to_bits());

    // Fails when we have an implementation bug.
    assert_eq!(our.to_bits(), into.to_bits());
}

/// Test casting INTO types that have custom equivalents (f16, f128)
#[rstest::rstest]
// f32 -> f16 cases
#[case::f32_f16_neg_nan(-f32::NAN, -f16::NAN)]
#[case::f32_f16_neg_inf(f32::NEG_INFINITY, f16::NEG_INFINITY)]
#[case::f32_f16_min(f16::MIN as f32, f16::MIN)]
#[case::f32_f16_neg_subnormal(-(2.0f32.powi(f16::MANTISSA_DIGITS as i32)), -2.0f16.powi(f16::MANTISSA_DIGITS as i32))]
#[case::f32_f16_neg_subnormal_plus_one(-(2.0f32.powi(f16::MANTISSA_DIGITS as i32)) + 1.0, -2.0f16.powi(f16::MANTISSA_DIGITS as i32) + 1.0)]
#[case::f32_f16_neg_one(-1.0f32, -1.0f16)]
#[case::f32_f16_neg_zero(-0.0f32, -0.0f16)]
#[case::f32_f16_zero(0.0f32, 0.0f16)]
#[case::f32_f16_one(1.0f32, 1.0f16)]
#[case::f32_f16_subnormal_minus_one((2.0f32.powi(f16::MANTISSA_DIGITS as i32)) - 1.0, 2.0f16.powi(f16::MANTISSA_DIGITS as i32) - 1.0)]
#[case::f32_f16_subnormal(2.0f32.powi(f16::MANTISSA_DIGITS as i32), 2.0f16.powi(f16::MANTISSA_DIGITS as i32))]
#[case::f32_f16_max(f16::MAX as f32, f16::MAX)]
#[case::f32_f16_inf(f32::INFINITY, f16::INFINITY)]
#[case::f32_f16_nan(f32::NAN, f16::NAN)]
// u8 -> f16 cases
#[case::u8_f16_min(u8::MIN, u8::MIN as f16)]
#[case::u8_f16_max(u8::MAX, u8::MAX as f16)]
#[case::u8_f16_default(u8::default(), u8::default() as f16)]
#[case::u8_f16_one(1u8, 1.0f16)]
// i8 -> f16 cases
#[case::i8_f16_min(i8::MIN, i8::MIN as f16)]
#[case::i8_f16_max(i8::MAX, i8::MAX as f16)]
#[case::i8_f16_default(i8::default(), i8::default() as f16)]
#[case::i8_f16_one(1i8, 1.0f16)]
// u16 -> f16 cases
#[case::u16_f16_min(u16::MIN, u16::MIN as f16)]
#[case::u16_f16_max(u16::MAX, u16::MAX as f16)]
#[case::u16_f16_default(u16::default(), u16::default() as f16)]
#[case::u16_f16_one(1u16, 1.0f16)]
// i16 -> f16 cases
#[case::i16_f16_min(i16::MIN, i16::MIN as f16)]
#[case::i16_f16_max(i16::MAX, i16::MAX as f16)]
#[case::i16_f16_default(i16::default(), i16::default() as f16)]
#[case::i16_f16_one(1i16, 1.0f16)]
// u32 -> f16 cases
#[case::u32_f16_min(u32::MIN, u32::MIN as f16)]
#[case::u32_f16_max(u32::MAX, u32::MAX as f16)]
#[case::u32_f16_default(u32::default(), u32::default() as f16)]
#[case::u32_f16_one(1u32, 1.0f16)]
// i32 -> f16 cases
#[case::i32_f16_min(i32::MIN, i32::MIN as f16)]
#[case::i32_f16_max(i32::MAX, i32::MAX as f16)]
#[case::i32_f16_default(i32::default(), i32::default() as f16)]
#[case::i32_f16_one(1i32, 1.0f16)]
// u64 -> f16 cases
#[case::u64_f16_min(u64::MIN, u64::MIN as f16)]
#[case::u64_f16_max(u64::MAX, u64::MAX as f16)]
#[case::u64_f16_default(u64::default(), u64::default() as f16)]
#[case::u64_f16_one(1u64, 1.0f16)]
// i64 -> f16 cases
#[case::i64_f16_min(i64::MIN, i64::MIN as f16)]
#[case::i64_f16_max(i64::MAX, i64::MAX as f16)]
#[case::i64_f16_default(i64::default(), i64::default() as f16)]
#[case::i64_f16_one(1i64, 1.0f16)]
// u128 -> f16 cases
#[case::u128_f16_min(u128::MIN, u128::MIN as f16)]
#[case::u128_f16_max(u128::MAX, u128::MAX as f16)]
#[case::u128_f16_default(u128::default(), u128::default() as f16)]
#[case::u128_f16_one(1u128, 1.0f16)]
// i128 -> f16 cases
#[case::i128_f16_min(i128::MIN, i128::MIN as f16)]
#[case::i128_f16_max(i128::MAX, i128::MAX as f16)]
#[case::i128_f16_default(i128::default(), i128::default() as f16)]
#[case::i128_f16_one(1i128, 1.0f16)]
// f64 -> f128 cases
#[case::f64_f128_neg_nan(-f64::NAN, -f128::NAN)]
#[case::f64_f128_neg_inf(f64::NEG_INFINITY, f128::NEG_INFINITY)]
#[case::f64_f128_min(f64::MIN, f64::MIN as f128)]
#[case::f64_f128_neg_subnormal(-2.0f64.powi(f64::MANTISSA_DIGITS as i32), -2.0f128.powi(f64::MANTISSA_DIGITS as i32))]
#[case::f64_f128_neg_subnormal_plus_one(-2.0f64.powi(f64::MANTISSA_DIGITS as i32) + 1.0, -2.0f128.powi(f64::MANTISSA_DIGITS as i32) + 1.0)]
#[case::f64_f128_neg_one(-1.0f64, -1.0f128)]
#[case::f64_f128_neg_zero(-0.0f64, -0.0f128)]
#[case::f64_f128_zero(0.0f64, 0.0f128)]
#[case::f64_f128_one(1.0f64, 1.0f128)]
#[case::f64_f128_subnormal_minus_one(2.0f64.powi(f64::MANTISSA_DIGITS as i32) - 1.0, 2.0f128.powi(f64::MANTISSA_DIGITS as i32) - 1.0)]
#[case::f64_f128_subnormal(2.0f64.powi(f64::MANTISSA_DIGITS as i32), 2.0f128.powi(f64::MANTISSA_DIGITS as i32))]
#[case::f64_f128_max(f64::MAX, f64::MAX as f128)]
#[case::f64_f128_inf(f64::INFINITY, f128::INFINITY)]
#[case::f64_f128_nan(f64::NAN, f128::NAN)]
// f32 -> f128 cases
#[case::f32_f128_neg_nan(-f32::NAN, -f128::NAN)]
#[case::f32_f128_neg_inf(f32::NEG_INFINITY, f128::NEG_INFINITY)]
#[case::f32_f128_min(f32::MIN, f32::MIN as f128)]
#[case::f32_f128_neg_one(-1.0f32, -1.0f128)]
#[case::f32_f128_neg_zero(-0.0f32, -0.0f128)]
#[case::f32_f128_zero(0.0f32, 0.0f128)]
#[case::f32_f128_one(1.0f32, 1.0f128)]
#[case::f32_f128_max(f32::MAX, f32::MAX as f128)]
#[case::f32_f128_inf(f32::INFINITY, f128::INFINITY)]
#[case::f32_f128_nan(f32::NAN, f128::NAN)]
// u8 -> f128 cases
#[case::u8_f128_min(u8::MIN, u8::MIN as f128)]
#[case::u8_f128_max(u8::MAX, u8::MAX as f128)]
#[case::u8_f128_default(u8::default(), u8::default() as f128)]
#[case::u8_f128_one(1u8, 1.0f128)]
// i8 -> f128 cases
#[case::i8_f128_min(i8::MIN, i8::MIN as f128)]
#[case::i8_f128_max(i8::MAX, i8::MAX as f128)]
#[case::i8_f128_default(i8::default(), i8::default() as f128)]
#[case::i8_f128_one(1i8, 1.0f128)]
// u16 -> f128 cases
#[case::u16_f128_min(u16::MIN, u16::MIN as f128)]
#[case::u16_f128_max(u16::MAX, u16::MAX as f128)]
#[case::u16_f128_default(u16::default(), u16::default() as f128)]
#[case::u16_f128_one(1u16, 1.0f128)]
// i16 -> f128 cases
#[case::i16_f128_min(i16::MIN, i16::MIN as f128)]
#[case::i16_f128_max(i16::MAX, i16::MAX as f128)]
#[case::i16_f128_default(i16::default(), i16::default() as f128)]
#[case::i16_f128_one(1i16, 1.0f128)]
// u32 -> f128 cases
#[case::u32_f128_min(u32::MIN, u32::MIN as f128)]
#[case::u32_f128_max(u32::MAX, u32::MAX as f128)]
#[case::u32_f128_default(u32::default(), u32::default() as f128)]
#[case::u32_f128_one(1u32, 1.0f128)]
// i32 -> f128 cases
#[case::i32_f128_min(i32::MIN, i32::MIN as f128)]
#[case::i32_f128_max(i32::MAX, i32::MAX as f128)]
#[case::i32_f128_default(i32::default(), i32::default() as f128)]
#[case::i32_f128_one(1i32, 1.0f128)]
// u64 -> f128 cases
#[case::u64_f128_min(u64::MIN, u64::MIN as f128)]
#[case::u64_f128_max(u64::MAX, u64::MAX as f128)]
#[case::u64_f128_default(u64::default(), u64::default() as f128)]
#[case::u64_f128_one(1u64, 1.0f128)]
// i64 -> f128 cases
#[case::i64_f128_min(i64::MIN, i64::MIN as f128)]
#[case::i64_f128_max(i64::MAX, i64::MAX as f128)]
#[case::i64_f128_default(i64::default(), i64::default() as f128)]
#[case::i64_f128_one(1i64, 1.0f128)]
// u128 -> f128 cases
#[case::u128_f128_min(u128::MIN, u128::MIN as f128)]
#[case::u128_f128_max(u128::MAX, u128::MAX as f128)] // breaks when optimization is off
#[case::u128_f128_default(u128::default(), u128::default() as f128)]
#[case::u128_f128_one(1u128, 1.0f128)]
// i128 -> f128 cases
#[case::i128_f128_min(i128::MIN, i128::MIN as f128)] // breaks when optimization is off
#[case::i128_f128_max(i128::MAX, i128::MAX as f128)] // breaks when optimization is off
#[case::i128_f128_default(i128::default(), i128::default() as f128)]
#[case::i128_f128_one(1i128, 1.0f128)]
fn test_cast_into<F, I>(#[case] from: F, #[case] into: I)
where
    F: Copy + Bitable + CastInto<I> + CastInto<I::Custom>,
    I: Debug + Copy + Bitable<Bits: Debug + PartialEq> + Customized,
    I::Custom: Debug + Copy + Bitable<Bits = I::Bits>,
{
    let test: I = from.cast_into();
    let cust: I::Custom = from.cast_into();

    // If the outputs here are different, it means we have an implementation bug.
    eprintln!("test: {:?}, cust: {:?}", test, cust);

    // Fails when the test case inputs are bad or the platform is broken.
    assert_eq!(test.to_bits(), into.to_bits());

    // Fails when we have an implementation bug.
    assert_eq!(cust.to_bits(), into.to_bits());
}

/// Test casting between custom f16 and custom f128 types
/// These don't go through stdlib types since there's no conversion between stdlib f16 and custom f128
#[rstest::rstest]
// f128 -> f16 cases
#[case::f128_f16_neg_nan(-f128::NAN, -f16::NAN)]
#[case::f128_f16_neg_inf(f128::NEG_INFINITY, f16::NEG_INFINITY)]
#[case::f128_f16_min(f128::MIN, f16::from_bits(0xfc00))]
#[case::f128_f16_neg_one(-1.0f128, -1.0f16)]
#[case::f128_f16_neg_zero(-0.0f128, -0.0f16)]
#[case::f128_f16_zero(0.0f128, 0.0f16)]
#[case::f128_f16_one(1.0f128, 1.0f16)]
#[case::f128_f16_max(f128::MAX, f16::from_bits(0x7c00))]
#[case::f128_f16_inf(f128::INFINITY, f16::INFINITY)]
#[case::f128_f16_nan(f128::NAN, f16::NAN)]
// f16 -> f128 cases
#[case::f16_f128_neg_nan(-f16::NAN, -f128::NAN)]
#[case::f16_f128_neg_inf(f16::NEG_INFINITY, f128::NEG_INFINITY)]
#[case::f16_f128_min(f16::MIN, f16::MIN as f128)]
#[case::f16_f128_neg_one(-1.0f16, -1.0f128)]
#[case::f16_f128_neg_zero(-0.0f16, -0.0f128)]
#[case::f16_f128_zero(0.0f16, 0.0f128)]
#[case::f16_f128_one(1.0f16, 1.0f128)]
#[case::f16_f128_max(f16::MAX, f16::MAX as f128)]
#[case::f16_f128_inf(f16::INFINITY, f128::INFINITY)]
#[case::f16_f128_nan(f16::NAN, f128::NAN)]
fn test_custom_f16_f128<F, I>(#[case] from: F, #[case] into: I)
where
    F: Copy + Bitable + Customized + CastInto<I>,
    I: Copy + Bitable<Bits: Debug + PartialEq> + Customized,
    F::Custom: Copy + Bitable<Bits = F::Bits> + CastInto<I::Custom>,
    I::Custom: Copy + Bitable<Bits = I::Bits>,
{
    let test: I = from.cast_into();
    let cust: I::Custom = F::Custom::from_bits(from.to_bits()).cast_into();

    // Fails when the test case inputs are bad or the platform is broken.
    assert_eq!(test.to_bits(), into.to_bits());

    // Fails when we have an implementation bug.
    assert_eq!(cust.to_bits(), into.to_bits());
}
