trait PixelValue {}

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
        for _value in chan.values {
            //
            // THIS RUNS, BUT CAN NOT GET VALUES OUT OF "dyn PixelValue"
            //
            // print!("{}", value);
        }
    }
}
