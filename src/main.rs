use libpbm::NetPBM;
use raytracer::HitRecord;
use raytracer::Hittable;
use raytracer::HittableList;
use raytracer::Ray;
use raytracer::Sphere;
use raytracer::Vec3;

fn ray_color(ray: &Ray, world: &HittableList) -> [u16; 3] {
    let mut record = HitRecord::default();
    if world.hit(*ray, 0.0..f64::INFINITY, &mut record) {
        return (0.5 * (record.normal + Vec3::new_i32(1, 1, 1))).color(255);
    }
    let unit_direction = ray.direction.normal();
    let a = 0.5 * (unit_direction.y + 1.0);
    ((1.0 - a) * Vec3::new_i32(1, 1, 1) + a * Vec3::new(0.5, 0.7, 1.0)).color(255)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // World

    let mut world = HittableList::new();

    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new_i32(0, 0, -1), 0.5)));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new_i32(0, 0, 0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    let mut img = NetPBM::new_ppm(image_width, image_height, 255);

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);
            img.set_pixel(i, j, color);
        }
    }

    img.save_raw("render.ppm").expect("Failed to save image!");
}
