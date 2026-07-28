#![cfg(feature = "casting")]

// Common f128 bit layout constants used across all conversion modules
/// f128 infinity/NaN representation (exponent = 32767, mantissa = 0)
pub const F128_INF: u128 = 0x7FFF_0000_0000_0000_0000_0000_0000_0000;

/// f128 exponent bias
pub const F128_EXP_BIAS: i32 = 16383;

/// Masks
pub const F128_MANT_MASK: u128 = (1u128 << 112) - 1;
pub const F128_ABS_MASK: u128 = (1u128 << 127) - 1;

mod fl16;
mod fl32;
mod fl64;
mod int128;
mod int16;
mod int32;
mod int64;
mod int8;
