// Under the `nightly` feature `floats` re-exports the standard library's f16
// and f128, so `Custom` would map each std type to itself and every comparison
// against it would be vacuous. `bitable.rs` is gated the same way.
#![cfg(not(feature = "nightly"))]

use floats::{f128 as F128, f16 as F16};

// Helper trait to map std types to our custom types
pub trait Customized {
    type Custom;
}

impl Customized for f16 {
    type Custom = F16;
}

impl Customized for f128 {
    type Custom = F128;
}
