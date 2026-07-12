use crate::f16;
use casting::CastFrom;

// AVX-512 FP16: f16 <-> i64/u64 conversions (available since Sapphire Rapids, 2023)
//
// See `fl32.rs` for why these use `asm!` with compiler-allocated registers.

impl CastFrom<f16> for i64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> i64 {
        if value.is_nan() {
            return 0;
        }
        if value.is_infinite() {
            return if value.is_sign_negative() {
                i64::MIN
            } else {
                i64::MAX
            };
        }

        let result: i64;

        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvttsh2si {out:r}, {tmp}",        // Convert f16 to i64 (truncate)
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
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
        let result: u32;

        unsafe {
            core::arch::asm!(
                "vcvtsi2sh {tmp}, {tmp}, {input:r}", // Convert i64 to scalar f16
                "vmovd {out:e}, {tmp}",              // Move f16 bits to a GPR
                input = in(reg) value,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(pure, nomem, nostack)
            );
        }

        f16(result as u16)
    }
}

impl CastFrom<f16> for u64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> u64 {
        if value.is_nan() || value.is_sign_negative() {
            return 0;
        }
        if value.is_infinite() {
            return u64::MAX;
        }

        let result: u64;

        unsafe {
            core::arch::asm!(
                "vmovd {tmp}, {input:e}",           // Move u16 into a vector register
                "vcvttsh2usi {out:r}, {tmp}",       // Convert f16 to u64 (truncate)
                input = in(reg) value.0 as u32,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
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
        let result: u32;

        unsafe {
            core::arch::asm!(
                "vcvtusi2sh {tmp}, {tmp}, {input:r}", // Convert u64 to scalar f16
                "vmovd {out:e}, {tmp}",               // Move f16 bits to a GPR
                input = in(reg) value,
                tmp = out(xmm_reg) _,
                out = lateout(reg) result,
                options(pure, nomem, nostack)
            );
        }

        f16(result as u16)
    }
}
