use crate::{
    light::Light,
    objects::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub light: Light,
}

impl Scene {
    pub fn new(light: Light) -> Self {
        Self {
            objects: Vec::new(),
            light,
        }
    }

    pub fn add(&mut self, objcet: Box<dyn Hittable>) {
        self.objects.push(objcet);
    }

    // pub fn closest_hit(&self, ray: &Ray) -> Option<HitRecord> {
    //     let mut closest: Option<HitRecord> = None;
    //     for obj in &self.objects {
    //         if let Some(hit) = obj.hit(ray)
    //             && closest.as_ref().map_or(true, |c| hit.t < c.t)
    //         {
    //             closest = Some(hit)
    //         }
    //     }
    //     closest
    // }

    pub fn closest_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest_t = f64::INFINITY;
        let mut closest: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray) {
                if hit.t > 0.0 && hit.t < closest_t {
                    closest_t = hit.t;
                    closest = Some(hit);
                }
            }
        }

        closest
    }

    pub fn is_shadowed(&self, point: Vec3) -> bool {
        let to_light = self.light.position - point;
        let distance = to_light.length();
        let shadow_ray = Ray::new(point, to_light.normalize());
        match self.closest_hit(&shadow_ray) {
            Some(h) => h.t < distance,
            None => false,
        }
    }
}
