use core::fmt::{Debug, Formatter, Result};

use super::f16;

struct Buffer {
    bytes: [u8; 16],
    len: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            bytes: [0; 16],
            len: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        self.bytes[self.len] = byte;
        self.len += 1;
    }

    fn extend(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.push(byte);
        }
    }

    fn push_u32(&mut self, mut value: u32) {
        let start = self.len;
        loop {
            self.push(b'0' + (value % 10) as u8);
            value /= 10;
            if value == 0 {
                break;
            }
        }
        self.bytes[start..self.len].reverse();
    }

    fn as_str(&self) -> &str {
        // SAFETY: every byte written by this module is ASCII.
        unsafe { core::str::from_utf8_unchecked(&self.bytes[..self.len]) }
    }
}

struct Candidate {
    distance: u128,
    integer: u32,
    digits: i32,
    exponent: i32,
}

/// The exact integer significand `m` and binary exponent `e` such that a
/// nonnegative binary16 magnitude bit pattern (0..=0x7c00) equals `m * 2^e`.
/// Extending the formula algebraically to `0x7c00` (normally infinity) gives
/// the exact value `2^16`, the true rounding boundary just past `f16::MAX` --
/// useful as a sentinel, even though `0x7c00` is not itself a finite value.
fn magnitude(bits: u16) -> (u128, i32) {
    let exp_field = (bits >> 10) & 0x1f;
    let mant_field = (bits & 0x3ff) as u128;
    if exp_field == 0 {
        (mant_field, -24)
    } else {
        (mant_field | 0x400, exp_field as i32 - 25)
    }
}

/// The exact midpoint `(a + b) / 2` of two `m * 2^e` values, computed by
/// aligning to a common exponent and halving via an exponent decrement --
/// exact, since halving a binary integer never loses precision.
fn midpoint(a: (u128, i32), b: (u128, i32)) -> (u128, i32) {
    let (small, large) = if a.1 <= b.1 { (a, b) } else { (b, a) };
    let shift = (large.1 - small.1) as u32;
    (small.0 + (large.0 << shift), small.1 - 1)
}

/// Converts an exact `m * 2^e` value to `n * 10^exp10`, exact for every input
/// in range here: multiplying by `5^-e` turns a `2^e` (e < 0) denominator
/// into a `10^e` one.
fn to_decimal(m_e: (u128, i32)) -> (u128, i32) {
    let (m, e) = m_e;
    if e >= 0 {
        (m << e, 0)
    } else {
        (m * pow5((-e) as u32), e)
    }
}

fn pow10(exponent: u32) -> u128 {
    let mut value = 1u128;
    for _ in 0..exponent {
        value *= 10;
    }
    value
}

fn pow5(exponent: u32) -> u128 {
    let mut value = 1u128;
    for _ in 0..exponent {
        value *= 5;
    }
    value
}

fn digit_count(n: u128) -> u32 {
    let mut n = n;
    let mut count = 1;
    while n >= 10 {
        n /= 10;
        count += 1;
    }
    count
}

/// Exact-rational comparison of two `n * 10^exp10` values via cross
/// multiplication after aligning to the smaller (finer) exponent.
fn decimal_cmp(a: (u128, i32), b: (u128, i32)) -> core::cmp::Ordering {
    let (na, ea) = a;
    let (nb, eb) = b;
    if ea <= eb {
        na.cmp(&(nb * pow10((eb - ea) as u32)))
    } else {
        (na * pow10((ea - eb) as u32)).cmp(&nb)
    }
}

