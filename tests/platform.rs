#![feature(f16)]

use std::fmt::Debug;

// This test demonstrates whether the platform conversions are sane or not.
// If you run this at `opt-level=0`, you may see breakage.
#[rstest::rstest]
#[case::u8_min(u8::MIN as f16 as u8, u8::MIN)]
#[case::u8_max(u8::MAX as f16 as u8, u8::MAX)]
#[case::i8_min(i8::MIN as f16 as i8, i8::MIN)]
#[case::i8_max(i8::MAX as f16 as i8, i8::MAX)]
#[case::u16_min(u16::MIN as f16 as u16, u16::MIN)]
#[case::u16_max(u16::MAX as f16 as u16, u16::MAX)]
#[case::i16_min(i16::MIN as f16 as i16, i16::MIN)]
#[case::i16_max(i16::MAX as f16 as i16, i16::MAX)]
#[case::u32_min(u32::MIN as f16 as u32, u32::MIN)]
#[case::u32_max(u32::MAX as f16 as u32, u32::MAX)]
#[case::i32_min(i32::MIN as f16 as i32, i32::MIN)]
#[case::i32_max(i32::MAX as f16 as i32, i32::MAX)]
#[case::u64_min(u64::MIN as f16 as u64, u64::MIN)]
#[case::u64_max(u64::MAX as f16 as u64, u64::MAX)]
#[case::i64_min(i64::MIN as f16 as i64, i64::MIN)]
#[case::i64_max(i64::MAX as f16 as i64, i64::MAX)]
#[case::u128_min(u128::MIN as f16 as u128, u128::MIN)]
#[case::u128_max(u128::MAX as f16 as u128, u128::MAX)]
#[case::i128_min(i128::MIN as f16 as i128, i128::MIN)]
#[case::i128_max(i128::MAX as f16 as i128, i128::MAX)]
fn platform<T: PartialEq + Debug>(#[case] from: T, #[case] into: T) {
    assert_eq!(from, into);
}
