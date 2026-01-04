use raytracer::Camera;
use raytracer::HittableList;
use raytracer::Sphere;
use raytracer::Vec3;
use raytracer::material::{Dielectric, Lambertian, Metal};

fn main() {
    let mut world = HittableList::default();

    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        Box::new(material_center),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        Box::new(material_bubble),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));

    let camera = Camera::new(
        16.0 / 9.0,
        1600,
        1000,
        100,
        20.0,
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        10.0,
        3.4,
    );
    camera.render(&world);
}
