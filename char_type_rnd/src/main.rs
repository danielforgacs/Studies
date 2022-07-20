fn main() {
    let text = "Δέκα";
    for c in text.chars() {
        println!("{}: 0x{:>04x}, {} -> {:032b}", c, c as u32, c as u32, c as u32);
    }

    let char_u32 = 916_u32;
    let char_u32 = 0x0394_u32;
    if let Some(char_from_u32) = std::char::from_u32(char_u32) {
        println!("new char: {}", char_from_u32);
    };
    let char_bytes = [
        (char_u32 >> 24) as u8,
        (char_u32 >> 16) as u8,
        (char_u32 >> 8) as u8,
        (char_u32 >> 0) as u8,
    ];
    println!("char bytes: {:?}", char_bytes);
    println!("char bytes array: {:08b} {:08b} {:08b} {:08b}", char_bytes[0], char_bytes[1], char_bytes[2], char_bytes[3]);
    let bytes_text = String::from_utf8(char_bytes.to_vec()).unwrap_or("[conversion error]".to_string());
    println!("String from u8 vec/array: {}", bytes_text);

    let pizza_from_hex = '\u{01f355}';
    println!("pizza_from_hex: {}", pizza_from_hex);
    println!("pizza_from_hex u32: {:>032b}", pizza_from_hex as u32);

    let pizza_bits = 0b00000000_00000001_11110011_01010101_u32;

}
