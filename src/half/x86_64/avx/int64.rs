use super::super::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> i64/u64 conversions (available since Sapphire Rapids, 2023)

impl CastFrom<f16> for i64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i64 {
        let result: i64;

        unsafe {
            core::arch::asm!(
                "vmovd xmm0, eax",          // Move u16 to xmm0
                "vcvtsh2si rax, xmm0",      // Convert scalar f16 to i64
                in("eax") value.0 as u32,
                out("rax") result,
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
                "vcvtsi2sh xmm0, xmm0, rax",   // Convert i64 to scalar f16
                "vmovd eax, xmm0",             // Move result to eax
                in("rax") value,
                out("eax") result,
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
                "vmovd xmm0, eax",          // Move u16 to xmm0
                "vcvtsh2usi rax, xmm0",     // Convert scalar f16 to u64
                in("eax") value.0 as u32,
                out("rax") result,
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
                "vcvtusi2sh xmm0, xmm0, rax",  // Convert u64 to scalar f16
                "vmovd eax, xmm0",             // Move result to eax
                in("rax") value,
                out("eax") result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
