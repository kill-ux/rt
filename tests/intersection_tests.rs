#[cfg(test)]
mod tests {
    use rt::{
        objects::{Hittable, sphere::Sphere},
        ray::Ray,
        vec3::Vec3,
    };

    #[test]
    fn ray_hits_sphere_and_returns_nearest_t() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray);

        assert!(hit.is_some());

        let t = hit.unwrap();

        // The ray enters at z = -4, so t must be 4.
        assert!((t - 4.0).abs() < 0.000_001);
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        let hit = sphere.hit(&ray);

        assert_eq!(hit, None);
    }

    #[test]
    fn ray_starting_inside_sphere_uses_t2() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        let hit = sphere.hit(&ray);

        assert!(hit.is_some());

        let t = hit.unwrap();

        // t1 = -1: behind ray origin.
        // t2 =  1: the ray exits the sphere here.
        assert!((t - 1.0).abs() < 0.000_001);
    }
    

    #[test]
    fn ray_far_from_sphere_returns_nearest_hit() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -10.0), 1.0);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray);

        assert!(hit.is_some());

        let t = hit.unwrap();

        // Near surface is at z = -9.
        // With direction (0, 0, -1), P(t) = (0, 0, -t).
        // P(9) = (0, 0, -9): first point that touches the sphere.
        assert!((t - 9.0).abs() < 0.000_001);
    }
}
