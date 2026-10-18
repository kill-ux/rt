use std::io::{BufWriter, Write, stdout};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rt::{
    camera::Camera,
    color::Color,
    light::Light,
    objects::{plane::Plane, sphere::Sphere},
    renderer::trace,
    scene::Scene,
    vec3::Vec3,
};

use clap::Parser;

/// A ray tracer that writes a PPM image to stdout
#[derive(Parser)]
#[command(name = "rt")]
struct Config {
    /// Image width in pixels
    #[arg(short = 'w', long, default_value_t = 200)]
    width: usize,

    /// Image height in pixels
    #[arg(short = 'H', long, default_value_t = 150)]
    height: usize,

    /// Samples per pixel (more = smoother but slower)
    #[arg(short, long, default_value_t = 50)]
    samples: u32,

    /// sky light
    #[arg(short, long, default_value_t = 0.3)]
    sky: f64
}

fn main() {
    let cfg = Config::parse();
    let (width, height, samples, sky_strength) = (cfg.width, cfg.height, cfg.samples, cfg.sky);

    let mut scene = Scene::new(Light::new(Vec3::new(5.0, 5.0, -5.0), 1.0));

    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -5.0),
        1.0,
        Color::new(0.8, 0.1, 0.1),
    ))); // the ball1
    scene.add(Box::new(Plane::new(
        Vec3::new(0., -1., 0.),
        Vec3::new(0., 1., 0.),
        Color::new(0.8, 0.8, 0.8),
    )));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        width,
        height,
    );

    let max_depth = 5;

    let rows: Vec<Vec<[u8; 3]>> = (0..height)
        .into_par_iter()
        .map(|py| {
            (0..width)
                .map(|px| {
                    let mut sum = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..samples {
                        let ray = camera.get_ray(
                            px as f64 + rand::random::<f64>(),
                            py as f64 + rand::random::<f64>(),
                        );
                        sum = sum + trace(&ray, &scene, max_depth, sky_strength);
                    }

                    let c = sum * (1.0 / samples as f64);

                    [
                        (c.x.sqrt().min(1.0) * 255.0) as u8,
                        (c.y.sqrt().min(1.0) * 255.0) as u8,
                        (c.z.sqrt().min(1.0) * 255.0) as u8,
                    ]
                })
                .collect()
        })
        .collect();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());
    writeln!(out, "P3\n{} {}\n255", width, height).unwrap();
    for row in &rows {
        for p in row {
            writeln!(out, "{} {} {}", p[0], p[1], p[2]).unwrap();
        }
    }
}

// for py in 0..height {
//         for px in 0..width {
//             let mut sum = Vec3::ZERO;

//             for _ in 0..samples {
//                 let ray = camera.get_ray(px as f64 + random::<f64>(), py as f64 + random::<f64>());
//                 sum = sum + trace(&ray, &scene, max_depth);
//             }

//             let color = sum * (1. / samples as f64);

//             // gamma correction: linear -> display
//             let r = (color.x.sqrt().min(1.0) * 255.) as u8;
//             let g = (color.y.sqrt().min(1.0) * 255.) as u8;
//             let b = (color.z.sqrt().min(1.0) * 255.) as u8;

//             println!("{} {} {}", r, g, b);

//             //     match scene.closest_hit(&ray) {
//             //         Some(hit) => {
//             //             let shadow_origin = hit.point + hit.normal * 1e-4;
//             //             let brightness = if scene.is_shadowed(shadow_origin) {
//             //                 0.
//             //             } else {
//             //                 let light_dir = (scene.light.position - hit.point).normalize();
//             //                 hit.normal.dot(&light_dir).max(0.) * scene.light.intensity
//             //             };

//             //             let r = (brightness * hit.color.x * 255.).min(255.) as u32;
//             //             let g = (brightness * hit.color.y * 255.).min(255.) as u32;
//             //             let b = (brightness * hit.color.z * 255.).min(255.) as u32;
//             //             println!("{} {} {}", r, g, b);

//             //             // let d = (hit.normal + Vec3::random_unit()).normalize();
//             //             // let r = ((d.x * 0.5 + 0.5) * 255.0) as u8;
//             //             // let g = ((d.y * 0.5 + 0.5) * 255.0) as u8;
//             //             // let b = ((d.z * 0.5 + 0.5) * 255.0) as u8;
//             //             // println!("{} {} {}", r, g, b);
//             //             // tln!("255 255 255");
//             //         }
//             //         None => println!("0 0 0"),
//             //     }

//             //     // match sphere.hit(&ray) {
//             //     //     Some(hit) => {
//             //     //         let l prinight_dir = (light.position - hit.point).normalize();
//             //     //         let brightness = hit.normal.dot(&light_dir).max(0.) * light.intensity;

//             //     //         let c = (brightness * 255.).min(255.) as u8;
//             //     //         println!("{} {} {}", c, c, c);
//             //     //     }
//             //     //     None => {
//             //     //         println!("0 0 0")
//             //     //     }
//             //     // }
//         }
//     }
