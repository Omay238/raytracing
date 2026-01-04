use crate::HitRecord;
use crate::Hittable;
use crate::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: std::ops::Range<f64>, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.end;

        for object in &self.objects {
            if object.hit(ray, ray_t.start..closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}
