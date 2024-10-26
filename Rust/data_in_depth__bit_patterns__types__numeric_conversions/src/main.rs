use std::mem::transmute;

fn main() {
    datatype_and_bit_sequence_vs_value();
    interpreting_float_bit_sequence_as_integer();
    inspecting_endianness();
    look_inside_an_f32();
}

// Listing 5.1
fn datatype_and_bit_sequence_vs_value() {
    {
        println!("\n::>>> datatype_and_bit_sequence_vs_value:");
        let a: u16 = 50115;
        let b: i16 = -15421;
        println!("a(u16):    {:016b},  {}", a, a);
        println!("a(i16):    {:016b}, {}", b, b);
        debug_assert!(format!("{:016b}", a) == format!("{:016b}", b))
    }
    {
        let x: u8 = 0b10111011;
        let y: i8 = -69;
        println!("x: u8:    {:08b} = {}", x, x);
        println!("x: i8:    {:08b} = {}", y, y);
    }
}

// Listing 5.2
fn interpreting_float_bit_sequence_as_integer() {
    println!("\n::>>> interpreting_float_bit_sequence_as_integer:");
    let a: f32 = 42.42;
    println!("a:            {}", a);
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };
    println!("frankentype:  {}", frankentype);
    println!("frankentype:  {:032b}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };
    println!("b(f32):       {}", b);
    assert_eq!(a, b);
}

// 5.6
fn inspecting_endianness() {
    println!("\n::>>> inspecting_endianness:");
    {
        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
        let num_big_e: i32 = unsafe { transmute(big_endian) };
        let num_little_e: i32 = unsafe { transmute(little_endian) };

        println!("big:      {}", num_big_e);
        println!("little:   {}", num_little_e);
    };
    println!("");
    {
        let big_endian: [u8; 4] = [0b00000000, 0b00000000, 0b00000000, 0b11111111];
        let big_endian: u32 = unsafe { transmute(big_endian) };
        println!("big 2:      {}", big_endian);

        let little_endian: [u8; 4] = [0b11111111, 0b00000000, 0b00000000, 0b00000000];
        let little_endian: u32 = unsafe { transmute(little_endian) };
        println!("little 2:   {}", little_endian);
    };
    println!("");
    {
        let big_endian: u32 = 0b11111111_00000000_00000000_00000000;
        let big_endian: u32 = unsafe { transmute(big_endian) };
        println!("big 2:      {}", big_endian);

        let little_endian: u32 = 0b00000000_00000000_00000000_11111111;
        let little_endian: u32 = unsafe { transmute(little_endian) };
        println!("little 2:   {}", little_endian);
    };
    println!("");
    {
        let num: [u8; 4] = [0b11111111, 0b00000000, 0b00000000, 0b00000000];
        let big_endian:[u8; 4] = [num[3], num[2], num[1], num[0]];
        let little_endian:[u8; 4] = [num[0], num[1], num[2], num[3]];
        let big_endian: u32 = unsafe { transmute(big_endian) };
        let little_endian: u32 = unsafe { transmute(little_endian) };
        println!("big 2:      {}", big_endian);
        println!("little 2:   {}", little_endian);
    };
    println!("");
    {
        let num: [u8; 2] = [0b11111111, 0b00000000];
        let big_endian:[u8; 2] = [num[1], num[0]];
        let little_endian:[u8; 2] = [num[0], num[1]];
        let big_endian: u16 = unsafe { transmute(big_endian) };
        let little_endian: u16 = unsafe { transmute(little_endian) };
        println!("big:      {:>10} - {:016b}", big_endian, big_endian);
        println!("little:   {:>10} - {:016b}", little_endian, little_endian);
        let from_bin: u16 = 0b11111111_00000000;
        let from_bin_conv: i16 = unsafe { transmute(from_bin) };
        println!("ctrl u16: {:>10} - {:016b}", from_bin, from_bin);
        println!("ctrl i16: {:>10} - {:016b}", from_bin_conv, from_bin_conv);
    };
}

// 5.4.1
fn look_inside_an_f32() {
    /*
    00000000  00000000  00000000 00000000
    0|0000000 0|0000000 00000000 00000000
    0           0000000 0       0000000 00000000 00000000
    sign bit    exponent        mantissa
    1 bit       8bit / 1 byte   23bit
    */

    println!("");
    println!(":: look inside an f23");
    {
        // 00111111  10000000 00000000 00000000
        // 0 01111111 0000000 00000000 00000000
        let num: f32 = 1.0;
        let num_x: u32 = unsafe { transmute(num) };
        let num_x_x: f32 = unsafe { transmute(num) };
        assert_eq!(num, num_x_x);
        println!("{:<20}{}", "num: f32", num);
        println!("{:<20}{:032b}", "num: u32", num_x);
    }
    println!("");
    {
        // 10111111  10000000 00000000 00000000
        // 1 01111111 0000000 00000000 00000000
        let num: f32 = -1.0;
        let num_x: u32 = unsafe { transmute(num) };
        let num_x_x: f32 = unsafe { transmute(num) };
        assert_eq!(num, num_x_x);
        println!("{:<20}{}", "num: f32", num);
        println!("{:<20}{:032b}", "num: u32", num_x);
    }
    println!("");
    println!(":: 42.42");
    {
        let num: f32 = unsafe { transmute(0b0_10000100_0101001_10101110_00010100) };
        // let num: f32 = unsafe { transmute(0x4229AE14) };
        assert_eq!(num, 42.42);
        let num_x: u32 = unsafe { transmute(num) };
        let num_x_x: f32 = unsafe { transmute(num_x) };
        assert_eq!(num, num_x_x);
        println!("{:<20}{}", "num: f32", num);
        println!("{:<20}{:032b}", "num: u32", num_x);
    }
    println!("");
    println!(":: 42.42 components");
    {
        // 42.42 -> 0 10000100 0101001 10101110 00010100
        let sign_bit: u32 = unsafe { transmute(0b00000000) };
        let exponent: u32 = unsafe { transmute(0b10000100) };
        let mantissa: u32 = unsafe { transmute(0b101001_10101110_00010100) };
        println!("{:<20}{}", "sign bit", sign_bit);
        println!("{:<20}{}", "exponent", exponent);
        println!("{:<20}{}", "mantissa", mantissa);
    }
    println!("");
    println!(":: 42.42 isolating the components");
    {
        // 42.42 -> 0 10000100 0101001 10101110 00010100
        let num: f32 = 42.42;
        let num_bits = num.to_bits();
        let num_x: u32 = unsafe { transmute(num) };
        println!("{:<20}{}", "num", num);
        println!("{:<20}{:032b}", "num bits", num_bits);

        let sign_bit = num_bits >> 31;
        assert_eq!(sign_bit, 0b0);
        assert_eq!(num_bits, num_x);
        println!("{:<20}{:032b}", "sign bit", sign_bit);

        let exponent = num_bits >> 23;
        assert_eq!(exponent, 0b10000100);
        println!("{:<20}{:032b} / {}", "exponent raw", exponent, exponent);
        let exponent = exponent & 0b11111111;
        let exponent_bias = 127_i32;
        let exponent = (exponent as i32) - exponent_bias;
        println!("{:<20}{:032b} / {}", "exponent", exponent, exponent);
        let mantissa: u32 = num_bits & 0b00000000_01111111_11111111_11111111;
        println!("{:<20}{:032b} / {}", "mantissa", mantissa, mantissa);
    }
}
