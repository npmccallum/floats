use crate::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> i32/u32 conversions (available since Sapphire Rapids, 2023)
//
// See `fl32.rs` for why these use `asm!` with compiler-allocated registers.

impl CastFrom<f16> for i32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i32 {
        let result: i32;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvttsh2si {out:e}, {tmp}",        // Convert f16 to i32 (truncate)
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
                i32::MIN
            } else {
                i32::MAX
            }
        } else {
            result
        }
    }
}

impl CastFrom<i32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: i32) -> f16 {
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
                "vcvtsi2sh {tmp}, {tmp}, {input:e}", // Convert i32 to scalar f16
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

impl CastFrom<f16> for u32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> u32 {
        let result: u32;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvttsh2usi {out:e}, {tmp}",       // Convert f16 to u32 (truncate)
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        if value.is_nan() || value.is_sign_negative() {
            0
        } else if value.is_infinite() {
            u32::MAX
        } else {
            result
        }
    }
}

impl CastFrom<u32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: u32) -> f16 {
        let result: u32;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                // See the `i32` impl above for why `tmp` is zeroed first.
                "vpxor {tmp}, {tmp}, {tmp}",
                "vcvtusi2sh {tmp}, {tmp}, {input:e}", // Convert u32 to scalar f16
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
