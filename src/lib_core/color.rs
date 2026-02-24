use image::{Rgb, RgbImage};

use crate::lib_core::{interval::Interval, vec::Vec3};

pub type Color = Vec3;

pub fn write_color(
    image: &mut RgbImage,
    i: u32,
    j: u32,
    pixel_color: &Color,
    samples_per_pixel: u32,
) {
    let scale = 1.0 / f64::from(samples_per_pixel);

    let r = linear_to_gamma(pixel_color.x() * scale);
    let g = linear_to_gamma(pixel_color.y() * scale);
    let b = linear_to_gamma(pixel_color.z() * scale);

    let intensity = Interval::new(0.0, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as u8;
    let ig = (256.0 * intensity.clamp(g)) as u8;
    let ib = (256.0 * intensity.clamp(b)) as u8;

    image.put_pixel(i, j, Rgb([ir, ig, ib]));
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    return linear_component.sqrt();
}
