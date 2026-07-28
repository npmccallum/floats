use crate::f16;
use casting::CastFrom;

#[cfg(all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"))]
impl CastFrom<f16> for f64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f64 {
        let result: f64;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "fmov {tmp:h}, {input:w}",  // Move u16 into a vector register as f16
                "fcvt {output:d}, {tmp:h}", // Convert f16 to f64
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(vreg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

#[cfg(all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"))]
impl CastFrom<f64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f64) -> f16 {
        let result: u16;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "fcvt {tmp:h}, {input:d}",  // Convert f64 to f16
                "fmov {output:w}, {tmp:h}", // Move f16 to a GPR (u16 in low bits)
                input = in(vreg) value,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        f16(result)
    }
}

// AVX-512 FP16: f16 <-> f64 conversions (available since Sapphire Rapids, 2023)
//
// See `fl32.rs` for why these use `asm!` with compiler-allocated registers.

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16"))]
impl CastFrom<f16> for f64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f64 {
        let result: f64;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvtsh2sd {out}, {tmp}, {tmp}",    // Convert scalar f16 to f64
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(xmm_reg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

#[cfg(all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16"))]
impl CastFrom<f64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f64) -> f16 {
        let result: u32;

        // SAFETY: the module is gated on `target_feature = "avx512fp16"`, so the
        // fp16 conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `xmm_reg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                // See the `f32` impl for why both sources are `input`.
                "vcvtsd2sh {tmp}, {input}, {input}", // Convert scalar f64 to f16
                "vmovd {out:e}, {tmp}",              // Move f16 bits to a GPR
                input = in(xmm_reg) value,
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
impl CastFrom<f16> for f64 {
    #[inline]
    fn cast_from(value: f16) -> f64 {
        f32::cast_from(value) as f64
    }
}

#[cfg(not(any(
    all(feature = "asm", target_arch = "aarch64", target_feature = "fp16"),
    all(feature = "asm", target_arch = "x86_64", target_feature = "avx512fp16")
)))]
impl CastFrom<f64> for f16 {
    #[inline]
    fn cast_from(value: f64) -> f16 {
        let bits = value.to_bits();
        let sign = ((bits >> 48) & 0x8000) as u16;
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let mant = bits & 0x000f_ffff_ffff_ffff;

        if exp == 0 {
            return f16(sign);
        }

        if exp == 0x7ff {
            if mant == 0 {
                return f16(sign | 0x7c00);
            }

            // Preserve the high payload bits, as nightly does. The quiet bit is
            // forced on so that a payload living entirely in the truncated low
            // bits cannot turn the NaN into an infinity.
            let payload = (mant >> 42) as u16 & 0x03FF;
            return f16(sign | 0x7c00 | 0x0200 | payload);
        }

        let f16_exp = exp - 1008;
        if f16_exp > 30 {
            return f16(sign | 0x7c00);
        }

        let significand = mant | (1u64 << 52);
        if f16_exp <= 0 {
            let shift = (43 - f16_exp) as u32;
            if shift > 53 {
                return f16(sign);
            }
            let truncated = significand >> shift;
            let remainder = significand & ((1u64 << shift) - 1);
            let halfway = 1u64 << (shift - 1);
            let rounded = truncated
                + u64::from(remainder > halfway || (remainder == halfway && truncated & 1 != 0));
            return f16(sign | rounded as u16);
        }

        let truncated = significand >> 42;
        let remainder = significand & ((1u64 << 42) - 1);
        let halfway = 1u64 << 41;
        let rounded = truncated
            + u64::from(remainder > halfway || (remainder == halfway && truncated & 1 != 0));

        if rounded >= 0x800 {
            if f16_exp >= 30 {
                f16(sign | 0x7c00)
            } else {
                f16(sign | (((f16_exp + 1) as u16) << 10))
            }
        } else {
            f16(sign | ((f16_exp as u16) << 10) | (rounded as u16 & 0x03ff))
        }
    }
}
