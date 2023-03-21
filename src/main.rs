use color_eyre::eyre;
use drawing_algorithms::*;
use image::codecs::tga::TgaEncoder;
use image::GenericImage;
use obj_reader::Model;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Div, Mul, Sub};
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
    let light_direction = Vec3::new(0f32, 0f32, -1f32);
    for face in model.faces {
        let world_coords = face
            .indices
            .iter()
            .map(|i| model.vertices[i - 1])
            .collect::<Vec<_>>();

        let screen_coords = world_coords
            .iter()
            .map(|vertex| {
                (
                    ((vertex.x + 1f32) * (DIMENSION - 1) as f32 / 2f32) as i32,
                    ((vertex.y + 1f32) * (DIMENSION - 1) as f32 / 2f32) as i32,
                )
            })
            .collect::<Vec<_>>();

        let normal = (world_coords[2] - world_coords[0])
            .cross(world_coords[1] - world_coords[0])
            .normalize();

        let light_intensity = normal.dot(light_direction);

        if light_intensity > 0f32 {
            filled_triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
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
        DIMENSION as u32,
        DIMENSION as u32,
        image::ColorType::Rgba8,
    )?;

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(self) -> Self {
        self / self.abs()
    }

    fn abs(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
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
impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1f32 / rhs)
    }
}
