trait Pixel {
    fn to_chan(&self);
}

struct Exr<'a> {
    channels: Vec<&'a dyn Pixel>,
}

struct Channel<T> {
    name: String,
    pixels: Vec<T>,
}

impl<T> Pixel for Channel<T>
where
    T: std::fmt::Display + std::fmt::Debug
{
    fn to_chan(&self) {
        for p in &self.pixels {
            println!("{}, {:?}", p, p);
        }
    }
}

fn main() {
    let chan_a = Channel { name: "a".to_string(), pixels: vec![1u8, 2u8, 3u8] };
    let chan_b = Channel { name: "b".to_string(), pixels: vec![1.0f32, 2.0f32, 3.0f32] };
    let exr = Exr { channels: vec![&chan_a, &chan_b] };

    let ch = exr.channels[0].to_chan();
    let ch = exr.channels[1].to_chan();
}
