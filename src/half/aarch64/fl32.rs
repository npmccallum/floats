use super::super::f16;
use casting::CastFrom;

use core::arch::aarch64::{float16x4_t, float32x4_t, vcvt_f16_f32, vcvt_f32_f16, vgetq_lane_f32};

// f16 <-> f32 conversions.
//
// These use the `core::arch::aarch64` intrinsics rather than `asm!`. They are
// the four-lane NEON forms -- there is no stable scalar fp16 intrinsic -- but
// only lane 0 is populated, and LLVM folds the whole thing down to the same
// scalar `fcvt` the hand-written assembly emitted. Letting the register
// allocator assign the registers means there is no operand contract left to get
// wrong; the integer conversions in `int32.rs`/`int64.rs` still need `asm!`
// because no stable intrinsic covers them.

impl CastFrom<f16> for f32 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f16) -> f32 {
        // SAFETY: `neon` is baseline on aarch64, and `float16x4_t` has the same
        // layout as `[u16; 4]`.
        unsafe {
            let packed: float16x4_t = core::mem::transmute([value.0, 0u16, 0, 0]);
            let widened: float32x4_t = vcvt_f32_f16(packed);
            vgetq_lane_f32::<0>(widened)
        }
    }
}

impl CastFrom<f32> for f16 {
    #[inline]
    #[allow(unsafe_code)]
    fn cast_from(value: f32) -> f16 {
        // SAFETY: `neon` is baseline on aarch64, and `float16x4_t` has the same
        // layout as `[u16; 4]`.
        unsafe {
            let packed: float32x4_t = core::arch::aarch64::vsetq_lane_f32::<0>(
                value,
                core::arch::aarch64::vdupq_n_f32(0.0),
            );
            let narrowed: float16x4_t = vcvt_f16_f32(packed);
            let lanes: [u16; 4] = core::mem::transmute(narrowed);
            f16(lanes[0])
        }
    }
}
