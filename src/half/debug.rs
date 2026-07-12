use core::fmt::{Debug, Formatter, Result};

use super::f16;

const F16_INF: u16 = 0x7c00;

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
    distance: f64,
    integer: u32,
    digits: i32,
    exponent: i32,
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

        let negative = exact.is_sign_negative();
        let absolute = if negative {
            -(exact as f64)
        } else {
            exact as f64
        };
        let mut normalized = absolute;
        let mut exponent = 0;
        while normalized >= 10.0 {
            normalized /= 10.0;
            exponent += 1;
        }
        while normalized < 1.0 {
            normalized *= 10.0;
            exponent -= 1;
        }

        let mut selected = None;
        for digits in 1..=5 {
            let decimal_exponent = exponent - (digits - 1);
            let scaled = absolute / pow10(decimal_exponent);
            let lower = scaled as u32;
            let upper = lower + 1;
            let nearest = if scaled - lower as f64 >= 0.5 {
                upper
            } else {
                lower
            };

            let mut best: Option<Candidate> = None;
            for integer in [nearest, lower, upper] {
                let value = integer as f64 * pow10(decimal_exponent);
                let signed = if negative { -value } else { value };
                if from_f32(signed as f32).to_bits() != self.to_bits() {
                    continue;
                }

                let distance = if value >= absolute {
                    value - absolute
                } else {
                    absolute - value
                };
                let replace = match best {
                    None => true,
                    Some(ref previous) => {
                        let distance_delta = if distance >= previous.distance {
                            distance - previous.distance
                        } else {
                            previous.distance - distance
                        };
                        distance < previous.distance
                            || (distance_delta <= f64::EPSILON * absolute
                                && integer > previous.integer)
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

fn pow10(mut exponent: i32) -> f64 {
    let mut value = 1.0;
    if exponent >= 0 {
        while exponent > 0 {
            value *= 10.0;
            exponent -= 1;
        }
    } else {
        while exponent < 0 {
            value /= 10.0;
            exponent += 1;
        }
    }
    value
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

fn from_f32(value: f32) -> f16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = bits & 0x007f_ffff;

    if exponent == 0 {
        return f16(sign);
    }
    if exponent == 0xff {
        return if mantissa == 0 {
            f16(sign | F16_INF)
        } else {
            f16(sign | F16_INF | 0x0200 | ((mantissa >> 13) as u16 & 0x03ff))
        };
    }

    let f16_exponent = exponent as i32 - 112;
    if f16_exponent > 30 {
        return f16(sign | F16_INF);
    }
    if f16_exponent <= 0 {
        let shift = (14 - f16_exponent) as u32;
        if shift > 24 {
            return f16(sign);
        }
        let significand = mantissa | 0x0080_0000;
        let truncated = significand >> shift;
        let remainder = significand & ((1 << shift) - 1);
        let halfway = 1 << (shift - 1);
        let rounded = truncated
            + u32::from(remainder > halfway || (remainder == halfway && truncated & 1 != 0));
        return f16(sign | rounded as u16);
    }

    let truncated = mantissa >> 13;
    let remainder = mantissa & 0x1fff;
    let rounded =
        truncated + u32::from(remainder > 0x1000 || (remainder == 0x1000 && truncated & 1 != 0));
    if rounded < 0x400 {
        f16(sign | ((f16_exponent as u16) << 10) | rounded as u16)
    } else if f16_exponent >= 30 {
        f16(sign | F16_INF)
    } else {
        f16(sign | (((f16_exponent + 1) as u16) << 10))
    }
}
