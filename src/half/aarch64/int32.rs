use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for i32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i32 {
        let result: i32;

        unsafe {
            core::arch::asm!(
                "fmov h0, w1",      // Move u16 from w1 to h0
                "fcvtzs w0, h0",    // Convert f16 in h0 to i32 in w0 (round toward zero)
                in("w1") value.0,
                out("w0") result,
                options(pure, nomem, nostack)
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

        unsafe {
            core::arch::asm!(
                "scvtf h0, w0",     // Convert i32 in w0 to f16 in h0
                "fmov w1, h0",      // Move f16 from h0 to w1 (u16 in low bits)
                in("w0") value,
                out("w1") result,
                options(pure, nomem, nostack)
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

        unsafe {
            core::arch::asm!(
                "fmov h0, w1",      // Move u16 from w1 to h0
                "fcvtzu w0, h0",    // Convert f16 in h0 to u32 in w0 (round toward zero)
                in("w1") value.0,
                out("w0") result,
                options(pure, nomem, nostack)
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

        unsafe {
            core::arch::asm!(
                "ucvtf h0, w0",     // Convert u32 in w0 to f16 in h0
                "fmov w1, h0",      // Move f16 from h0 to w1 (u16 in low bits)
                in("w0") value,
                out("w1") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
