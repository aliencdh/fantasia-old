use color_eyre::eyre;
use drawing_algorithms::*;
use image::codecs::tga::TgaEncoder;
use image::GenericImage;
use std::fs::File;
use std::io::BufWriter;

mod drawing_algorithms;

const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() -> eyre::Result<()> {
    // draw a black image
    let mut image = image::DynamicImage::ImageRgba8(image::RgbaImage::new(100, 100));

    for x in 0..100 {
        for y in 0..100 {
            image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }

    // load obj
    let (models, _) = tobj::load_obj("obj/head.obj", &tobj::LoadOptions::default())?;
    let model = models
        .get(0)
        .ok_or(eyre::eyre!("Couldn't get the model."))?;

    // draw
    let mut next_face = 0;
    for face in 0..model.mesh.face_arities.len() {
        let end = next_face + model.mesh.face_arities[face] as usize;

        let face_indices = &model.mesh.indices[next_face..end];

        for i in (0..7).step_by(3) {
            let v0 = Vec3::from(
                &model.mesh.positions[(face_indices[i] as usize - 1)..(face_indices[i + 2] as usize - 1)],
            );
            let v1 = Vec3::from(
                &model.mesh.positions
                    [(face_indices[(i + 3) % 9] as usize - 1)..(face_indices[i + 5] as usize - 1)],
            );

            let x0 = (v0.x + 1f32) * (WIDTH / 2) as f32;
            let y0 = (v0.y + 1f32) * (HEIGHT / 2) as f32;
            let x1 = (v1.x + 1f32) * (WIDTH / 2) as f32;
            let y1 = (v1.y + 1f32) * (HEIGHT / 2) as f32;

            line(
                x0 as i32, y0 as i32, x1 as i32, y1 as i32, &mut image, WHITE,
            );
        }
        next_face = end;
    }

    // encode and write to file
    let writer = BufWriter::new(File::create("output.tga")?);
    let encoder = TgaEncoder::new(writer);

    encoder.encode(image.flipv().as_bytes(), 100, 100, image::ColorType::Rgba8)?;

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
            z: value[3],
        }
    }
}
