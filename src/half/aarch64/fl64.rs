use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for f64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f64 {
        let result: f64;

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
