use color_eyre::eyre;
use drawing_algorithms::*;
use image::codecs::tga::TgaEncoder;
use image::GenericImage;
use obj_reader::Model;
use std::fs::File;
use std::io::BufWriter;
use std::str::FromStr;
use structures::Vec3;

mod drawing_algorithms;
mod obj_reader;
mod structures;

const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);
const GREEN: image::Rgba<u8> = image::Rgba([0, 255, 0, 255]);
const BLUE: image::Rgba<u8> = image::Rgba([0, 0, 255, 255]);

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
    let mut zbuffer = [[f32::NEG_INFINITY; HEIGHT as usize]; WIDTH as usize];

    let light_direction = Vec3::new(0f32, 0f32, -1f32);
    for face in model.faces {
        let world_coords = face
            .indices
            .iter()
            .map(|i| model.vertices[i - 1])
            .collect::<Vec<_>>();

        let normal = (world_coords[2] - world_coords[0])
            .cross(world_coords[1] - world_coords[0])
            .normalize();

        let light_intensity = normal.dot(light_direction);

        if light_intensity > 0f32 {
            triangle_3d(
                world_coords
                    .try_into()
                    .map_err(|err| eyre::eyre!("{err:?}"))?,
                &mut zbuffer,
                &mut image,
                image::Rgba([
                    (light_intensity * 255f32) as u8,
                    (light_intensity * 255f32) as u8,
                    (light_intensity * 255f32) as u8,
                    255,
                ]),
            );
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
