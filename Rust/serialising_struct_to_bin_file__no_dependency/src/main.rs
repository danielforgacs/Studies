use std::fs::File;
use std::io::prelude::*;

// #[derive(Clone)]
struct Person {
    name: String,
    year: u16,
}

impl Person {
    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        for c in self.name.chars() {
            buf.push(c as u8);
        }
        buf.push(0_u8);
        buf.push(self.year as u8);
        buf.push((self.year >> 8) as u8);
        buf
    }

    fn from_bytes(buf: Vec<u8>) -> Self {
        let mut name = String::new();
        let mut index = 0;
        loop {
            let byte = buf[index];
            if byte == 0 {
                index += 1;
                break;
            }
            name.push(byte as char);
            index += 1;
            if index == buf.len()-1{
                break;
            }
        }
        let y_low = buf[index] as u16;
        let y_high = (buf[index+1] as u16) << 8;
        let year = y_low | y_high;
        println!("::: {}", year);
        Self { name, year }
    }
}

fn main() {
    example_03();
}

fn example_01() {
    {
        let data = &[97, 'b' as u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut file = File::create("test.dat").unwrap();
        file.write_all(data).unwrap();
    }

    {
        let mut file = File::open("test.dat").unwrap();
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();
        println!("{:?}", buffer);
    }

}

fn example_02() {
    {
        let txt = "fasza".to_string();
        let txt = txt.into_bytes();
        let mut file = std::fs::File::create("data.02.dat").unwrap();
        file.write(&txt).unwrap();
    }

    {
        let mut file = std::fs::File::open("data.02.dat").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        println!("text raw: {:?}", buf);
        println!("text String: {:?}", String::from_utf8(buf));
    }

}

fn example_03() {
    let file_name = String::from("person.dat");
    let name = String::from("Longname De La Rust");
    let year = 1983_u16;

    {
        let person = Person { name: name.clone(), year: year.clone() };
        println!("person serialized: {:?}", person.to_vec());
        let mut file = std::fs::File::create(&file_name).unwrap();
        file.write_all(&person.to_vec()).unwrap();
    }

    {
        let mut buf = Vec::<u8>::new();
        let mut file = std::fs::File::open(&file_name).unwrap();
        file.read_to_end(&mut buf).unwrap();
        println!("read buf len: {}", &buf.len());
        let person = Person::from_bytes(buf);
        println!("name: {}", person.name);
        println!("year: {}", person.year);
        assert_eq!(person.name, name);
        assert_eq!(person.year, year);
    }
}
