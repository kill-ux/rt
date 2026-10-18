use crate::{Scene, Vec3, ray::Ray};

pub fn trace(ray: &Ray, scene: &Scene, depth: u32, sky_strength: f64) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    match scene.closest_hit(ray) {
        Some(hit) => {
            // direct light
            let shadow_origin = hit.point + hit.normal * 1e-4;
            let direct = if scene.is_shadowed(shadow_origin) {
                0.0
            } else {
                let light_dir = (scene.light.position - hit.point).normalize();
                hit.normal.dot(&light_dir).max(0.0) * scene.light.intensity
            };

            // indirect light
            let mut dir = hit.normal + Vec3::random_unit();
            if dir.dot(&dir) < 1e-12 {
                dir = hit.normal
            }

            // start slightly off the surface to avoid hitting ourselves
            let origin = hit.point + hit.normal * 1e-4;
            let bounced = Ray::new(origin, dir);
            let indirect = trace(&bounced, scene, depth - 1, sky_strength);

            hit.color.mul_elem(Vec3::new(direct, direct, direct) + indirect)
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
