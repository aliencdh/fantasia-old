use color_eyre::eyre;
use drawing_algorithms::*;
use image::codecs::tga::TgaEncoder;
use image::GenericImage;
use obj_reader::Model;
use std::fs::File;
use std::io::BufWriter;
use std::str::FromStr;

mod drawing_algorithms;
mod obj_reader;

const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;

fn main() -> eyre::Result<()> {
    // draw a black image
    let mut image =
        image::DynamicImage::ImageRgba8(image::RgbaImage::new(WIDTH as u32, HEIGHT as u32));

    for x in 0..(WIDTH as u32) {
        for y in 0..(HEIGHT as u32) {
            image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }

    // load obj
    let model = std::fs::read_to_string("obj/head.obj")
        .map_err(|err| eyre::eyre!("{err:?}"))
        .and_then(|src| Model::from_str(&src))?;

    // draw
    for face in model.faces {
        for i in 0..3 {
            let v0 = model.vertices[face.indices[i] - 1];
            let v1 = model.vertices[face.indices[(i + 1) % 3] - 1];

            let x0 = ((v0.x + 1f32) * ((WIDTH-1) as f32) / 2f32) as i32;
            let y0 = ((v0.y + 1f32) * ((HEIGHT-1) as f32) / 2f32) as i32;
            let x1 = ((v1.x + 1f32) * ((WIDTH-1) as f32) / 2f32) as i32;
            let y1 = ((v1.y + 1f32) * ((HEIGHT-1) as f32) / 2f32) as i32;

            line(x0, y0, x1, y1, &mut image, WHITE);
        }
    }

    // encode and write to file
    let writer = BufWriter::new(File::create("output.tga")?);
    let encoder = TgaEncoder::new(writer);

    encoder.encode(
        image.flipv().as_bytes(),
        WIDTH as u32,
        HEIGHT as u32,
        image::ColorType::Rgba8,
    )?;

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl From<&[f32]> for Vec3 {
    fn from(value: &[f32]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
