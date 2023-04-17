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
        let x0 = (points[0].x + (points[1].x - points[0].x) * t + 1f32) * WIDTH as f32 / 2f32;
        let y0 = (points[0].y + (points[1].y - points[0].y) * t + 1f32) * HEIGHT as f32 / 2f32;
        let z0 = points[0].z + (points[1].z - points[0].z) * t;
        let x1 = (points[0].x + (points[2].x - points[0].x) * t + 1f32) * WIDTH as f32 / 2f32;
        let y1 = (points[0].y + (points[2].y - points[0].y) * t + 1f32) * HEIGHT as f32 / 2f32;
        let z1 = points[0].z + (points[2].z - points[0].z) * t;

        if zbuffer[x1 as usize][y1 as usize] < z1 {
            zbuffer[x1 as usize][y1 as usize] = z1;
            image.put_pixel(x1 as u32, y1 as u32, color);
        }
        if zbuffer[x1 as usize][y1 as usize] < z1 {
            zbuffer[x1 as usize][y1 as usize] = z0;
            image.put_pixel(x1 as u32, y1 as u32, color);
        }

        let point0 = Vec3::new(x0, y0, z0);
        let point1 = Vec3::new(x1, y1, z1);
        rasterize(point0, point1, image, color, zbuffer);
    }
}

pub fn rasterize(
    point0: Vec3,
    point1: Vec3,
    image: &mut DynamicImage,
    color: image::Rgba<u8>,
    zbuffer: &mut [[f32; HEIGHT as usize]; WIDTH as usize],
) {
    for t in 0..WIDTH.max(HEIGHT) {
        let t = t as f32 / WIDTH.max(HEIGHT) as f32;
        let x = (point0.x + (point1.x - point0.x) * t) as u32;
        let y = (point0.y + (point1.y - point0.y) * t) as u32;
        let z = point0.z + (point1.z - point0.z) * t;

        if zbuffer[x as usize][y as usize] < z {
            zbuffer[x as usize][y as usize] = z;
            image.put_pixel(x, y, color);
        }
    }
}
