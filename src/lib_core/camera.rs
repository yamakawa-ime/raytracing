use crate::lib_core::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    point::Point3,
    ray::Ray,
    rtweekend::*,
    vec::Vec3,
};
use image::RgbImage;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        assert!(aspect_ratio > 0.0, "aspect_ratio");
        assert!(image_width > 0, "image_width");
        assert!(samples_per_pixel > 0, "samples_per_pixel");
        assert!(max_depth > 0, "max_depth");

        let image_height = (f64::from(image_width) / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;

        // Viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let mut image = RgbImage::new(self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, self.max_depth, world);
                }
                write_color(&mut image, i, j, &pixel_color, self.samples_per_pixel);
            }
        }
        image.save("output.png").unwrap();
        println!("\nDone.");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc
            + (self.pixel_delta_u * f64::from(i))
            + (self.pixel_delta_v * f64::from(j));
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn ray_color(&self, r: Ray, depth: u32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if world.hit(&r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            let direction = Vec3::random_on_hemisphere(rec.normal);
            return self.ray_color(Ray::new(rec.p, direction), depth - 1, world) * 0.5;
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
