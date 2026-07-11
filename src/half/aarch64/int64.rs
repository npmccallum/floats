use super::super::f16;
use casting::CastFrom;

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
                "fmov {tmp:h}, {input:w}",    // Move u16 into a vector register as f16
                "fcvtzs {output:x}, {tmp:h}", // Convert f16 to i64 (round toward zero)
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(reg) result,
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
                "scvtf {tmp:h}, {input:x}",  // Convert i64 to f16
                "fmov {output:w}, {tmp:h}",  // Move f16 to a GPR (u16 in low bits)
                input = in(reg) value,
                tmp = out(vreg) _,
                output = lateout(reg) result,
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
        if value.is_nan() || value.is_sign_negative() {
            return 0;
        }
        if value.is_infinite() {
            return u64::MAX;
        }

        let result: u64;

        unsafe {
            core::arch::asm!(
                "fmov {tmp:h}, {input:w}",    // Move u16 into a vector register as f16
                "fcvtzu {output:x}, {tmp:h}", // Convert f16 to u64 (round toward zero)
                input = in(reg) value.0,
                tmp = out(vreg) _,
                output = lateout(reg) result,
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
                "ucvtf {tmp:h}, {input:x}",  // Convert u64 to f16
                "fmov {output:w}, {tmp:h}",  // Move f16 to a GPR (u16 in low bits)
                input = in(reg) value,
                tmp = out(vreg) _,
                output = lateout(reg) result,
                options(pure, nomem, nostack)
            );
        }

        f16(result)
    }
}
