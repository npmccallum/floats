use super::super::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> i32/u32 conversions (available since Sapphire Rapids, 2023)

impl CastFrom<f16> for i32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i32 {
        let result: i32;

        unsafe {
            core::arch::asm!(
                "vmovd xmm0, eax",          // Move u16 to xmm0
                "vcvtsh2si eax, xmm0",      // Convert scalar f16 to i32
                in("eax") value.0 as u32,
                out("eax") result,
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
                "vcvtsi2sh xmm0, xmm0, eax",   // Convert i32 to scalar f16
                "vmovd eax, xmm0",             // Move result to eax
                in("eax") value,
                out("eax") result,
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
                "vmovd xmm0, eax",          // Move u16 to xmm0
                "vcvtsh2usi eax, xmm0",     // Convert scalar f16 to u32
                in("eax") value.0 as u32,
                out("eax") result,
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
                "vcvtusi2sh xmm0, xmm0, eax",  // Convert u32 to scalar f16
                "vmovd eax, xmm0",             // Move result to eax
                in("eax") value,
                out("eax") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
