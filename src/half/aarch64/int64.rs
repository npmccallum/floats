use super::super::f16;
use casting::CastFrom;

impl CastFrom<f16> for i64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i64 {
        let result: i64;

        unsafe {
            core::arch::asm!(
                "fmov h0, w1",      // Move u16 from w1 to h0
                "fcvtzs x0, h0",    // Convert f16 in h0 to i64 in x0 (round toward zero)
                in("w1") value.0,
                out("x0") result,
                options(pure, nomem, nostack)
            );
        }

        result
    }
}

impl CastFrom<i64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: i64) -> f16 {
        let result: u16;

        unsafe {
            core::arch::asm!(
                "scvtf h0, x0",     // Convert i64 in x0 to f16 in h0
                "fmov w1, h0",      // Move f16 from h0 to w1 (u16 in low bits)
                in("x0") value,
                out("w1") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}

impl CastFrom<f16> for u64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> u64 {
        let result: u64;

        unsafe {
            core::arch::asm!(
                "fmov h0, w1",      // Move u16 from w1 to h0
                "fcvtzu x0, h0",    // Convert f16 in h0 to u64 in x0 (round toward zero)
                in("w1") value.0,
                out("x0") result,
                options(pure, nomem, nostack)
            );
        }

        result
    }
}

impl CastFrom<u64> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: u64) -> f16 {
        let result: u16;

        unsafe {
            core::arch::asm!(
                "ucvtf h0, x0",     // Convert u64 in x0 to f16 in h0
                "fmov w1, h0",      // Move f16 from h0 to w1 (u16 in low bits)
                in("x0") value,
                out("w1") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
