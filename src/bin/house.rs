
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

fn add_cube(scene: &mut Scene, center: Vec3, size: f64, color: Color, reflectivity: f64) {
    scene.add(Box::new(
        Cube::new(center, size, color).with_reflectivity(reflectivity),
    ));
}

fn add_sphere(scene: &mut Scene, center: Vec3, radius: f64, color: Color, reflectivity: f64) {
    scene.add(Box::new(
        Sphere::new(center, radius, color).with_reflectivity(reflectivity),
    ));
}

fn add_tree(scene: &mut Scene, x: f64, z: f64) {
    // Ground is y = -1.
    // Trunk extends from y = -1 to y = 2.
    scene.add(Box::new(Cylinder::new(
        Vec3::new(x, 0.5, z),
        0.24,
        3.0,
        Color::new(0.30, 0.13, 0.05),
    )));

    // Three overlapping spheres make the leafy crown.
    add_sphere(
        scene,
        Vec3::new(x, 2.9, z),
        1.2,
        Color::new(0.10, 0.38, 0.08),
        0.0,
    );

    add_sphere(
        scene,
        Vec3::new(x - 0.65, 2.1, z + 0.1),
        0.9,
        Color::new(0.16, 0.48, 0.09),
        0.0,
    );

    add_sphere(
        scene,
        Vec3::new(x + 0.65, 2.2, z - 0.1),
        0.9,
        Color::new(0.12, 0.42, 0.06),
        0.0,
    );
}

