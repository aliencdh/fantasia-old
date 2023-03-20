use color_eyre::eyre;
use image::codecs::tga::TgaEncoder;
use image::GenericImage;
use std::fs::File;
use std::io::BufWriter;

fn main() -> eyre::Result<()> {
    let mut image = image::DynamicImage::ImageRgba8(image::RgbaImage::new(100, 100));

    for x in 0..100 {
        for y in 0..100 {
            image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }

    image.put_pixel(52, 41, image::Rgba([255, 0, 0, 255]));

    let writer = BufWriter::new(File::create("output.tga")?);
    let encoder = TgaEncoder::new(writer);

    encoder.encode(image.flipv().as_bytes(), 100, 100, image::ColorType::Rgba8)?;

    Ok(())
}
