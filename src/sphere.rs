use crate::Hittable;
use crate::{HitRecord, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius: 0f64.max(radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: std::ops::Range<f64>, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;

        if !ray_t.contains(&root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(&root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        true
    }
}
