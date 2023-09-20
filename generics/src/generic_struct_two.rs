trait PixelValue {}

impl std::fmt::Display for dyn PixelValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PixelValue for u8 {}
impl PixelValue for f32 {}

struct Exr<'a> {
    channels: Vec<Channel<'a>>,
}

struct Channel<'a> {
    values: Vec<&'a dyn PixelValue>,
}

pub fn generic_struct_two() {
    print!("Generic struct two");
    let red = Channel { values: vec![&1_u8, &2_u8, &5_u8] };
    let green = Channel { values: vec![&1.0_f32, &2.0_f32, &5.0_f32] };
    let exr = Exr { channels: vec![red, green] };

    for chan in exr.channels {
        for value in chan.values {
            print!("{}", value);
        }
    }
}