impl Debug for f16 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let exact = to_f32(*self);

        // Explicit precision uses exact fixed formatting. Every binary16 value
        // is exactly representable as f32, so the primitive f32 formatter has
        // the same result as nightly's binary16 formatter in this mode.
        if formatter.precision().is_some() || !exact.is_finite() || exact == 0.0 {
            return Debug::fmt(&exact, formatter);
        }

        // Everything below is exact integer arithmetic on the bit pattern, not
        // floating point, so the result cannot depend on the ambient rounding mode.
        let negative = (self.0 & 0x8000) != 0;
        let bits = self.0 & 0x7fff;
        let self_even = (bits & 1) == 0;

        let value = to_decimal(magnitude(bits));
        let low_mid = to_decimal(midpoint(magnitude(bits - 1), magnitude(bits)));
        let high_mid = to_decimal(midpoint(magnitude(bits), magnitude(bits + 1)));

        let exponent = digit_count(value.0) as i32 - 1 + value.1;

        let mut selected = None;
        for digits in 1..=5 {
            let decimal_exponent = exponent - (digits - 1);
            let a = decimal_exponent - value.1;
            let (q, r, d) = if a <= 0 {
                (value.0 * pow10((-a) as u32), 0u128, 1u128)
            } else {
                let d = pow10(a as u32);
                (value.0 / d, value.0 % d, d)
            };
            let lower = q as u32;
            let upper = lower + 1;
            let nearest = if r * 2 >= d { upper } else { lower };

            let mut best: Option<Candidate> = None;
            for integer in [nearest, lower, upper] {
                let candidate = (integer as u128, decimal_exponent);
                let round_trips = match (
                    decimal_cmp(candidate, low_mid),
                    decimal_cmp(candidate, high_mid),
                ) {
                    (core::cmp::Ordering::Greater, core::cmp::Ordering::Less) => true,
                    (core::cmp::Ordering::Equal, _) | (_, core::cmp::Ordering::Equal) => self_even,
                    _ => false,
                };
                if !round_trips {
                    continue;
                }

                let distance = if integer == lower { r } else { d - r };
                let replace = match best {
                    None => true,
                    Some(ref previous) => {
                        distance < previous.distance
                            || (distance == previous.distance && integer > previous.integer)
                    }
                };
                if replace {
                    best = Some(Candidate {
                        distance,
                        integer,
                        digits,
                        exponent,
                    });
                }
            }

            if best.is_some() {
                selected = best;
                break;
            }
        }

        let selected = selected.expect("five decimal digits identify every finite binary16 value");
        let body = render(selected);
        formatter.pad_integral(!negative, "", body.as_str())
    }
}

fn render(candidate: Candidate) -> Buffer {
    let mut integer = candidate.integer;
    let mut decimal_exponent = candidate.exponent - (candidate.digits - 1);
    while integer >= 10 && integer % 10 == 0 {
        integer /= 10;
        decimal_exponent += 1;
    }

    let mut digits = Buffer::new();
    digits.push_u32(integer);

    let mut output = Buffer::new();
    let scientific = candidate.exponent < -4;
    if scientific {
        output.push(digits.bytes[0]);
        if digits.len > 1 {
            output.push(b'.');
            output.extend(&digits.bytes[1..digits.len]);
        }
        output.push(b'e');
        let exponent = decimal_exponent + digits.len as i32 - 1;
        if exponent < 0 {
            output.push(b'-');
            output.push_u32((-exponent) as u32);
        } else {
            output.push_u32(exponent as u32);
        }
    } else {
        let point = digits.len as i32 + decimal_exponent;
        if point <= 0 {
            output.extend(b"0.");
            for _ in 0..-point {
                output.push(b'0');
            }
            output.extend(&digits.bytes[..digits.len]);
        } else if point >= digits.len as i32 {
            output.extend(&digits.bytes[..digits.len]);
            for _ in 0..point - digits.len as i32 {
                output.push(b'0');
            }
            output.extend(b".0");
        } else {
            output.extend(&digits.bytes[..point as usize]);
            output.push(b'.');
            output.extend(&digits.bytes[point as usize..digits.len]);
        }
    }
    output
}

fn to_f32(value: f16) -> f32 {
    let bits = value.0 as u32;
    let sign = (bits & 0x8000) << 16;
    let exponent = (bits >> 10) & 0x1f;
    let mantissa = bits & 0x03ff;

    if exponent == 0x1f {
        return f32::from_bits(sign | 0x7f80_0000 | (mantissa << 13));
    }
    if exponent != 0 {
        return f32::from_bits(sign | ((exponent + 112) << 23) | (mantissa << 13));
    }
    if mantissa == 0 {
        return f32::from_bits(sign);
    }

    let shift = mantissa.leading_zeros() - 21;
    let normalized = ((mantissa << shift) & 0x03ff) << 13;
    f32::from_bits(sign | ((113 - shift) << 23) | normalized)
}
