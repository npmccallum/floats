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
