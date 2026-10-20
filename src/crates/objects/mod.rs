pub mod plane;
pub mod sphere;
pub mod cube;
use crate::{color::Color, ray::Ray, vec3::Vec3};

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,  // where the ray hit
    pub normal: Vec3, // surface normal at that point
    pub color: Color,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
