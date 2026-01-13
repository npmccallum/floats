//! Benchmarks for f16/f128 cast operations.
//!
//! Compares our custom CastFrom/CastInto implementations against nightly std library.

#![cfg(feature = "casting")]
#![feature(f16, f128)]
#![allow(deprecated)]

// Workaround for missing compiler-rt symbols on macOS
// Double-casting strategy: go through f64 as an intermediate type
#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn __floattihf(a: i128) -> f16 {
    (a as f64) as f16
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn __floatuntihf(a: u128) -> f16 {
    (a as f64) as f16
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use casting::CastInto;
use floats::{f128 as F128, f16 as F16};

// ============================================================================
// Benchmark generation macros
// ============================================================================

/// Macro for benchmarking cast operations (generates both into and from)
macro_rules! bench_impl {
    (
        $into_fn:ident, $into_group:expr, $into_rep:ident,
        $from_fn:ident, $from_group:expr, $from_rep:ident,
        $src_std_ty:ty, $src_custom_ty:ty, $dst_ty:ty
    ) => {
        fn $into_fn(c: &mut Criterion) {
            let std_values = $into_rep;
            let custom_values: Vec<$src_custom_ty> = std_values
                .iter()
                .map(|&v| <$src_custom_ty>::from_bits(v.to_bits()))
                .collect();

            let mut group = c.benchmark_group($into_group);

            group.bench_function("custom", |b| {
                b.iter(|| {
                    for &v in &custom_values {
                        let result: $dst_ty = v.cast_into();
                        black_box(result);
                    }
                })
            });

            group.bench_function("std", |b| {
                b.iter(|| {
                    for &v in std_values {
                        black_box(v as $dst_ty);
                    }
                })
            });

            group.finish();
        }

        fn $from_fn(c: &mut Criterion) {
            let values = $from_rep;

            let mut group = c.benchmark_group($from_group);

            group.bench_function("custom", |b| {
                b.iter(|| {
                    for &v in values {
                        let result: $src_custom_ty = v.cast_into();
                        black_box(result);
                    }
                })
            });

            group.bench_function("std", |b| {
                b.iter(|| {
                    for &v in values {
                        black_box(v as $src_std_ty);
                    }
                })
            });

            group.finish();
        }
    };
}

// ============================================================================
// Test data generators
// ============================================================================

/// Representative f16 values for benchmarking
const F16_VALUES: &[f16] = &[
    // Special values
    0.0f16,
    -0.0f16,
    f16::INFINITY,
    f16::NEG_INFINITY,
    f16::NAN,
    // Boundary values
    f16::MIN,
    f16::MAX,
    f16::MIN_POSITIVE,
    f16::EPSILON,
    // Common values
    1.0f16,
    -1.0f16,
    0.5f16,
    -0.5f16,
    2.0f16,
    -2.0f16,
    10.0f16,
    -10.0f16,
    100.0f16,
    -100.0f16,
    1000.0f16,
    -1000.0f16,
    // Small values
    0.001f16,
    -0.001f16,
    0.0001f16,
    -0.0001f16,
];

/// Representative f32 values for f16 conversion benchmarks
const F32_VALUES: &[f32] = &[
    // Special values
    0.0f32,
    -0.0f32,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN,
    // Boundary values (f16 range)
    65504.0f32,      // f16::MAX
    -65504.0f32,     // f16::MIN
    6.1035156e-5f32, // f16::MIN_POSITIVE
    // Common values
    1.0f32,
    -1.0f32,
    0.5f32,
    -0.5f32,
    2.0f32,
    -2.0f32,
    10.0f32,
    -10.0f32,
    100.0f32,
    -100.0f32,
    1000.0f32,
    -1000.0f32,
    // Small values
    0.001f32,
    -0.001f32,
    0.0001f32,
    -0.0001f32,
];

/// Representative f64 values for f128 conversion benchmarks
const F64_VALUES: &[f64] = &[
    // Special values
    0.0f64,
    -0.0f64,
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::NAN,
    // Boundary values
    f64::MIN,
    f64::MAX,
    f64::MIN_POSITIVE,
    f64::EPSILON,
    // Common values
    1.0f64,
    -1.0f64,
    0.5f64,
    -0.5f64,
    2.0f64,
    -2.0f64,
    10.0f64,
    -10.0f64,
    100.0f64,
    -100.0f64,
    1000.0f64,
    -1000.0f64,
    // Small values
    1e-10f64,
    -1e-10f64,
    1e-100f64,
    -1e-100f64,
    // Large values
    1e10f64,
    -1e10f64,
    1e100f64,
    -1e100f64,
];

/// Representative f128 values for benchmarking
const F128_VALUES: &[f128] = &[
    // Special values
    0.0f128,
    -0.0f128,
    f128::INFINITY,
    f128::NEG_INFINITY,
    f128::NAN,
    // Boundary values
    f128::MIN,
    f128::MAX,
    f128::MIN_POSITIVE,
    f128::EPSILON,
    // Common values
    1.0f128,
    -1.0f128,
    0.5f128,
    -0.5f128,
    2.0f128,
    -2.0f128,
    10.0f128,
    -10.0f128,
    100.0f128,
    -100.0f128,
    1000.0f128,
    -1000.0f128,
    // Small values
    1e-10f128,
    -1e-10f128,
    1e-100f128,
    -1e-100f128,
    // Large values
    1e10f128,
    -1e10f128,
    1e100f128,
    -1e100f128,
    // Values that round-trip through f64
    (f64::MAX as f128),
    (f64::MIN as f128),
    (f64::MIN_POSITIVE as f128),
];

/// Representative integer values for benchmarking
const U8_VALUES: &[u8] = &[0, 1, i8::MAX as u8, u8::MAX];

const I8_VALUES: &[i8] = &[0, 1, i8::MAX, i8::MIN, -1];

const U16_VALUES: &[u16] = &[0, 1, i16::MAX as u16, u16::MAX];

const I16_VALUES: &[i16] = &[0, 1, i16::MAX, i16::MIN, -1];

const U32_VALUES: &[u32] = &[0, 1, i32::MAX as u32, u32::MAX];

const I32_VALUES: &[i32] = &[0, 1, i32::MAX, i32::MIN, -1];

const U64_VALUES: &[u64] = &[0, 1, i64::MAX as u64, u64::MAX];

const I64_VALUES: &[i64] = &[0, 1, i64::MAX, i64::MIN, -1];

const U128_VALUES: &[u128] = &[0, 1, i128::MAX as u128, u128::MAX];

const I128_VALUES: &[i128] = &[0, 1, i128::MAX, i128::MIN, -1];

// ============================================================================
// f16 <-> float benchmarks
// ============================================================================

bench_impl! {
    bench_f16_into_f32, "f16_into_f32", F16_VALUES,
    bench_f16_from_f32, "f16_from_f32", F32_VALUES,
    f16, F16, f32
}

bench_impl! {
    bench_f16_into_f64, "f16_into_f64", F16_VALUES,
    bench_f16_from_f64, "f16_from_f64", F64_VALUES,
    f16, F16, f64
}

// ============================================================================
// f128 <-> float benchmarks
// ============================================================================

bench_impl! {
    bench_f128_into_f32, "f128_into_f32", F128_VALUES,
    bench_f128_from_f32, "f128_from_f32", F32_VALUES,
    f128, F128, f32
}

bench_impl! {
    bench_f128_into_f64, "f128_into_f64", F128_VALUES,
    bench_f128_from_f64, "f128_from_f64", F64_VALUES,
    f128, F128, f64
}

// ============================================================================
// f16 <-> integer benchmarks
// ============================================================================

bench_impl! {
    bench_f16_into_u8, "f16_into_u8", F16_VALUES,
    bench_f16_from_u8, "f16_from_u8", U8_VALUES,
    f16, F16, u8
}
bench_impl! {
    bench_f16_into_i8, "f16_into_i8", F16_VALUES,
    bench_f16_from_i8, "f16_from_i8", I8_VALUES,
    f16, F16, i8
}
bench_impl! {
    bench_f16_into_u16, "f16_into_u16", F16_VALUES,
    bench_f16_from_u16, "f16_from_u16", U16_VALUES,
    f16, F16, u16
}
bench_impl! {
    bench_f16_into_i16, "f16_into_i16", F16_VALUES,
    bench_f16_from_i16, "f16_from_i16", I16_VALUES,
    f16, F16, i16
}
bench_impl! {
    bench_f16_into_u32, "f16_into_u32", F16_VALUES,
    bench_f16_from_u32, "f16_from_u32", U32_VALUES,
    f16, F16, u32
}
bench_impl! {
    bench_f16_into_i32, "f16_into_i32", F16_VALUES,
    bench_f16_from_i32, "f16_from_i32", I32_VALUES,
    f16, F16, i32
}
bench_impl! {
    bench_f16_into_u64, "f16_into_u64", F16_VALUES,
    bench_f16_from_u64, "f16_from_u64", U64_VALUES,
    f16, F16, u64
}
bench_impl! {
    bench_f16_into_i64, "f16_into_i64", F16_VALUES,
    bench_f16_from_i64, "f16_from_i64", I64_VALUES,
    f16, F16, i64
}
bench_impl! {
    bench_f16_into_u128, "f16_into_u128", F16_VALUES,
    bench_f16_from_u128, "f16_from_u128", U128_VALUES,
    f16, F16, u128
}
bench_impl! {
    bench_f16_into_i128, "f16_into_i128", F16_VALUES,
    bench_f16_from_i128, "f16_from_i128", I128_VALUES,
    f16, F16, i128
}

// ============================================================================
// f128 <-> integer benchmarks
// ============================================================================

bench_impl! {
    bench_f128_into_u8, "f128_into_u8", F128_VALUES,
    bench_f128_from_u8, "f128_from_u8", U8_VALUES,
    f128, F128, u8
}
bench_impl! {
    bench_f128_into_i8, "f128_into_i8", F128_VALUES,
    bench_f128_from_i8, "f128_from_i8", I8_VALUES,
    f128, F128, i8
}
bench_impl! {
    bench_f128_into_u16, "f128_into_u16", F128_VALUES,
    bench_f128_from_u16, "f128_from_u16", U16_VALUES,
    f128, F128, u16
}
bench_impl! {
    bench_f128_into_i16, "f128_into_i16", F128_VALUES,
    bench_f128_from_i16, "f128_from_i16", I16_VALUES,
    f128, F128, i16
}
bench_impl! {
    bench_f128_into_u32, "f128_into_u32", F128_VALUES,
    bench_f128_from_u32, "f128_from_u32", U32_VALUES,
    f128, F128, u32
}
bench_impl! {
    bench_f128_into_i32, "f128_into_i32", F128_VALUES,
    bench_f128_from_i32, "f128_from_i32", I32_VALUES,
    f128, F128, i32
}
bench_impl! {
    bench_f128_into_u64, "f128_into_u64", F128_VALUES,
    bench_f128_from_u64, "f128_from_u64", U64_VALUES,
    f128, F128, u64
}
bench_impl! {
    bench_f128_into_i64, "f128_into_i64", F128_VALUES,
    bench_f128_from_i64, "f128_from_i64", I64_VALUES,
    f128, F128, i64
}
bench_impl! {
    bench_f128_into_u128, "f128_into_u128", F128_VALUES,
    bench_f128_from_u128, "f128_from_u128", U128_VALUES,
    f128, F128, u128
}
bench_impl! {
    bench_f128_into_i128, "f128_into_i128", F128_VALUES,
    bench_f128_from_i128, "f128_from_i128", I128_VALUES,
    f128, F128, i128
}

// ============================================================================
// Criterion setup
// ============================================================================

criterion_group!(
    benches,
    bench_f16_into_u8,
    bench_f16_from_u8,
    bench_f16_into_i8,
    bench_f16_from_i8,
    bench_f16_into_u16,
    bench_f16_from_u16,
    bench_f16_into_i16,
    bench_f16_from_i16,
    bench_f16_into_u32,
    bench_f16_from_u32,
    bench_f16_into_i32,
    bench_f16_from_i32,
    bench_f16_into_u64,
    bench_f16_from_u64,
    bench_f16_into_i64,
    bench_f16_from_i64,
    bench_f16_into_u128,
    bench_f16_from_u128,
    bench_f16_into_i128,
    bench_f16_from_i128,
    bench_f16_into_f32,
    bench_f16_from_f32,
    bench_f16_into_f64,
    bench_f16_from_f64,
    bench_f128_into_u8,
    bench_f128_from_u8,
    bench_f128_into_i8,
    bench_f128_from_i8,
    bench_f128_into_u16,
    bench_f128_from_u16,
    bench_f128_into_i16,
    bench_f128_from_i16,
    bench_f128_into_u32,
    bench_f128_from_u32,
    bench_f128_into_i32,
    bench_f128_from_i32,
    bench_f128_into_u64,
    bench_f128_from_u64,
    bench_f128_into_i64,
    bench_f128_from_i64,
    bench_f128_into_u128,
    bench_f128_from_u128,
    bench_f128_into_i128,
    bench_f128_from_i128,
    bench_f128_from_f32,
    bench_f128_into_f32,
    bench_f128_from_f64,
    bench_f128_into_f64,
);

criterion_main!(benches);
