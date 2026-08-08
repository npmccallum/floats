//! Conversions that `compat.rs` cannot check, because nightly cannot currently
//! perform them correctly.
//!
//! The crate's contract is that every observable matches nightly, and
//! `compat.rs` checks exactly that. Where nightly is broken there is nothing to
//! compare against, so these cases assert written-out values instead. That is
//! weaker -- it checks this crate against a hand-derived answer rather than
//! against the reference -- so each group carries a TODO to move it back to
//! `compat.rs` when the upstream fix lands.

#![cfg(all(feature = "casting", not(feature = "nightly")))]
#![feature(f16)]
// The crate's MSRV does not constrain this file: it is a nightly-only test
// (see `feature(f16)` above), so `black_box`'s 1.66 stabilization is moot.
#![allow(clippy::incompatible_msrv)]

use casting::CastFrom;
use floats::f16 as F16;
use std::hint::black_box;

#[rstest::rstest]
// `u128`/`i128` -> `f16` needs compiler-rt's `__floattihf` and
// `__floatuntihf`, which do not exist -- the binary fails to link.
// Tracked upstream by rust-lang/compiler-builtins#729 (open, stalled).
// TODO: move back to `compat.rs` once those builtins ship.
#[case::u128_0u128_f16(F16::cast_from(black_box(0u128)).to_bits(), 0.0f16.to_bits())]
#[case::u128_3u128_f16(F16::cast_from(black_box(3u128)).to_bits(), 3.0f16.to_bits())]
#[case::u128_1u128sh90_f16(F16::cast_from(black_box(1u128 << 90)).to_bits(), f16::INFINITY.to_bits())]
#[case::u128_1u128sh113_f16(F16::cast_from(black_box(1u128 << 113)).to_bits(), f16::INFINITY.to_bits())]
#[case::u128_1u128sh113p1_f16(F16::cast_from(black_box((1u128 << 113) + 1)).to_bits(), f16::INFINITY.to_bits())]
#[case::u128_u128_max_f16(F16::cast_from(black_box(u128::MAX)).to_bits(), f16::INFINITY.to_bits())]
#[case::i128_0i128_f16(F16::cast_from(black_box(0i128)).to_bits(), 0.0f16.to_bits())]
#[case::i128_3i128_f16(F16::cast_from(black_box(3i128)).to_bits(), 3.0f16.to_bits())]
#[case::i128_neg3i128_f16(F16::cast_from(black_box(-3i128)).to_bits(), (-3.0f16).to_bits())]
#[case::i128_1i128sh100_f16(F16::cast_from(black_box(1i128 << 100)).to_bits(), f16::INFINITY.to_bits())]
#[case::i128_i128_min_f16(F16::cast_from(black_box(i128::MIN)).to_bits(), f16::NEG_INFINITY.to_bits())]
#[case::i128_i128_max_f16(F16::cast_from(black_box(i128::MAX)).to_bits(), f16::INFINITY.to_bits())]
// LLVM miscompiled the saturating `f16` -> `i16` cast under
// `avx512fp16`, yielding `i16::MIN` for NaN instead of 0. Fixed in
// llvm/llvm-project e1823e8b (2026-07-31), not yet in nightly's LLVM
// as of 2026-08-08. Only NaN was ever affected.
// TODO: move back to `compat.rs` once the fix reaches nightly.
#[case::f16_qnan_i16(i16::cast_from(black_box(F16::from_bits(0x7E00))), 0)]
// With optimization off, nightly's `f16` -> 128-bit lowering saturates
// at 64 bits, so no single value is correct at every opt-level.
// TODO: move back to `compat.rs` once nightly lowers these correctly
// at `opt-level = 0`.
#[case::f16_neg_one_i128(i128::cast_from(black_box(F16::from_bits(0xBC00))), -1)]
#[case::f16_inf_i128(i128::cast_from(black_box(F16::from_bits(0x7C00))), i128::MAX)]
#[case::f16_inf_u128(u128::cast_from(black_box(F16::from_bits(0x7C00))), u128::MAX)]
#[case::f16_neg_inf_i128(i128::cast_from(black_box(F16::from_bits(0xFC00))), i128::MIN)]
fn value_matches_expected<T: PartialEq + core::fmt::Debug>(#[case] ours: T, #[case] expected: T) {
    assert_eq!(ours, expected);
}
