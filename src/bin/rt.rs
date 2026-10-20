use std::io::{BufWriter, Write, stdout};

use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rt::{
    Config,
    camera::Camera,
    color::Color,
    light::Light,
    objects::{cube::Cube, plane::Plane, sphere::Sphere},
    renderer::trace,
    scene::Scene,
    vec3::Vec3,
};

fn main() {
    let cfg = Config::parse();

    let mut scene = Scene::new(Light::new(Vec3::new(5.0, 5.0, -5.0), 1.0));

    scene.add(Box::new(Sphere::new(
        Vec3::new(-2.0, 0.0, -5.0),
        1.0,
        Color::new(0.8, 0.1, 0.1),
    ))); // the ball1

    scene.add(Box::new(Cube::new(
        Vec3::new(2., 0., -6.),
        2.,
        Color::new(0.1, 0.3, 0.8),
    )));

    scene.add(Box::new(Plane::new(
        Vec3::new(0., -1., 0.),
        Vec3::new(0., 1., 0.),
        Color::new(0.8, 0.8, 0.8),
    )));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        cfg.width,
        cfg.height,
    );

    let max_depth = 5;

    let rows: Vec<Vec<[u8; 3]>> = (0..cfg.height)
        .into_par_iter()
        .map(|py| {
            (0..cfg.width)
                .map(|px| {
                    let mut sum = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..cfg.samples {
                        let ray = camera.get_ray(
                            px as f64 + rand::random::<f64>(),
                            py as f64 + rand::random::<f64>(),
                        );
                        sum = sum + trace(&ray, &scene, max_depth, cfg.sky);
                    }

                    let c = sum * (1.0 / cfg.samples as f64);

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
    writeln!(out, "P3\n{} {}\n255", cfg.width, cfg.height).unwrap();
    for row in &rows {
        for p in row {
            writeln!(out, "{} {} {}", p[0], p[1], p[2]).unwrap();
        }
    }
}

// use std::io::{BufWriter, Write, stdout};

// use clap::Parser;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use rt::{
//     Config,
//     camera::Camera,
//     color::Color,
//     light::Light,
//     objects::{cube::Cube, plane::Plane, sphere::Sphere},
//     renderer::trace,
//     scene::Scene,
//     vec3::Vec3,
// };

// fn main() {
//     let cfg = Config::parse();
//     let mut scene = Scene::new(Light::new(Vec3::new(0.0, 3.0, -1.0), 0.2));

//     scene.add(Box::new(Cube::new(
//         Vec3::new(0.0, 0.0, -5.0),
//         12.0,
//         Color::new(0.5, 0.5, 0.5),
//     )));

//     scene.add(Box::new(Sphere::new(
//         Vec3::new(0.0, -4.8, -5.0), // -6.0 + 0.2
//         1.,
//         Color::new(0.8, 0.1, 0.1),
//     )));

//     // let camera = Camera::new(
//     //     Vec3::new(0.0, -2.0, -1.0),
//     //     Vec3::new(0.0, 0.0, -1.0),
//     //     Vec3::new(0.0, 1.0, 0.0),
//     //     cfg.width,
//     //     cfg.height,
//     // );

//     let camera = Camera::new(
//         Vec3::new(0.0, -3.0, -1.0),
//         Vec3::new(0.0, -3.0, -5.0),
//         Vec3::new(0.0, 1.0, 0.0),
//         cfg.width,
//         cfg.height,
//     );

//     let max_depth = 8;

//     let rows: Vec<Vec<[u8; 3]>> = (0..cfg.height)
//         .into_par_iter()
//         .map(|py| {
//             (0..cfg.width)
//                 .map(|px| {
//                     let mut sum = Vec3::new(0.0, 0.0, 0.0);

//                     for _ in 0..cfg.samples {
//                         let ray = camera.get_ray(
//                             px as f64 + rand::random::<f64>(),
//                             py as f64 + rand::random::<f64>(),
//                         );
//                         sum = sum + trace(&ray, &scene, max_depth, cfg.sky);
//                     }

//                     let c = sum * (1.0 / cfg.samples as f64);

//                     [
//                         (c.x.sqrt().min(1.0) * 255.0) as u8,
//                         (c.y.sqrt().min(1.0) * 255.0) as u8,
//                         (c.z.sqrt().min(1.0) * 255.0) as u8,
//                     ]
//                 })
//                 .collect()
//         })
//         .collect();

//     let stdout = stdout();
//     let mut out = BufWriter::new(stdout.lock());
//     writeln!(out, "P3\n{} {}\n255", cfg.width, cfg.height).unwrap();

//     for row in &rows {
//         for p in row {
//             writeln!(out, "{} {} {}", p[0], p[1], p[2]).unwrap();
//         }
//     }
// }
