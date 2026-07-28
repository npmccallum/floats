use crate::f16;
use casting::CastFrom;

use core::arch::x86_64::{
    _mm_cvtph_ps, _mm_cvtps_ph, _mm_cvtss_f32, _mm_extract_epi16, _mm_set_ss, _mm_setr_epi16,
    _MM_FROUND_CUR_DIRECTION,
};

// F16C: f16 <-> f32 conversions (available since Ivy Bridge, 2011)
//
// These use the `core::arch` intrinsics rather than `asm!`. They emit the same
// `vcvtph2ps`/`vcvtps2ph` instructions, but the register allocator assigns the
// registers, so no operand can be silently clobbered.

impl CastFrom<f16> for f32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f32 {
        // SAFETY: this module is gated on `target_feature = "f16c"`.
        unsafe {
            let packed = _mm_setr_epi16(value.0 as i16, 0, 0, 0, 0, 0, 0, 0);
            _mm_cvtss_f32(_mm_cvtph_ps(packed))
        }
    }
}

impl CastFrom<f32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f32) -> f16 {
        // SAFETY: this module is gated on `target_feature = "f16c"`.
        unsafe {
            // This immediate defers to the dynamic rounding mode.
            let packed = _mm_cvtps_ph::<_MM_FROUND_CUR_DIRECTION>(_mm_set_ss(value));
            f16(_mm_extract_epi16::<0>(packed) as u16)
        }
    }
}
