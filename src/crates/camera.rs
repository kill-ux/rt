use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub image_width: usize,
    pub image_height: usize,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        world_up: Vec3,
        image_width: usize,
        image_height: usize,
    ) -> Self {
        let forward = (look_at - origin).normalize();
        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward);

        Self {
            origin,
            forward,
            right,
            up,
            image_width,
            image_height,
        }
    }

    pub fn get_ray(&self, px: usize, py: usize) -> Ray {
        let u = (px as f64 / self.image_width as f64) * 2. - 1.;
        let v = 1. - (py as f64 / self.image_height as f64) * 2.;

        let aspect = self.image_width as f64 / self.image_height as f64;

        let direction = self.forward + self.right * (u * aspect) + self.up * v;

        Ray::new(self.origin, direction.normalize())
    }
}
