use crate::{Scene, Vec3, ray::Ray};

pub fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    match scene.closest_hit(ray) {
        Some(hit) => {
            let mut dir = hit.normal + Vec3::random_unit();
            if dir.dot(&dir) < 1e-12 {
                dir = hit.normal
            }

            // start slightly off the surface to avoid hitting ourselves
            let origin = hit.point + hit.normal * 1e-4;
            let bounced = Ray::new(origin, dir);
            hit.color.mul_elem(trace(&bounced, scene, depth - 1))
        }
        None => {
            let blue_amount = (ray.direction.y + 1.) / 2.;
            let white_amount = 1. - blue_amount;

            let white = Vec3::new(1., 1., 1.);
            let blue = Vec3::new(0.5, 0.7, 1.);

            return white * white_amount + blue * blue_amount;
        }
    }
}
