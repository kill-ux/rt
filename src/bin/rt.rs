// use std::io::{BufWriter, Write, stdout};

// use clap::Parser;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use rt::{
//     Config,
//     camera::Camera,
//     color::Color,
//     light::Light,
//     objects::{cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
//     renderer::trace,
//     scene::Scene,
//     vec3::Vec3,
// };

// fn main() {
//     let cfg = Config::parse();
//     let mut scene = Scene::new(Light::new(Vec3::new(0.0, 50.0, -5.0), 1.0 * cfg.brightness));

//     scene.add(Box::new(
//         Sphere::new(Vec3::new(-2.0, 0.0, -5.0), 1.0, Color::new(0.8, 0.1, 0.1))
//             .with_reflectivity(1.0),
//     )); // the ball1

//     scene.add(Box::new(Sphere::new(
//         Vec3::new(-2.0, -0.75, -4.0),
//         0.25,
//         Color::new(0.8, 0.1, 0.1),
//     ))); // the ball1

//     scene.add(Box::new(Sphere::new(
//         Vec3::new(0.0, -0.3, -4.0),
//         0.7,
//         Color::new(0.8, 0.8, 0.1),
//     ))); // the ball1

//     scene.add(Box::new(
//         Cube::new(Vec3::new(2., 0., -6.), 2., Color::new(0.1, 0.3, 0.8)).with_reflectivity(1.),
//     ));

//     // scene.add(Box::new(Cylinder::new(
//     //     Vec3::new(0.0, 2., -6.0),
//     //     1.0,
//     //     3.0,
//     //     Color::new(0.9, 0.5, 0.1),
//     //     1.
//     // )));

//     scene.add(Box::new(Plane::new(
//         Vec3::new(0., -1., 0.),
//         Vec3::new(0., 1., 0.),
//         Color::new(0.8, 0.8, 0.8),
//     )));

//     let camera = Camera::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, -1.0),
//         Vec3::new(0.0, 1.0, 0.0),
//         cfg.width,
//         cfg.height,
//     );

//     // let cfg = Config::parse();
//     let sky = cfg.sky * cfg.brightness * dimming(cfg.scene);

//     // let scene = build_scene(cfg.scene, cfg.brightness, cfg.reflect);

//     // let (def_pos, def_look) = default_camera(cfg.scene);
//     // let pos = cfg.cam_pos.unwrap_or(def_pos);
//     // let look = cfg.look_at.unwrap_or(def_look);

//     // // the camera math breaks when looking straight up or down (we hit this earlier)
//     // let fwd = look - pos;
//     // if fwd.x.abs() < 1e-9 && fwd.z.abs() < 1e-9 {
//     //     eprintln!("error: the camera can't look straight up or down");
//     //     std::process::exit(1);
//     // }

//     // let camera = Camera::new(pos, look, Vec3::new(0.0, 1.0, 0.0), cfg.width, cfg.height);

//     let max_depth = 5;

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
//                         sum = sum + trace(&ray, &scene, max_depth, sky);
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

// fn floor(r: bool) -> Box<Plane> {
//     if r {
//         Box::new(
//             Plane::new(
//                 Vec3::new(0.0, -1.0, 0.0),
//                 Vec3::new(0.0, 1.0, 0.0),
//                 Color::new(0.8, 0.8, 0.8),
//             )
//             .with_reflectivity(0.3),
//         )
//     } else {
//         Box::new(Plane::new(
//             Vec3::new(0.0, -1.0, 0.0),
//             Vec3::new(0.0, 1.0, 0.0),
//             Color::new(0.8, 0.8, 0.8),
//         ))
//     }
// }

// fn dimming(id: u32) -> f64 {
//     if id == 2 { 0.4 } else { 1.0 }
// }

// fn build_scene(id: u32, brightness: f64, reflect: bool) -> Scene {
//     let lamp = Light::new(Vec3::new(5.0, 6.0, -10.0), dimming(id) * brightness);
//     let mut scene = Scene::new(lamp);

//     let ball = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.8, 0.1, 0.1));

