use crate::{Color, Vec3};

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub color: Color,
    pub reflectivity: f64
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, color: Color) -> Self {
        Self {
            center,
            radius,
            height,
            color,
            reflectivity: 0.
        }
    }

    pub fn with_reflectivity(mut self, r: f64) -> Self {
        self.reflectivity =  r.clamp(0.0, 1.);
        self
    }
}

impl super::Hittable for Cylinder {
    fn hit(&self, ray: &crate::ray::Ray) -> Option<super::HitRecord> {
        let oc_x = ray.origin.x - self.center.x;
        let oc_z = ray.origin.z - self.center.z;

        let dx = ray.direction.x;
        let dz = ray.direction.z;

        let half_height = self.height / 2.0;
        let y_min = self.center.y - half_height;
        let y_max = self.center.y + half_height;

        let mut best_t = f64::INFINITY;
        let mut best_normal = Vec3::ZERO;

        // Side (skipped when the ray is parallel to the axis: a ~ 0)
        let a = dx * dx + dz * dz;
        if a > 1e-12 {
            let b = 2.0 * (oc_x * dx + oc_z * dz);
            let c = oc_x * oc_x + oc_z * oc_z - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                return None; // misses the infinite cylinder, so it misses the caps too
            }

            let sqrt_d = discriminant.sqrt();
            let mut t_side = (-b - sqrt_d) / (2.0 * a);
            if t_side <= 1e-6 {
                t_side = (-b + sqrt_d) / (2.0 * a);
            }

            if t_side > 1e-6 {
                let side_y = ray.origin.y + ray.direction.y * t_side;
                if side_y >= y_min && side_y <= y_max {
                    let p = ray.origin + ray.direction * t_side;
                    best_t = t_side;
                    best_normal = Vec3::new(
                        (p.x - self.center.x) / self.radius,
                        0.0,
                        (p.z - self.center.z) / self.radius,
                    );
                }
            }
        }

        // Bottom cap
        if ray.direction.y.abs() > 1e-12 {
            let t_bottom = (y_min - ray.origin.y) / ray.direction.y;
            if t_bottom > 1e-6 && t_bottom < best_t {
                let p = ray.origin + ray.direction * t_bottom;
                let px = p.x - self.center.x;
                let pz = p.z - self.center.z;
                if px * px + pz * pz <= self.radius * self.radius {
                    best_t = t_bottom;
                    best_normal = Vec3::new(0.0, -1.0, 0.0);
                }
            }
        }

        // Top cap
        if ray.direction.y.abs() > 1e-12 {
            let t_top = (y_max - ray.origin.y) / ray.direction.y;
            if t_top > 1e-6 && t_top < best_t {
                let p = ray.origin + ray.direction * t_top;
                let px = p.x - self.center.x;
                let pz = p.z - self.center.z;
                if px * px + pz * pz <= self.radius * self.radius {
                    best_t = t_top;
                    best_normal = Vec3::new(0.0, 1.0, 0.0);
                }
            }
        }

        if best_t == f64::INFINITY {
            return None;
        }

        Some(super::HitRecord {
            t: best_t,
            point: ray.origin + ray.direction * best_t,
            normal: best_normal,
            color: self.color,
            reflectivity: self.reflectivity
        })
    }
}
