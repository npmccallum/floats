# floats

[![CI](https://github.com/npmccallum/floats/workflows/Test/badge.svg)](https://github.com/npmccallum/floats/actions)
[![Crates.io](https://img.shields.io/crates/v/floats.svg)](https://crates.io/crates/floats)
[![Documentation](https://docs.rs/floats/badge.svg)](https://docs.rs/floats)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/npmccallum/floats/blob/main/LICENSE)

f16 and f128 floating point types for compatibility with future Rust versions.

The goal of this crate is to provide some minimal `f16` and `f128` functionality
on stable Rust today with a smooth transition to the currently unstable `f16`
and `f128` types in the Rust standard library.

This crate provides:

1. A custom `f16` type for half-precision floating point numbers
2. A custom `f128` type for quadruple-precision floating point numbers

You can load and store the `f16` and `f128` types to and from bits/bytes and use
the `CastFrom`/`CastInto` traits from the [`casting`](https://docs.rs/casting)
crate to cast them to fully supported Rust floating point types where you can do
arithmetic operations. However, no arithmetic operations are yet supported on
the custom `f16` and `f128` types.

## Installation

```toml
[dependencies]
floats = { version = "0.1", features = ["casting"] }
casting = "0.1.1"
```

## Quick Start

```rust
#![cfg_attr(feature = "nightly", feature(f16, f128))]

use floats::{f16, f128};

// Create f16 from bits
let half = f16::from_bits(0x3C00); // 1.0 in f16
assert_eq!(half.to_bits(), 0x3C00);

#[cfg(feature = "casting")]
{
  use casting::CastInto;

  // Cast to f32 for arithmetic
  let float: f32 = half.cast_into();
  assert_eq!(float, 1.0f32);

  // Create f128 from f64
  let quad: f128 = 3.141592653589793f64.cast_into();
  let back: f64 = quad.cast_into();
  assert_eq!(back, std::f64::consts::PI);
}
```

## Types

- `f16`: 16-bit half-precision float (IEEE 754)
- `f128`: 128-bit quadruple-precision float (IEEE 754)

## Feature Flags

- `asm` (default): Use hardware-accelerated inline assembly for f16 conversions
  when available (aarch64 fp16, x86_64 f16c). Disable for testing or
  compatibility.
- `casting` (default): Enable the optional `casting` dependency for
  `CastFrom`/`CastInto` trait implementations between `f16`/`f128` and other
  numeric types.
- `nightly`: Disable all crate code and simply re-export the nightly
  `f16`/`f128` types. This makes it trivial to support either our custom types
  or the nightly types without having to resort to complex dependency
  management.

## Upgrade Path

Upgrading to the standard library `f16` and `f128` types should be
API-compatible. Use the [`casting`](https://docs.rs/casting) crate's `CastFrom`
and `CastInto` traits for conversions.

## License

Licensed under the
[MIT License](https://github.com/npmccallum/floats/blob/main/LICENSE).