//     match id {
//         1 => {
//             scene.add(Box::new(ball));
//         }
//         2 => {
//             scene.add(floor(reflect));
//             scene.add(Box::new(Cube::new(
//                 Vec3::new(0.0, 0.0, -5.0),
//                 2.0,
//                 Color::new(0.1, 0.3, 0.8),
//             )));
//         }
//         3 if reflect => {
//             // scenes 3 and 4 share the same objects
//             scene.add(floor(reflect));
//             scene.add(Box::new(ball.with_reflectivity(1.)));
//             scene.add(Box::new(Cube::new(
//                 Vec3::new(2.5, 0.0, -6.0),
//                 2.0,
//                 Color::new(0.1, 0.3, 0.8),
//             ).with_reflectivity(0.9)));
//             scene.add(Box::new(Cylinder::new(
//                 Vec3::new(-2.5, 0.0, -6.0),
//                 0.7,
//                 2.0,
//                 Color::new(0.1, 0.7, 0.2),
//             ).with_reflectivity(0.2)));
//         }
//         _ => {
//             // scenes 3 and 4 share the same objects
//             scene.add(floor(reflect));
//             scene.add(Box::new(ball));
//             scene.add(Box::new(Cube::new(
//                 Vec3::new(2.5, 0.0, -6.0),
//                 2.0,
//                 Color::new(0.1, 0.3, 0.8),
//             )));
//             scene.add(Box::new(Cylinder::new(
//                 Vec3::new(-2.5, 0.0, -6.0),
//                 0.7,
//                 2.0,
//                 Color::new(0.1, 0.7, 0.2),
//             )));
//         }
//     }
//     scene
// }

// // (camera position, look_at)
// fn default_camera(id: u32) -> (Vec3, Vec3) {
//     let target = Vec3::new(0.0, 0.0, -5.0);
//     match id {
//         2 => (Vec3::new(3.5, 2.0, -1.0), target),
//         4 => (Vec3::new(6.0, 3.0, -1.0), target), // from the right and above
//         _ => (Vec3::new(0.0, 1.5, 3.0), target),  // from the front
//     }
// }

fn rainbow_color(hue: f64) -> Color {
    let h = hue.rem_euclid(1.0) * 6.0;

    let sector = h.floor() as u32;
    let f = h - h.floor();

    match sector {
        0 => Color::new(1.0, f, 0.0),
        1 => Color::new(1.0 - f, 1.0, 0.0),
        2 => Color::new(0.0, 1.0, f),
        3 => Color::new(0.0, 1.0 - f, 1.0),
        4 => Color::new(f, 0.0, 1.0),
        _ => Color::new(1.0, 0.0, 1.0 - f),
    }
}

use std::io::{BufWriter, Write, stdout};

