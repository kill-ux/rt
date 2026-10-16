use rt::{
    camera::Camera,
    light::Light,
    objects::{Hittable, sphere::Sphere},
    ray::Ray,
    scene::Scene,
    vec3::Vec3,
};

fn main() {
    let width = 200;
    let height = 150;
    let mut scene = Scene::new(Light::new(Vec3::new(2.0, 2.0, -5.0), 1.0));
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0))); // the ball1
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, -101.0, -5.0), 100.0))); // fake floor
    scene.add(Box::new(Sphere::new(
        Vec3::new(4.12132034, 4.12132034, -5.0),
        1.0,
    ))); // ball2

    //  scene.add(Box::new(Sphere::new(
    //     Vec3::new(5.14495755, 5.85994341, 0.14495755),
    //     1.0,
    // )));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        width,
        height,
    );

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for py in 0..height {
        for px in 0..width {
            let ray = camera.get_ray(px, py);

            match scene.closest_hit(&ray) {
                Some(hit) => {
                    
                    let shadow_origin = hit.point + hit.normal * 1e-4;
                    let brightness = if scene.is_shadowed(shadow_origin, hit.normal) {
                        0.
                    } else {
                        let light_dir = (scene.light.position - hit.point).normalize();
                        hit.normal.dot(&light_dir).max(0.) * scene.light.intensity
                    };

                    let c = (brightness * 255.).min(255.) as u8;
                    println!("{} {} {}", c, c, c);

                    //  println!("255 255 255");
                }
                None => println!("0 0 0"),
            }

            // match sphere.hit(&ray) {
            //     Some(hit) => {
            //         let light_dir = (light.position - hit.point).normalize();
            //         let brightness = hit.normal.dot(&light_dir).max(0.) * light.intensity;

            //         let c = (brightness * 255.).min(255.) as u8;
            //         println!("{} {} {}", c, c, c);
            //     }
            //     None => {
            //         println!("0 0 0")
            //     }
            // }
        }
    }
}
