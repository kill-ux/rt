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
    // let mut scene = Scene::new(Light::new(Vec3::new(0.0, 50.0, -5.0), 1.0 * cfg.brightness));

    // scene.add(Box::new(Sphere::new(
    //     Vec3::new(-2.0, 0.0, -5.0),
    //     1.0,
    //     Color::new(0.8, 0.1, 0.1),
    // ))); // the ball1

    // scene.add(Box::new(Cube::new(
    //     Vec3::new(2., 0., -6.),
    //     2.,
    //     Color::new(0.1, 0.3, 0.8),
    // )));

    // scene.add(Box::new(Cylinder::new(
    //     Vec3::new(0.0, 2., -6.0),
    //     1.0,
    //     3.0,
    //     Color::new(0.9, 0.5, 0.1),
    // )));

    // scene.add(Box::new(Plane::new(
    //     Vec3::new(0., -1., 0.),
    //     Vec3::new(0., 1., 0.),
    //     Color::new(0.8, 0.8, 0.8),
    // )));

    // let camera = Camera::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(0.0, 0.0, -1.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    //     cfg.width,
    //     cfg.height,
    // );

    let cfg = Config::parse();
    let sky = cfg.sky * cfg.brightness * dimming(cfg.scene);

    let scene = build_scene(cfg.scene, cfg.brightness);

    let (def_pos, def_look) = default_camera(cfg.scene);
    let pos = cfg.cam_pos.unwrap_or(def_pos);
    let look = cfg.look_at.unwrap_or(def_look);

    // the camera math breaks when looking straight up or down (we hit this earlier)
    let fwd = look - pos;
    if fwd.x.abs() < 1e-9 && fwd.z.abs() < 1e-9 {
        eprintln!("error: the camera can't look straight up or down");
        std::process::exit(1);
    }

    let camera = Camera::new(pos, look, Vec3::new(0.0, 1.0, 0.0), cfg.width, cfg.height);

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

fn floor() -> Box<Plane> {
    Box::new(Plane::new(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Color::new(0.8, 0.8, 0.8),
    ))
}

fn dimming(id: u32) -> f64 {
    if id == 2 { 0.4 } else { 1.0 }
}

fn build_scene(id: u32, brightness: f64) -> Scene {
    let lamp = Light::new(Vec3::new(5.0, 6.0, -2.0), dimming(id) * brightness);
    let mut scene = Scene::new(lamp);

    let ball = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.8, 0.1, 0.1));

    match id {
        1 => {
            scene.add(Box::new(ball));
        }
        2 => {
            scene.add(floor());
            scene.add(Box::new(Cube::new(
                Vec3::new(0.0, 0.0, -5.0),
                2.0,
                Color::new(0.1, 0.3, 0.8),
            )));
        }
        _ => {
            // scenes 3 and 4 share the same objects
            scene.add(floor());
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
