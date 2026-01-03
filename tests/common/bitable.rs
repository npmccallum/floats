use floats::{f128 as F128, f16 as F16};

// Helper trait for converting to/from bits generically
pub trait Bitable: Copy {
    type Bits;

    fn to_bits(self) -> Self::Bits;
    fn from_bits(bits: Self::Bits) -> Self;
}

impl Bitable for f32 {
    type Bits = u32;

    fn to_bits(self) -> u32 {
        f32::to_bits(self)
    }
    fn from_bits(bits: u32) -> Self {
        f32::from_bits(bits)
    }
}

impl Bitable for f16 {
    type Bits = u16;

    fn to_bits(self) -> u16 {
        f16::to_bits(self)
    }
    fn from_bits(bits: u16) -> Self {
        f16::from_bits(bits)
    }
}

impl Bitable for F16 {
    type Bits = u16;

    fn to_bits(self) -> u16 {
        F16::to_bits(self)
    }
    fn from_bits(bits: u16) -> Self {
        F16::from_bits(bits)
    }
}

impl Bitable for f64 {
    type Bits = u64;

    fn to_bits(self) -> u64 {
        f64::to_bits(self)
    }
    fn from_bits(bits: u64) -> Self {
        f64::from_bits(bits)
    }
}

impl Bitable for f128 {
    type Bits = u128;

    fn to_bits(self) -> u128 {
        f128::to_bits(self)
    }
    fn from_bits(bits: u128) -> Self {
        f128::from_bits(bits)
    }
}

impl Bitable for F128 {
    type Bits = u128;

    fn to_bits(self) -> u128 {
        F128::to_bits(self)
    }
    fn from_bits(bits: u128) -> Self {
        F128::from_bits(bits)
    }
}

macro_rules! impl_bitable_int {
    ($($t:ty),*) => {
        $(
            impl Bitable for $t {
                type Bits = $t;

                fn to_bits(self) -> $t {
                    self
                }
                fn from_bits(bits: $t) -> Self {
                    bits
                }
            }
        )*
    };
}

impl_bitable_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
