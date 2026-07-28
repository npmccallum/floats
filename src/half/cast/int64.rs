use crate::f16;
use casting::CastFrom;

#[cfg(all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"))]
impl CastFrom<f16> for i64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i64 {
        let result: i64;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "fmov {tmp:h}, {input:w}",    // Move u16 into a vector register as f16
                "fcvtzs {output:x}, {tmp:h}", // Convert f16 to i64 (round toward zero)
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

#[cfg(all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"))]
impl CastFrom<i64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: i64) -> f16 {
        let result: u16;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "scvtf {tmp:h}, {input:x}",  // Convert i64 to f16
                "fmov {output:w}, {tmp:h}",  // Move f16 to a GPR (u16 in low bits)
                input = in(reg) value,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        f16(result)
    }
}

#[cfg(all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"))]
impl CastFrom<f16> for u64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> u64 {
        let result: u64;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "fmov {tmp:h}, {input:w}",    // Move u16 into a vector register as f16
                "fcvtzu {output:x}, {tmp:h}", // Convert f16 to u64 (round toward zero)
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

#[cfg(all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"))]
impl CastFrom<u64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: u64) -> f16 {
        let result: u16;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "ucvtf {tmp:h}, {input:x}",  // Convert u64 to f16
                "fmov {output:w}, {tmp:h}",  // Move f16 to a GPR (u16 in low bits)
                input = in(reg) value,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        f16(result)
    }
}

// AVX-512 FP16: f16 <-> i64/u64 conversions (available since Sapphire Rapids, 2023)
//
// See `fl32.rs` for why these use `asm!` with compiler-allocated registers.

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16"))]
impl CastFrom<f16> for i64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i64 {
        let result: i64;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvttsh2si {out:r}, {tmp}",        // Convert f16 to i64 (truncate)
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        if value.is_nan() {
            0
        } else if value.is_infinite() {
            if value.is_sign_negative() {
                i64::MIN
            } else {
                i64::MAX
            }
        } else {
            result
        }
    }
}

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16"))]
impl CastFrom<i64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: i64) -> f16 {
        let result: u32;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                // `vcvtsi2sh` merges the upper bits of its destination from its
                // first source operand, which is `tmp` itself; zeroing breaks the
                // dependency on whatever last used the register.
                "vpxor {tmp}, {tmp}, {tmp}",
                "vcvtsi2sh {tmp}, {tmp}, {input:r}", // Convert i64 to scalar f16
                "vmovd {out:e}, {tmp}",              // Move f16 bits to a GPR
                input = in(reg) value,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        f16(result as u16)
    }
}

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16"))]
impl CastFrom<f16> for u64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> u64 {
        let result: u64;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvttsh2usi {out:r}, {tmp}",       // Convert f16 to u64 (truncate)
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        if value.is_nan() || value.is_sign_negative() {
            0
        } else if value.is_infinite() {
            u64::MAX
        } else {
            result
        }
    }
}

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16"))]
impl CastFrom<u64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: u64) -> f16 {
        let result: u32;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                // See the `i64` impl above for why `tmp` is zeroed first.
                "vpxor {tmp}, {tmp}, {tmp}",
                "vcvtusi2sh {tmp}, {tmp}, {input:r}", // Convert u64 to scalar f16
                "vmovd {out:e}, {tmp}",               // Move f16 bits to a GPR
                input = in(reg) value,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        f16(result as u16)
    }
}

#[cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]
impl CastFrom<u64> for f16 {
    #[inline]
    fn cast_from(value: u64) -> f16 {
        f16::cast_from(value as f32)
    }
}

#[cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]
impl CastFrom<f16> for u64 {
    #[inline]
    fn cast_from(value: f16) -> u64 {
        f32::cast_from(value).clamp(u64::MIN as f32, u64::MAX as f32) as u64
    }
}

#[cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]
impl CastFrom<i64> for f16 {
    #[inline]
    fn cast_from(value: i64) -> f16 {
        f16::cast_from(value as f32)
    }
}

#[cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]
impl CastFrom<f16> for i64 {
    #[inline]
    fn cast_from(value: f16) -> i64 {
        f32::cast_from(value).clamp(i64::MIN as f32, i64::MAX as f32) as i64
    }
}
