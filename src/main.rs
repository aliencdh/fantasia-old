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
const GREEN: image::Rgba<u8> = image::Rgba([0, 255, 0, 255]);

const DIMENSION: i32 = 1024;

fn main() -> eyre::Result<()> {
    // draw a black image
    let mut image =
        image::DynamicImage::ImageRgba8(image::RgbaImage::new(DIMENSION as u32, DIMENSION as u32));

    for x in 0..(DIMENSION as u32) {
        for y in 0..(DIMENSION as u32) {
            image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }

    // load obj
    let model = std::fs::read_to_string("obj/head.obj")
        .map_err(|err| eyre::eyre!("{err:?}"))
        .and_then(|src| Model::from_str(&src))?;

    // draw
    for face in model.faces {
        let vertices = face
            .indices
            .iter()
            .map(|i| model.vertices[i - 1])
            .map(|vertex| {
                (
                    ((vertex.x + 1f32) * (DIMENSION - 1) as f32 / 2f32) as i32,
                    ((vertex.y + 1f32) * (DIMENSION - 1) as f32 / 2f32) as i32,
                )
            })
            .collect::<Vec<_>>();

        filled_triangle(
            vertices[0],
            vertices[1],
            vertices[2],
            &mut image,
            image::Rgba([rand::random(), rand::random(), rand::random(), 255]),
        );
    }

    // encode and write to file
    let writer = BufWriter::new(File::create("output.tga")?);
    let encoder = TgaEncoder::new(writer);

    encoder.encode(
        image.flipv().as_bytes(),
        DIMENSION as u32,
        DIMENSION as u32,
        image::ColorType::Rgba8,
    )?;

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
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
