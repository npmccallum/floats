use crate::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> f64 conversions (available since Sapphire Rapids, 2023)
//
// See `fl32.rs` for why these use `asm!` with compiler-allocated registers.

impl CastFrom<f16> for f64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f64 {
        let result: f64;

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

impl CastFrom<f64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f64) -> f16 {
        let result: u32;

        unsafe {
            core::arch::asm!(
                "vcvtsd2sh {tmp}, {tmp}, {input}",  // Convert scalar f64 to f16
                "vmovd {out:e}, {tmp}",             // Move f16 bits to a GPR
                input = in(xmm_reg) value,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(nomem, nostack)
            );
        }

        f16(result as u16)
    }
}
