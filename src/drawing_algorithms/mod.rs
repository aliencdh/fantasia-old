use image::{DynamicImage, GenericImage};

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut DynamicImage, color: image::Rgba<u8>) {
    for t in 0..100 {
        // graph the line by "traveling" along its trajectory
        let t = t as f32 / 100f32;
        let x = x0 as f32 + (x1 - x0) as f32 * t;
        let y = y0 as f32 + (y1 - y0) as f32 * t;

        image.put_pixel(x as u32, y as u32, color);
    }
}
