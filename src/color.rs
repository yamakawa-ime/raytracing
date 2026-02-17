use image::{Rgb, RgbImage};

use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(image: &mut RgbImage, i: u32, j: u32, pixel_color: &Color) {
    let ir = (255.999 * pixel_color.x()) as u8;
    let ig = (255.999 * pixel_color.y()) as u8;
    let ib = (255.999 * pixel_color.z()) as u8;

    image.put_pixel(i, j, Rgb([ir, ig, ib]));
}
