use image::io::Reader as ImageReader;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let img = ImageReader::open("images/noaa-IooaujyvkVk-unsplash.jpg")?.decode()?;
    // let img = ImageReader::open("images/noaa-IooaujyvkVk-unsplash.jpg")?;
    let (width, height) = img.d
    Ok(())
}
