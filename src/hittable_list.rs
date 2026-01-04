use crate::HitRecord;
use crate::Hittable;
use crate::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: std::ops::Range<f64>, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.end;

        for object in &self.objects {
            let mut temp_record = HitRecord::default();
            if object.hit(ray, ray_t.start..closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}
