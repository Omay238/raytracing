use crate::HitRecord;
use crate::Hittable;
use crate::HittableList;
use crate::Ray;
use crate::Vec3;

use libpbm::NetPBM;
use rand::random;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    image_height: usize,
    pixel_samples_scale: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vec3::zero();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let mut img = NetPBM::new_ppm(self.image_width, self.image_height, 65535);

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut color = Vec3::default();

                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(i, j), self.max_depth, world);
                }

                color *= self.pixel_samples_scale;
                color.color_correct();
                img.set_pixel(i, j, color.color(65535));
            }
        }

        img.save_ascii("render.ppm", None)
            .expect("Failed to save image!");
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, ray: &Ray, depth: usize, world: &HittableList) -> Vec3 {
        if depth == 0 {
            return Vec3::zero();
        }
        let mut record = HitRecord::default();
        if world.hit(*ray, 0.001..f64::INFINITY, &mut record) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::zero();
            if record
                .material
                .scatter(ray, &record, &mut attenuation, &mut scattered)
            {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Vec3::zero();
        }
        let unit_direction = ray.direction.normal();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::one() + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
