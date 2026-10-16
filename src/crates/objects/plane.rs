use crate::{
    objects::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

impl Hittable for Plane {
    /// dot(P - point, normal) = 0
    /// [[<< P = O + t·D >>]]
    /// dot(O + t·D - point, normal) = 0
    ///
    /// t = dot(point - O, normal) / dot(D, normal)
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let denom = ray.direction.dot(&self.normal);
        if denom.abs() < 1e-4 {
            return None; // parallel
        }
        let t = (self.point - ray.origin).dot(&self.normal) / denom;

        if t < 1e-6 {
            return None; // behind the ray
        }
        let hit_point = ray.origin + ray.direction * t;

        let normal = if denom < 0. {
            self.normal * -1.
        } else {
            self.normal
        };

        Some(HitRecord {
            t,
            point: hit_point,
            normal,
        })
    }
}
