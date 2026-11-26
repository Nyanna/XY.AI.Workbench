/// Round-to-nearest-even conversion to IEEE-754 binary16 bits; NaN becomes the canonical quiet NaN.
pub fn f32_to_f16_bits2(x: f32) -> u16 {
    let bits = x.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let mag = bits & 0x7FFF_FFFF;
    if mag > 0x7F80_0000 {
        return sign | 0x7E00;
    }
    if mag >= 0x477F_F000 {
        return sign | 0x7C00;
    }
    if mag >= 0x3880_0000 {
        let r = mag - 0x3800_0000;
        return sign | ((r + 0xFFF + ((r >> 13) & 1)) >> 13) as u16;
    }
    let exp = mag >> 23;
    if exp < 102 {
        return sign;
    }
    let mant = (mag & 0x7F_FFFF) | 0x80_0000;
    let shift = 126 - exp;
    let mut q = mant >> shift;
    let rem = mant & ((1u32 << shift) - 1);
    let half = 1u32 << (shift - 1);
    if rem > half || (rem == half && (q & 1) == 1) {
        q += 1;
    }
    sign | q as u16
}

/// Round-to-nearest-even conversion to bfloat16 bits; NaN becomes the canonical quiet NaN.
pub fn f32_to_bf16_bits2(x: f32) -> u16 {
    let bits = x.to_bits();
    if bits & 0x7FFF_FFFF > 0x7F80_0000 {
        return ((bits >> 16) & 0x8000) as u16 | 0x7FC0;
    }
    (bits.wrapping_add(0x7FFF + ((bits >> 16) & 1)) >> 16) as u16
}

#[cfg(test)]
#[path = "test/float_codec.rs"]
mod tests;
