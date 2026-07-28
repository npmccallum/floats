use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for i32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i32 {
        let result: i32;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "fmov {tmp:h}, {input:w}",    // Move u16 into a vector register as f16
                "fcvtzs {output:w}, {tmp:h}", // Convert f16 to i32 (round toward zero)
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

impl CastFrom<i32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: i32) -> f16 {
        let result: u16;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "scvtf {tmp:h}, {input:w}",  // Convert i32 to f16
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

impl CastFrom<f16> for u32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> u32 {
        let result: u32;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "fmov {tmp:h}, {input:w}",    // Move u16 into a vector register as f16
                "fcvtzu {output:w}, {tmp:h}", // Convert f16 to u32 (round toward zero)
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

impl CastFrom<u32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: u32) -> f16 {
        let result: u16;

        // SAFETY: the module is gated on `target_feature = "fp16"`, so the fp16
        // conversion instructions are available. Every operand is a compiler-
        // allocated slot -- including the `vreg` scratch -- so the block declares
        // every register it writes, and it touches neither memory nor the stack,
        // matching `nomem` and `nostack`.
        unsafe {
            core::arch::asm!(
                "ucvtf {tmp:h}, {input:w}",  // Convert u32 to f16
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
