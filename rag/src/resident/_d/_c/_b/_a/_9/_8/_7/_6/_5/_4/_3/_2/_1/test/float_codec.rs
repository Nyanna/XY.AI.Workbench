use super::*;

fn f16_to_f32(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0f32 } else { 1.0 };
    let exp = ((h >> 10) & 0x1F) as i32;
    let man = (h & 0x3FF) as f32;
    match exp {
        0 => sign * man * 2f32.powi(-24),
        31 if man == 0.0 => sign * f32::INFINITY,
        31 => f32::NAN,
        _ => sign * (1.0 + man / 1024.0) * 2f32.powi(exp - 15),
    }
}

fn f16_table() -> Vec<f64> {
    (0..=0x7BFFu16).map(|h| f16_to_f32(h) as f64).collect()
}

fn ref_f16(table: &[f64], x: f32) -> u16 {
    let sign = if x.is_sign_negative() { 0x8000u16 } else { 0 };
    if x.is_nan() {
        return sign | 0x7E00;
    }
    let a = (x as f64).abs();
    if a.is_infinite() {
        return sign | 0x7C00;
    }
    let lo = table.partition_point(|v| *v <= a) - 1;
    if lo == 0x7BFF {
        return sign | if a >= (65504.0 + 65536.0) / 2.0 { 0x7C00 } else { 0x7BFF };
    }
    let hi = lo + 1;
    let d_lo = a - table[lo];
    let d_hi = table[hi] - a;
    let pick = if d_lo < d_hi {
        lo
    } else if d_hi < d_lo {
        hi
    } else if lo % 2 == 0 {
        lo
    } else {
        hi
    };
    sign | pick as u16
}

