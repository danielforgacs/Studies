fn main() {
    let num: f32 = 42.42;
    let num_bits: u32 = num.to_bits();
    assert_eq!(num_bits, 0b_01000010_00101001_10101110_00010100);
    println!("{:<25}{:032b} - {}", "num:", num_bits, num_bits);

    let sign_bit_raw: u32 = num_bits >> 31;
    assert_eq!(sign_bit_raw, 0b_00000000_00000000_00000000_00000000);

    let sign_bit: u8 = (num_bits >> 31) as u8;
    assert_eq!(sign_bit, 0b0);
    println!("{:<25}{:032b} - {}", "sign bit (u8):", sign_bit, sign_bit);

    let exponent_raw: u32 = num_bits & 0b_01111111_10000000_00000000_00000000;
    assert_eq!(exponent_raw, 0b_01000010_00000000_00000000_00000000);

    let exponent_raw: u8 = (num_bits >> 23) as u8;
    println!("{:<25}{:032b} - {}", "exponent_raw:", exponent_raw, exponent_raw);
    assert_eq!(exponent_raw, 0b10000100);

    let f32_exponent_bias: i16 = 127;
    let exponent: i16 = (exponent_raw as i16) - f32_exponent_bias;
    println!("{:<25}{:032b} - {}", "exponent - bias:", exponent, exponent);
    assert_eq!(exponent, 5);

    let mantissa_bits: u32 = num_bits & 0b_00000000_01111111_11111111_11111111;
    assert_eq!(mantissa_bits, 0b_00000000_00101001_10101110_00010100);
    println!("{:<25}{:032b} - {}", "mantissa_bits:", mantissa_bits, mantissa_bits);

    let mut mantissa = 1.0;

    for index in 0..23 {
        let mask = 1 << index;
        let one_at_index = mantissa_bits & mask;
        if one_at_index != 0 {
            let index_f = index as f32;
            let weigth = 2_f32.powf(index_f - 23.0);
            mantissa += weigth;
        }
    }
    println!("{:<25}{}", "mantissa:", mantissa);
    let mantissa_re_bits: u32 = mantissa.to_bits();
    println!("{:<25}{:32b} - {}", "mantissa_re_bits:", mantissa_re_bits, mantissa_re_bits);
    let mantissa_re_bits: u32 = mantissa.to_bits() & 0b_00000000_01111111_11111111_11111111;
    println!("{:<25}{:32b} - {}", "mantissa_re_bits:", mantissa_re_bits, mantissa_re_bits);
    assert_eq!(mantissa_re_bits, mantissa_bits);

    let num_rebuild = mantissa * 2_f32.powf(exponent as f32);
    println!("{:<25}{}", "rebuilt float num:", num_rebuild);
    assert_eq!(num, num_rebuild);
}
