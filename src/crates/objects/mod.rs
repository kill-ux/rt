pub mod sphere;
use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,  // where the ray hit
    pub normal: Vec3, // surface normal at that point
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
