use super::super::f16;
use casting::CastFrom;

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
