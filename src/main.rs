use image::RgbImage;
use raytracing::lib_core::{color::*, point::Point3, ray::Ray, vec::Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio) as u32;

    // Camera
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut image = RgbImage::new(image_width, image_height);
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * f64::from(i)) + (pixel_delta_v * f64::from(j));
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            write_color(&mut image, i, j, &pixel_color);
        }
    }
    image.save("output.png").unwrap();
    println!("\nDone.");
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(center, 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - center).unit_vector();
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
