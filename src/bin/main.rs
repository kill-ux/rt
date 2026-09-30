use rt::{
    camera::Camera,
    objects::{Hittable, sphere::Sphere},
    vec3::Vec3,
};

fn main() {
    let width = 200;
    let height = 150;
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0);

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
                Some(_t) => println!("255 255 255"),
                None => println!("0 0 0"),
            }
        }
    }
}
