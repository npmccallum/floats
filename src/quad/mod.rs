use core::ops::Neg;

mod cast;

/// 128-bit floating point type (IEEE 754 quadruple-precision).
///
/// This provides a minimal implementation with construction methods and
/// conversions through f64.
#[derive(Clone, Copy, Default)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f128(pub(crate) u128);

impl core::fmt::Debug for f128 {
    #[inline]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{:#034x}", self.0)
    }
}

impl Neg for f128 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0 ^ (1 << 127))
    }
}

impl PartialEq for f128 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // IEEE 754 compliance:
        // 1. NaN != NaN (including all NaN bit patterns)
        // 2. +0.0 == -0.0

        // Check for NaN using proper IEEE 754 detection:
        // exponent = 0x7FFF (all 1s) AND mantissa != 0
        let self_exp = ((self.0 >> 112) & 0x7FFF) as u16;
        let self_mantissa = self.0 & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
        let other_exp = ((other.0 >> 112) & 0x7FFF) as u16;
        let other_mantissa = other.0 & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

        let self_is_nan = self_exp == 0x7FFF && self_mantissa != 0;
        let other_is_nan = other_exp == 0x7FFF && other_mantissa != 0;

        if self_is_nan || other_is_nan {
            return false;
        }

        // Handle +0.0 == -0.0: both have exponent=0 and mantissa=0
        let self_is_zero = (self.0 & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) == 0;
        let other_is_zero = (other.0 & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) == 0;

        if self_is_zero && other_is_zero {
            return true;
        }

        // Normal comparison
        self.0 == other.0
    }
}

impl f128 {
    pub const RADIX: u32 = 2;

    pub const MANTISSA_DIGITS: u32 = 113;
    pub const DIGITS: u32 = 33;

    pub const EPSILON: Self = Self(0x3f8f0000000000000000000000000000);

    pub const MIN: Self = Self(0xfffeffffffffffffffffffffffffffff);
    pub const MIN_POSITIVE: Self = Self(0x00010000000000000000000000000000);
    pub const MAX: Self = Self(0x7ffeffffffffffffffffffffffffffff);

    pub const MIN_EXP: i32 = -16381;
    pub const MAX_EXP: i32 = 16384;
    pub const MIN_10_EXP: i32 = -4931;
    pub const MAX_10_EXP: i32 = 4932;

    pub const NAN: Self = Self(0x7fff8000000000000000000000000000);
    pub const INFINITY: Self = Self(0x7fff0000000000000000000000000000);
    pub const NEG_INFINITY: Self = Self(0xffff0000000000000000000000000000);

    #[inline]
    pub const fn is_nan(self) -> bool {
        (self.0 & 0x7fff0000000000000000000000000000) == 0x7fff0000000000000000000000000000
            && (self.0 & 0x0000ffffffffffffffffffffffffffff) != 0
    }

    #[inline]
    pub const fn is_infinite(self) -> bool {
        (self.0 & 0x7fffffffffffffffffffffffffffffff) == Self::INFINITY.0
    }

    #[inline]
    pub const fn is_finite(self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }

    #[inline]
    pub const fn is_sign_positive(self) -> bool {
        (self.0 & (1 << 127)) == 0
    }

    #[inline]
    pub const fn is_sign_negative(self) -> bool {
        (self.0 & (1 << 127)) != 0
    }

    /// Creates an `f128` from its representation as a `u128`.
    #[inline]
    pub const fn from_bits(bits: u128) -> Self {
        Self(bits)
    }

    /// Creates an `f128` from its representation as a byte array in big endian.
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }

    /// Creates an `f128` from its representation as a byte array in little endian.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_le_bytes(bytes))
    }

    /// Creates an `f128` from its representation as a byte array in native endian.
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_ne_bytes(bytes))
    }

    /// Returns the raw representation of this `f128` as a `u128`.
    #[inline]
    pub const fn to_bits(self) -> u128 {
        self.0
    }

    /// Returns the memory representation of this `f128` as a byte array in big endian.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Returns the memory representation of this `f128` as a byte array in little endian.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    /// Returns the memory representation of this `f128` as a byte array in native endian.
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; 16] {
        self.0.to_ne_bytes()
    }
}