fn ref_bf16(x: f32) -> u16 {
    let bits = x.to_bits();
    let sign = (bits >> 16) as u16 & 0x8000;
    if x.is_nan() {
        return sign | 0x7FC0;
    }
    let upper = (bits >> 16) as u64;
    let lower = (bits & 0xFFFF) as u64;
    let up = lower > 0x8000 || (lower == 0x8000 && upper % 2 == 1);
    (upper + up as u64) as u16
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }

    fn unit(&mut self) -> f32 {
        (self.next() as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

#[test]
fn f16_known_values() {
    assert_eq!(f32_to_f16_bits2(1.0), 0x3C00);
    assert_eq!(f32_to_f16_bits2(-2.0), 0xC000);
    assert_eq!(f32_to_f16_bits2(0.0), 0x0000);
    assert_eq!(f32_to_f16_bits2(-0.0), 0x8000);
    assert_eq!(f32_to_f16_bits2(65504.0), 0x7BFF);
    assert_eq!(f32_to_f16_bits2(f32::from_bits(0x477F_EFFF)), 0x7BFF);
    assert_eq!(f32_to_f16_bits2(65520.0), 0x7C00);
    assert_eq!(f32_to_f16_bits2(1e10), 0x7C00);
    assert_eq!(f32_to_f16_bits2(-1e10), 0xFC00);
    assert_eq!(f32_to_f16_bits2(f32::INFINITY), 0x7C00);
    assert_eq!(f32_to_f16_bits2(f32::NEG_INFINITY), 0xFC00);
    assert_eq!(f32_to_f16_bits2(f32::NAN), 0x7E00);
    assert_eq!(f32_to_f16_bits2(-f32::NAN), 0xFE00);
    assert_eq!(f32_to_f16_bits2(f32::from_bits(0x7F80_0001)), 0x7E00);
    assert_eq!(f32_to_f16_bits2(f32::from_bits(0x7FBF_FFFF)), 0x7E00);
    assert_eq!(f32_to_f16_bits2(2f32.powi(-24)), 0x0001);
    assert_eq!(f32_to_f16_bits2(2f32.powi(-25)), 0x0000);
    assert_eq!(f32_to_f16_bits2(f32::from_bits(2f32.powi(-25).to_bits() + 1)), 0x0001);
    assert_eq!(f32_to_f16_bits2(3.0 * 2f32.powi(-25)), 0x0002);
    assert_eq!(f32_to_f16_bits2(2f32.powi(-14)), 0x0400);
    assert_eq!(f32_to_f16_bits2(-2f32.powi(-30)), 0x8000);
    assert_eq!(f32_to_f16_bits2(f32::MIN_POSITIVE / 2.0), 0x0000);
}

#[test]
fn f16_round_trips_every_finite_value() {
    for h in 0..=0xFFFFu16 {
        if (h & 0x7C00) == 0x7C00 && (h & 0x3FF) != 0 {
            continue;
        }
        assert_eq!(f32_to_f16_bits2(f16_to_f32(h)), h, "{h:#06x}");
    }
}

#[test]
fn f16_matches_reference_on_ties_neighbours_and_random_inputs() {
    let table = f16_table();
    let mut inputs: Vec<f32> = Vec::new();
    for lo in 0..0x7BFFusize {
        let mid = ((table[lo] + table[lo + 1]) / 2.0) as f32;
        let b = mid.to_bits();
        inputs.extend([f32::from_bits(b - 1), mid, f32::from_bits(b + 1)]);
    }
    let mut rng = Lcg(7);
    for _ in 0..200_000 {
        inputs.push(f32::from_bits(rng.next()));
    }
    for _ in 0..50_000 {
        inputs.push(rng.unit() * 70_000.0);
        inputs.push(rng.unit() * 1e-4);
    }
    for x in inputs {
        for v in [x, -x] {
            assert_eq!(f32_to_f16_bits2(v), ref_f16(&table, v), "{:#010x}", v.to_bits());
        }
    }
}

#[test]
fn bf16_known_values() {
    assert_eq!(f32_to_bf16_bits2(1.0), 0x3F80);
    assert_eq!(f32_to_bf16_bits2(-1.0), 0xBF80);
    assert_eq!(f32_to_bf16_bits2(0.0), 0);
    assert_eq!(f32_to_bf16_bits2(-0.0), 0x8000);
    assert_eq!(f32_to_bf16_bits2(f32::from_bits(0x3F80_8000)), 0x3F80);
    assert_eq!(f32_to_bf16_bits2(f32::from_bits(0x3F81_8000)), 0x3F82);
    assert_eq!(f32_to_bf16_bits2(f32::from_bits(0x3F80_8001)), 0x3F81);
    assert_eq!(f32_to_bf16_bits2(f32::from_bits(0x3F80_7FFF)), 0x3F80);
    assert_eq!(f32_to_bf16_bits2(f32::MAX), 0x7F80);
    assert_eq!(f32_to_bf16_bits2(f32::INFINITY), 0x7F80);
    assert_eq!(f32_to_bf16_bits2(f32::NEG_INFINITY), 0xFF80);
    assert_eq!(f32_to_bf16_bits2(f32::NAN), 0x7FC0);
    assert_eq!(f32_to_bf16_bits2(-f32::NAN), 0xFFC0);
    assert_eq!(f32_to_bf16_bits2(f32::from_bits(0x7F80_0001)), 0x7FC0);
    assert_eq!(f32_to_bf16_bits2(f32::from_bits(1)), 0);
}

#[test]
fn bf16_matches_reference_on_random_inputs() {
    let mut rng = Lcg(11);
    for _ in 0..500_000 {
        let x = f32::from_bits(rng.next());
        assert_eq!(f32_to_bf16_bits2(x), ref_bf16(x), "{:#010x}", x.to_bits());
    }
    for upper in 0..=0xFFFFu32 {
        for lower in [0u32, 0x7FFF, 0x8000, 0x8001, 0xFFFF] {
            let x = f32::from_bits((upper << 16) | lower);
            assert_eq!(f32_to_bf16_bits2(x), ref_bf16(x), "{:#010x}", x.to_bits());
        }
    }
}
