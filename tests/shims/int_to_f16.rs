//! CI-only stand-ins for the `int -> f16` compiler builtins.
//!
//! LLVM emits `__floattihf`/`__floatuntihf` for `i128`/`u128 as f16`, but no
//! runtime provides them, so those casts fail to link. See
//! rust-lang/compiler-builtins#1261, which implements them upstream (open as
//! of 2026-08-08).
//!
//! Defining them here lets CI exercise the twelve 128-bit-to-`f16` cases in
//! `compat.rs`. The implementation below is deliberately *not* this crate's --
//! it is written from the IEEE 754 rounding rules directly, so the comparison
//! in `compat.rs` still contrasts two independent implementations rather than
//! checking one against itself.
//!
//! TODO: delete this file and the `--cfg ci_builtin_shims` in CI once
//! rust-lang/compiler-builtins#1261 reaches a nightly -- it must be merged and
//! then josh-synced into rust-lang/rust before it appears there.

/// Correctly-rounded `u128` -> `f16`, round-to-nearest-ties-even.
///
/// Every nonzero integer has an unbiased exponent >= 0, so no result is
/// subnormal and only the overflow-to-infinity edge needs handling.
fn u128_to_f16_bits(i: u128) -> u16 {
    if i == 0 {
        return 0;
    }

    let significant = 128 - i.leading_zeros();
    let mut exp = significant as i32 - 1;

    let mantissa = if significant <= 11 {
        // Exact: the value fits in the 11-bit significand.
        i << (11 - significant)
    } else {
        let dropped = significant - 11;
        let head = i >> dropped;
        let rest = i & ((1u128 << dropped) - 1);
        let halfway = 1u128 << (dropped - 1);

        // Ties to even.
        let mut m = head;
        if rest > halfway || (rest == halfway && head & 1 == 1) {
            m += 1;
        }
        if m == 1 << 11 {
            m >>= 1;
            exp += 1;
        }
        m
    };

    if exp > 15 {
        return 0x7C00; // infinity
    }

    (((exp + 15) as u16) << 10) | (mantissa as u16 & 0x03FF)
}

#[unsafe(no_mangle)]
pub extern "C" fn __floatuntihf(i: u128) -> f16 {
    f16::from_bits(u128_to_f16_bits(i))
}

#[unsafe(no_mangle)]
pub extern "C" fn __floattihf(i: i128) -> f16 {
    let sign = if i < 0 { 0x8000 } else { 0 };
    f16::from_bits(sign | u128_to_f16_bits(i.unsigned_abs()))
}
