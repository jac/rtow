use super::{HitRecord, Hittable, Material, Point3, Ray};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let mut rec = HitRecord {
                    t: temp,
                    p,
                    material: self.material.clone(),
                    normal: (p - self.center) / self.radius,
                    front_face: false,
                };
                rec.set_face_normal(ray);
                return Some(rec);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let mut rec = HitRecord {
                    t: temp,
                    p,
                    material: self.material.clone(),
                    normal: (p - self.center) / self.radius,
                    front_face: false,
                };
                rec.set_face_normal(ray);
                return Some(rec);
            }
        }
        None
    }
}
