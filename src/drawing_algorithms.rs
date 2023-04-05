use crate::{Vec3, HEIGHT, WIDTH};
use image::{DynamicImage, GenericImage};

pub fn triangle_3d(
    mut points: [Vec3; 3],
    zbuffer: &mut [[f32; HEIGHT as usize]; WIDTH as usize],
    image: &mut DynamicImage,
    color: image::Rgba<u8>,
) {
    points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    for t in 0..WIDTH.max(HEIGHT) {
        let t = t as f32 / WIDTH.max(HEIGHT) as f32;
        let x1 =
            ((points[0].x + (points[1].x - points[0].x) * t + 1f32) * WIDTH as f32 / 2f32) as i32;
        let y1 =
            ((points[0].y + (points[1].y - points[0].y) * t + 1f32) * HEIGHT as f32 / 2f32) as i32;
        let z1 = points[0].z + (points[1].z - points[0].z) * t;
        let x2 =
            ((points[0].x + (points[2].x - points[0].x) * t + 1f32) * WIDTH as f32 / 2f32) as i32;
        let y2 =
            ((points[0].y + (points[2].y - points[0].y) * t + 1f32) * HEIGHT as f32 / 2f32) as i32;
        let z2 = points[0].z + (points[2].z - points[0].z) * t;

        if zbuffer[x1 as usize][y1 as usize] < z1 {
            zbuffer[x1 as usize][y1 as usize] = z1;
            image.put_pixel(x1 as u32, y1 as u32, color);
        }
        if zbuffer[x2 as usize][y2 as usize] < z2 {
            zbuffer[x2 as usize][y2 as usize] = z1;
            image.put_pixel(x2 as u32, y2 as u32, color);
        }

        rasterize(x1, y1, z1, x2, y2, z2, image, color, zbuffer);
    }
}

pub fn rasterize(
    x0: i32,
    y0: i32,
    z0: f32,
    x1: i32,
    y1: i32,
    z1: f32,
    image: &mut DynamicImage,
    color: image::Rgba<u8>,
    zbuffer: &mut [[f32; HEIGHT as usize]; WIDTH as usize],
) {
    for t in 0..WIDTH.max(HEIGHT) {
        let t = t as f32 / WIDTH.max(HEIGHT) as f32;
        let x = (x0 as f32 + (x1 - x0) as f32 * t) as i32;
        let y = (y0 as f32 + (y1 - y0) as f32 * t) as i32;
        let z = z0 as f32 + (z1 - z0) as f32 * t;

        if zbuffer[x as usize][y as usize] < z {
            zbuffer[x as usize][y as usize] = z;
            image.put_pixel(x as u32, y as u32, color);
        }
    }
}