use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rt::{
    Config,
    camera::Camera,
    color::Color,
    light::Light,
    objects::{cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
    renderer::trace,
    scene::Scene,
    vec3::Vec3,
};

fn main() {
    let cfg = Config::parse();

    let mut scene = Scene::new(Light::new(Vec3::new(-8.0, 15.0, 5.0), 1.2 * cfg.brightness));

    // Dark floor: all spheres rest on y = -1.
    scene.add(Box::new(Plane::new(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Color::new(0.08, 0.10, 0.12),
    )));

    // Main mirror sphere.
    scene.add(Box::new(
        Sphere::new(Vec3::new(-1.8, 1.2, -8.0), 2.2, Color::new(1.0, 1.0, 1.0))
            .with_reflectivity(1.0),
    ));

    // Smaller mirror sphere, farther away.
    scene.add(Box::new(
        Sphere::new(Vec3::new(4.0, 0.5, -14.0), 1.5, Color::new(1.0, 1.0, 1.0))
            .with_reflectivity(1.0),
    ));

    // Rainbow rings around the main sphere.
    let ring_center_x = -1.8;
    let ring_center_z = -8.0;

    let ball_radius = 0.14;
    let ball_y = -1.0 + ball_radius;

    // Distance between the centers of neighboring balls.
    let ball_spacing = 0.42;

    for ring in 0..10 {
        let ring_radius = 2.7 + ring as f64 * 0.85;

        let circumference = std::f64::consts::TAU * ring_radius;
        let count = (circumference / ball_spacing).floor() as usize;

        for j in 0..count {
            let fraction = j as f64 / count as f64;
            let angle = fraction * std::f64::consts::TAU;

            let x = ring_center_x + ring_radius * angle.cos();
            let z = ring_center_z + ring_radius * angle.sin();

            // Avoid placing small balls inside the second mirror sphere.
            let dx = x - 4.0;
            let dz = z - (-14.0);

            if dx * dx + dz * dz < 1.85 * 1.85 {
                continue;
            }

            // Repeat the rainbow several times around each ring.
            let hue = (fraction * 4.0 + ring as f64 * 0.08).rem_euclid(1.0);

            scene.add(Box::new(Sphere::new(
                Vec3::new(x, ball_y, z),
                ball_radius,
                rainbow_color(hue),
            )));
        }
    }

    // Elevated camera looking slightly downward.
    // This uses the direction-vector convention of your earlier camera code.
    let camera = Camera::new(
        Vec3::new(0.0, 4.5, 5.0),
        Vec3::new(0.0, -0.25, -1.0).normalize(),
        Vec3::new(0.0, 1.0, 0.0),
        cfg.width,
        cfg.height,
    );

    let max_depth = 5;

    let sky = cfg.sky * cfg.brightness * dimming(cfg.scene);

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
                        sum = sum + trace(&ray, &scene, max_depth, sky);
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

fn floor(r: bool) -> Box<Plane> {
    if r {
        Box::new(
            Plane::new(
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.8, 0.8, 0.8),
            )
            .with_reflectivity(0.3),
        )
    } else {
        Box::new(Plane::new(
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Color::new(0.8, 0.8, 0.8),
        ))
    }
}

fn dimming(id: u32) -> f64 {
    if id == 2 { 0.4 } else { 1.0 }
}

fn build_scene(id: u32, brightness: f64, reflect: bool) -> Scene {
    let lamp = Light::new(Vec3::new(5.0, 6.0, -10.0), dimming(id) * brightness);
    let mut scene = Scene::new(lamp);

    let ball = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.8, 0.1, 0.1));

    match id {
        1 => {
            scene.add(Box::new(ball));
        }
        2 => {
            scene.add(floor(reflect));
            scene.add(Box::new(Cube::new(
                Vec3::new(0.0, 0.0, -5.0),
                2.0,
                Color::new(0.1, 0.3, 0.8),
            )));
        }
        3 if reflect => {
            // scenes 3 and 4 share the same objects
            scene.add(floor(reflect));
            scene.add(Box::new(ball.with_reflectivity(1.)));
            scene.add(Box::new(
                Cube::new(Vec3::new(2.5, 0.0, -6.0), 2.0, Color::new(0.1, 0.3, 0.8))
                    .with_reflectivity(0.9),
            ));
            scene.add(Box::new(
                Cylinder::new(
                    Vec3::new(-2.5, 0.0, -6.0),
                    0.7,
                    2.0,
                    Color::new(0.1, 0.7, 0.2),
                )
                .with_reflectivity(0.2),
            ));
        }
        _ => {
            // scenes 3 and 4 share the same objects
            scene.add(floor(reflect));
            scene.add(Box::new(ball));
            scene.add(Box::new(Cube::new(
                Vec3::new(2.5, 0.0, -6.0),
                2.0,
                Color::new(0.1, 0.3, 0.8),
            )));
            scene.add(Box::new(Cylinder::new(
                Vec3::new(-2.5, 0.0, -6.0),
                0.7,
                2.0,
                Color::new(0.1, 0.7, 0.2),
            )));
        }
    }
    scene
}

// (camera position, look_at)
fn default_camera(id: u32) -> (Vec3, Vec3) {
    let target = Vec3::new(0.0, 0.0, -5.0);
    match id {
        2 => (Vec3::new(3.5, 2.0, -1.0), target),
        4 => (Vec3::new(6.0, 3.0, -1.0), target), // from the right and above
        _ => (Vec3::new(0.0, 1.5, 3.0), target),  // from the front
    }
}
