use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for f64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f64 {
        let result: f64;

        unsafe {
            core::arch::asm!(
                "fmov h0, w0",      // Move u16 from w0 to h0
                "fcvt d0, h0",      // Convert f16 in h0 to f64 in d0
                in("w0") value.0,
                out("d0") result,
                options(pure, nomem, nostack)
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
                "fcvt h0, d0",      // Convert f64 in d0 to f16 in h0
                "fmov w0, h0",      // Move f16 from h0 to w0 (u16 in low bits)
                in("d0") value,
                out("w0") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
