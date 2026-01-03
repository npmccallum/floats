#![cfg(all(feature = "asm", target_arch = "x86_64"))]
#![cfg(feature = "casting")]

#[cfg(target_feature = "avx512fp16")]
mod avx;

#[cfg(all(not(target_feature = "avx512fp16"), target_feature = "f16c"))]
mod f16c;
