use crate::{
    objects::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    /// (P−C)⋅(P−C)=r²
    /// ((O+tD)−C)⋅((O+tD)−C)=r²
    /// ========================> oc=O−C
    /// (oc+tD)⋅(oc+tD)=r²
    /// ========================> (a+b)² = a² + 2ab + b²
    /// oc² + 2t(oc * D) + t²*D² = r²
    /// ========================> at² + bt + c = 0
    /// (D⋅D)t² + 2(oc⋅D)t + (oc⋅oc−r2) = 0
    /// a = D⋅D
    /// b = 2(oc⋅D)
    /// c = oc²−r²
    ///
    /// Δ=b²−4ac
    ///
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center; // O - C
        let a = ray.direction.dot(&ray.direction); // D²
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4. * a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2. * a);
        let t2 = (-b + sqrt_d) / (2. * a);

        let t = if t1 > 0. {
            t1
        } else if t2 > 0. {
            t2
        } else {
            return None;
        };

        let point = ray.origin + ray.direction * t;
        let normal = (point - self.center).normalize();

        Some(HitRecord { t, point, normal })
    }
}
