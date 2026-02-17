use image::RgbImage;
use raytracing::color::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut image = RgbImage::new(image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                f64::from(i) / f64::from(image_width - 1),
                f64::from(j) / f64::from(image_height - 1),
                0.0,
            );
            write_color(&mut image, i, j, &pixel_color);
        }
    }
    image.save("output.png").unwrap();
    println!("\nDone.");
}