fn main() {
    let cfg = Config::parse();

    let mut scene = Scene::new(Light::new(
        Vec3::new(-10.0, 18.0, 6.0),
        1.2 * cfg.brightness,
    ));

    // --------------------------------------------------
    // Ground
    // --------------------------------------------------

    scene.add(Box::new(Plane::new(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Color::new(0.20, 0.38, 0.12),
    )));

    // --------------------------------------------------
    // House body
    //
    // Center: (0, 2, -12)
    // Size: 6
    //
    // X: -3 to 3
    // Y: -1 to 5
    // Z: -15 to -9
    //
    // The front faces toward positive Z.
    // --------------------------------------------------

    add_cube(
        &mut scene,
        Vec3::new(0.0, 2.0, -12.0),
        6.0,
        Color::new(0.85, 0.72, 0.50),
        0.0,
    );

    // --------------------------------------------------
    // Stepped roof
    //
    // Each smaller cube sits on top of the previous one.
    // --------------------------------------------------

    let roof_layers = [
        // center Y, size
        (5.35, 6.8),
        (6.05, 5.4),
        (6.75, 4.0),
        (7.45, 2.6),
        (8.15, 1.2),
    ];

    for (y, size) in roof_layers {
        add_cube(
            &mut scene,
            Vec3::new(0.0, y, -12.0),
            size,
            Color::new(0.52, 0.10, 0.055),
            0.0,
        );
    }

    // --------------------------------------------------
    // Chimney
    // --------------------------------------------------

    add_cube(
        &mut scene,
        Vec3::new(1.8, 7.3, -13.0),
        0.85,
        Color::new(0.32, 0.24, 0.20),
        0.0,
    );

    add_cube(
        &mut scene,
        Vec3::new(1.8, 8.0, -13.0),
        0.85,
        Color::new(0.32, 0.24, 0.20),
        0.0,
    );

    add_cube(
        &mut scene,
        Vec3::new(1.8, 8.5, -13.0),
        1.0,
        Color::new(0.18, 0.16, 0.15),
        0.0,
    );

    // --------------------------------------------------
    // Front door
    //
    // Two stacked cubes form a tall door.
    // Most of each cube is embedded in the house.
    // --------------------------------------------------

    for y in [-0.3, 1.1] {
        add_cube(
            &mut scene,
            Vec3::new(0.0, y, -9.55),
            1.4,
            Color::new(0.24, 0.10, 0.04),
            0.0,
        );
    }

    // Door handle.
    add_sphere(
        &mut scene,
        Vec3::new(0.43, 0.7, -8.77),
        0.09,
        Color::new(0.95, 0.65, 0.12),
        0.55,
    );

    // --------------------------------------------------
    // Front windows
    //
    // White cubes form frames.
    // Smaller blue cubes protrude slightly farther.
    // These are opaque blue panels, not transparent glass.
    // --------------------------------------------------

    for x in [-1.9, 1.9] {
        for y in [0.8, 3.3] {
            add_cube(
                &mut scene,
                Vec3::new(x, y, -9.45),
                1.25,
                Color::new(0.93, 0.90, 0.80),
                0.0,
            );

            add_cube(
                &mut scene,
                Vec3::new(x, y, -9.20),
                0.95,
                Color::new(0.12, 0.40, 0.65),
                0.25,
            );
        }
    }

    // --------------------------------------------------
    // Windows on the right side
    // --------------------------------------------------

    for z in [-10.8, -13.2] {
        for y in [0.8, 3.3] {
            add_cube(
                &mut scene,
                Vec3::new(2.55, y, z),
                1.25,
                Color::new(0.93, 0.90, 0.80),
                0.0,
            );

            add_cube(
                &mut scene,
                Vec3::new(2.80, y, z),
                0.95,
                Color::new(0.12, 0.40, 0.65),
                0.25,
            );
        }
    }

    // --------------------------------------------------
    // Path leading toward the front door
    //
    // Cubes are mostly buried so only their tops appear.
    // --------------------------------------------------

    for step in 0..9 {
        let z = -7.8 + step as f64 * 1.15;

        add_cube(
            &mut scene,
            Vec3::new(0.0, -1.45, z),
            1.2,
            Color::new(0.55, 0.52, 0.45),
            0.0,
        );
    }

    // --------------------------------------------------
    // Bushes along the front of the house
    // --------------------------------------------------

    for x in [-2.6, -1.8, 1.8, 2.6] {
        add_sphere(
            &mut scene,
            Vec3::new(x, -0.45, -7.9),
            0.55,
            Color::new(0.09, 0.32, 0.055),
            0.0,
        );
    }

    // --------------------------------------------------
    // Trees
    // --------------------------------------------------

    add_tree(&mut scene, -5.8, -11.0);
    add_tree(&mut scene, 6.0, -14.0);
    add_tree(&mut scene, -7.0, -18.0);

    // --------------------------------------------------
    // Flower beds beside the path
    // --------------------------------------------------

    for i in 0..7 {
        let z = -6.8 + i as f64 * 0.9;

        for x in [-1.4, 1.4] {
            let flower_color = match i % 3 {
                0 => Color::new(0.95, 0.12, 0.18),
                1 => Color::new(1.0, 0.72, 0.05),
                _ => Color::new(0.70, 0.10, 0.80),
            };

            add_sphere(&mut scene, Vec3::new(x, -0.86, z), 0.14, flower_color, 0.0);
        }
    }

    // --------------------------------------------------
    // Mirror ornament on a stone pedestal
    // --------------------------------------------------

    add_cube(
        &mut scene,
        Vec3::new(4.7, -0.5, -5.0),
        1.0,
        Color::new(0.45, 0.44, 0.42),
        0.0,
    );

    add_sphere(
        &mut scene,
        Vec3::new(4.7, 0.65, -5.0),
        0.65,
        Color::new(1.0, 1.0, 1.0),
        1.0,
    );

    // --------------------------------------------------
    // Camera
    //
    // View the front and right side of the house.
    // --------------------------------------------------

    let camera_position = Vec3::new(14.0, 13.0, 15.0);
    let camera_target = Vec3::new(0.0, 2.5, -10.0);

    let camera = Camera::new(
        camera_position,
        (camera_target - camera_position).normalize(),
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
