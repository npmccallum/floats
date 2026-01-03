use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for f32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f32 {
        let result: f32;

        unsafe {
            core::arch::asm!(
                "fmov h0, w0",      // Move u16 from w0 to h0
                "fcvt s0, h0",      // Convert f16 in h0 to f32 in s0
                in("w0") value.0,
                out("s0") result,
                options(pure, nomem, nostack)
            );
        }

        result
    }
}

impl CastFrom<f32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f32) -> f16 {
        let result: u16;

        unsafe {
            core::arch::asm!(
                "fcvt h0, s0",      // Convert f32 in s0 to f16 in h0
                "fmov w0, h0",      // Move f16 from h0 to w0 (u16 in low bits)
                in("s0") value,
                out("w0") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
