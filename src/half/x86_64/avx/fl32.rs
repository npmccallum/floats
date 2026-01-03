use super::super::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> f32 conversions (available since Sapphire Rapids, 2023)

impl CastFrom<f16> for f32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f32 {
        let result: f32;

        unsafe {
            core::arch::asm!(
                "vmovd xmm0, eax",          // Move u16 to xmm0
                "vcvtsh2ss xmm0, xmm0, xmm0",  // Convert scalar f16 to f32
                "vmovss eax, xmm0",         // Move result to eax
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
                "vcvtss2sh xmm0, xmm0, xmm1",  // Convert scalar f32 to f16
                "vmovd eax, xmm0",             // Move result to eax
                in("xmm1") value,
                out("eax") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
