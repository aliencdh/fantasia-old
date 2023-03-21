use crate::{Vec3, DIMENSION};
use image::{DynamicImage, GenericImage};

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut DynamicImage, color: image::Rgba<u8>) {
    for t in 0..DIMENSION {
        // graph the line by "traveling" along its trajectory
        let t = t as f32 / DIMENSION as f32;
        let x = x0 as f32 + (x1 - x0) as f32 * t;
        let y = y0 as f32 + (y1 - y0) as f32 * t;

        image.put_pixel(x as u32, y as u32, color);
    }
}

pub fn empty_triangle(
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    image: &mut DynamicImage,
    color: image::Rgba<u8>,
) {
    line(a.0 as i32, a.1 as i32, b.0 as i32, b.1 as i32, image, color);
    line(b.0 as i32, b.1 as i32, c.0 as i32, c.1 as i32, image, color);
    line(c.0 as i32, c.1 as i32, a.0 as i32, a.1 as i32, image, color)
}

pub fn filled_triangle(
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    image: &mut DynamicImage,
    color: image::Rgba<u8>,
) {
    let mut vertices = [a, b, c];
    vertices.sort_by(|x, y| x.1.cmp(&y.1));

    for t in 0..DIMENSION {
        // rasterize the point on the left and right sides
        let t = t as f32 / DIMENSION as f32;
        let x1 = vertices[0].0 as f32 + (vertices[1].0 - vertices[0].0) as f32 * t;
        let y1 = vertices[0].1 as f32 + (vertices[1].1 - vertices[0].1) as f32 * t;
        let x2 = vertices[0].0 as f32 + (vertices[2].0 - vertices[0].0) as f32 * t;
        let y2 = vertices[0].1 as f32 + (vertices[2].1 - vertices[0].1) as f32 * t;

        image.put_pixel(x1 as u32, y1 as u32, color);
        image.put_pixel(x2 as u32, y2 as u32, color);

        // draw a line between the segments
        line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, image, color);
    }
}
