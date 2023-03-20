use color_eyre::eyre;
use drawing_algorithms::*;
use image::codecs::tga::TgaEncoder;
use image::GenericImage;
use std::fs::File;
use std::io::BufWriter;

mod drawing_algorithms;

const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

fn main() -> eyre::Result<()> {
    let mut image = image::DynamicImage::ImageRgba8(image::RgbaImage::new(100, 100));

    for x in 0..100 {
        for y in 0..100 {
            image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }

    line(13, 20, 80, 40, &mut image, WHITE);
    line(20, 13, 40, 80, &mut image, RED);
    line(80, 40, 13, 20, &mut image, RED);

    let writer = BufWriter::new(File::create("output.tga")?);
    let encoder = TgaEncoder::new(writer);

    encoder.encode(image.flipv().as_bytes(), 100, 100, image::ColorType::Rgba8)?;

    Ok(())
}
