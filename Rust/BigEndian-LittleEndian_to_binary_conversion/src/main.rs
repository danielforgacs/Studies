use byteorder::{ByteOrder, BigEndian, LittleEndian};

fn main() {
    let num = 0b00000000_00000000_11111111_11111111;
    println!("Number:           {}", num);
    println!("Number:           {:032b}", num);

    let mut buf = [0; 4];
    BigEndian::write_u32(&mut buf, num);
    assert_eq!(num, BigEndian::read_u32(&buf));
    println!("Big endian:       {:?}", buf);
    println!("Big endian:       {:08b} {:08b} {:08b} {:08b}", buf[0], buf[1], buf[2], buf[3]);

    let mut buf = [0; 4];
    LittleEndian::write_u32(&mut buf, num);
    assert_eq!(num, LittleEndian::read_u32(&buf));
    println!("little endian:    {:?}", buf);
    println!("little endian:    {:08b} {:08b} {:08b} {:08b}", buf[0], buf[1], buf[2], buf[3]);

    println!("\nOwn converter:");

    let convert = make_big_endian_u32(&num);
    println!("big endian:       {:?}", convert);
    println!("big endian:       {:08b} {:08b} {:08b} {:08b}", convert[0], convert[1], convert[2], convert[3]);
    let convert = make_little_endian_u32(&num);
    println!("little endian:    {:?}", convert);
    println!("little endian:    {:08b} {:08b} {:08b} {:08b}", convert[0], convert[1], convert[2], convert[3]);
}

fn make_big_endian_u32(num: &u32) -> [u8; 4] {
    let mut result = [0_u8; 4];
    result[0] = (num >> 24) as u8;
    result[1] = (num >> 16) as u8;
    result[2] = (num >> 8) as u8;
    result[3] = (num >> 0) as u8;
    result
}

fn make_little_endian_u32(num: &u32) -> [u8; 4] {
    let mut result = [255_u8; 4];
    result[3] = (num >> 24) as u8;
    result[2] = (num >> 16) as u8;
    result[1] = (num >> 8) as u8;
    result[0] = (num >> 0) as u8;
    result
}
