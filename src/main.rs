use libpbm::NetPBM;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    let mut img = NetPBM::new_ppm(image_width, image_height, 255);

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            img.set_pixel(
                i,
                j,
                [
                    (r * 255.999) as u16,
                    (g * 255.999) as u16,
                    (b * 255.999) as u16,
                ],
            );
        }
    }

    img.save_raw("render.ppm").expect("Failed to save image!");
}
