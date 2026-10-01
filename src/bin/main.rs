use rt::{
    camera::Camera,
    light::Light,
    objects::{Hittable, sphere::Sphere},
    vec3::Vec3,
};

fn main() {
    let width = 200;
    let height = 150;
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);
    let light = Light::new(Vec3::new(2.0, 2.0, 0.0), 1.0);

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
            match sphere.hit(&ray) {
                Some(hit) => {
                    let light_dir = (light.position - hit.point).normalize();
                    let brightness = hit.normal.dot(&light_dir).max(0.) * light.intensity;

                    let c = (brightness * 255.).min(255.) as u8;
                    println!("{} {} {}", c, c, c);
                }
                None => {
                    println!("0 0 0")
                },
            }
        }
    }
}
