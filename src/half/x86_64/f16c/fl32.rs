use super::super::f16;
use casting::CastFrom;

// F16C: f16 <-> f32 conversions (available since Ivy Bridge, 2011)

impl CastFrom<f16> for f32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f32 {
        let result: f32;

        unsafe {
            core::arch::asm!(
                "vmovd xmm0, eax",          // Move u16 to xmm0
                "vcvtph2ps xmm0, xmm0",     // Convert f16 to f32
                "vmovss eax, xmm0",         // Move result to output
                in("eax") value.0 as u32,
                out("eax") result,
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
                "vcvtps2ph xmm1, xmm0, 0",  // Convert f32 to f16 with round-to-nearest
                "vmovd eax, xmm1",          // Move result to eax
                in("xmm0") value,
                out("eax") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
