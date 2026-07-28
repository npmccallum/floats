use crate::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> f32 conversions (available since Sapphire Rapids, 2023)
//
// The `core::arch` avx512fp16 intrinsics are still unstable, so these use
// `asm!`. Registers are compiler-allocated: hardcoding `xmm0`/`eax` and
// omitting the scratch register from the operand list lets the register
// allocator keep a live value in a register the block overwrites.

impl CastFrom<f16> for f32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f32 {
        let result: f32;

        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvtsh2ss {out}, {tmp}, {tmp}",    // Convert scalar f16 to f32
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(xmm_reg) result,
                options(nomem, nostack)
            );
        }

        result
    }
}

impl CastFrom<f32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f32) -> f16 {
        let result: u32;

        unsafe {
            core::arch::asm!(
                // `vcvtss2sh` writes only the low 16 bits of its destination and
                // merges the upper bits from its first source operand, so both
                // sources are `input`: `tmp` is never read before it is written.
                "vcvtss2sh {tmp}, {input}, {input}", // Convert scalar f32 to f16
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
