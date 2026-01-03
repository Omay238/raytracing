use libpbm::NetPBM;
use raytracer::Vec3;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    let mut img = NetPBM::new_ppm(image_width, image_height, 255);

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let color = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            )
            .color(255);

            img.set_pixel(i, j, color);
        }
    }

    img.save_raw("render.ppm").expect("Failed to save image!");
}
