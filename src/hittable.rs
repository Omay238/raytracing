use crate::Material;
use crate::Ray;
use crate::Vec3;

#[derive(Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = ray.direction.dot(outward_normal) <= 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: std::ops::Range<f64>, hit_record: &mut HitRecord) -> bool;
}
