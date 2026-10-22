pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;
use crate::{color::Color, ray::Ray, vec3::Vec3};

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,  // where the ray hit
    pub normal: Vec3, // surface normal at that point
    pub color: Color,
    pub reflectivity: f64,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
