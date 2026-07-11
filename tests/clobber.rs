//! Regression tests for inline-assembly register clobbers.
//!
//! The hardware conversion paths (`src/half/aarch64`, `src/half/x86_64`) use
//! inline assembly. If such a block writes a register that is not declared as
//! an output or a clobber, the compiler may keep a caller's live value in that
//! register across the block and read back garbage afterwards.
//!
//! Conversions exercised in isolation cannot observe this: the corruption is
//! only visible when the *caller* keeps a value live across the conversion.
//! Each test below therefore uses a value both before and after a cast, with
//! `black_box` to keep the value genuinely live (otherwise constant folding
//! computes the result at compile time and the clobber goes unnoticed).

#![cfg(feature = "casting")]

use std::hint::black_box;

use casting::CastFrom;
use floats::f16;

#[inline(never)]
fn narrow_f32_keep_input(x: f32) -> (u16, f32) {
    let h = f16::cast_from(x);
    (h.to_bits(), x * 2.0)
}

#[inline(never)]
fn narrow_f64_keep_input(x: f64) -> (u16, f64) {
    let h = f16::cast_from(x);
    (h.to_bits(), x * 2.0)
}

#[inline(never)]
fn widen_keep_accumulator(h: f16, acc: f32) -> (f32, f32) {
    let widened = f32::cast_from(h);
    (widened, acc * 2.0)
}

#[test]
fn f32_to_f16_preserves_caller_value() {
    let (bits, doubled) = narrow_f32_keep_input(black_box(1.5f32));
    assert_eq!(bits, 0x3e00);
    assert_eq!(doubled, 3.0);
}

#[test]
fn f64_to_f16_preserves_caller_value() {
    let (bits, doubled) = narrow_f64_keep_input(black_box(1.5f64));
    assert_eq!(bits, 0x3e00);
    assert_eq!(doubled, 3.0);
}

#[test]
fn f16_to_f32_preserves_caller_value() {
    let h = f16::cast_from(black_box(2.0f32));
    let (widened, doubled) = widen_keep_accumulator(black_box(h), black_box(4.0f32));
    assert_eq!(widened, 2.0);
    assert_eq!(doubled, 8.0);
}

/// Mixes float and integer conversions in one loop, keeping a float
/// accumulator live across the integer casts. The integer conversion blocks
/// declare only general-purpose registers, so a vector-register clobber shows
/// up here as a corrupted `acc`.
#[test]
fn mixed_int_and_float_casts_preserve_accumulator() {
    let halves: Vec<f16> = (0..8)
        .map(|i| f16::cast_from(black_box(i as f32)))
        .collect();

    let mut acc = 0.0f32;
    let mut isum = 0i64;
    for &h in black_box(&halves) {
        acc += f32::cast_from(h) * black_box(1.0f32);
        isum += i64::cast_from(h);
    }

    assert_eq!(
        acc, 28.0,
        "float accumulator corrupted across integer casts"
    );
    assert_eq!(isum, 28);
}

/// Same, for the 32-bit integer conversion blocks.
#[test]
fn i32_casts_preserve_float_accumulator() {
    let mut acc = 0.0f32;
    let mut isum = 0i32;
    for i in 0..8i32 {
        let h = f16::cast_from(black_box(i));
        acc += black_box(1.5f32);
        isum += i32::cast_from(h);
    }

    assert_eq!(acc, 12.0, "float accumulator corrupted across i32 casts");
    assert_eq!(isum, 28);
}

/// Unsigned conversions use `ucvtf`/`fcvtzu` but the same registers.
#[test]
fn u64_casts_preserve_float_accumulator() {
    let mut acc = 1.0f64;
    let mut usum = 0u64;
    for i in 0..8u64 {
        let h = f16::cast_from(black_box(i));
        acc *= black_box(2.0f64);
        usum += u64::cast_from(h);
    }

    assert_eq!(acc, 256.0, "float accumulator corrupted across u64 casts");
    assert_eq!(usum, 28);
}
