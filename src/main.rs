use raytracer::Camera;
use raytracer::HittableList;
use raytracer::Sphere;
use raytracer::Vec3;

fn main() {
    let mut world = HittableList::new();

    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new_i32(0, 0, -1), 0.5)));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}
