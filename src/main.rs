use image::{Rgb, RgbImage};
use raytracing::vec::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let a = Vec3::new(2.0, 3.9, 4.9);
    println!("aaa {}", a.x());

    let mut image = RgbImage::new(image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            image.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }
    image.save("output.png").unwrap();
    println!("\nDone.");
}
