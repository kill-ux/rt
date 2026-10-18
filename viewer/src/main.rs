use anyhow::Result;
use minifb::{Key, Window, WindowOptions};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rt::{Camera, Color, Light, Plane, Scene, Sphere, Vec3, renderer::trace};

fn main() -> Result<()> {
    let width = 400;
    let height = 300;

    let mut window = Window::new(
        "Rust Ray Tracer - Real-Time Viewer",
        width,
        height,
        WindowOptions::default(),
    )?;

    window.set_target_fps(60);

    let mut light_pos = Vec3::new(5.0, 5.0, -5.0);
    let mut buffer = vec![0; width * height];

    let mut radius = 5.0;
    let mut angle = 0.0;
    let mut cam_height = 0.0;
    let sphere_center = Vec3::new(0.0, 0.0, -5.0);
    let max_depth = 5;

    // Use 1 sample per pixel for real-time performance (increase if your CPU is very fast)
    let samples = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- Handle Input ---
        let speed = 0.1;
        let light_speed = speed;

        if window.is_key_down(Key::Left) {
            light_pos.x -= light_speed;
        }
        if window.is_key_down(Key::Right) {
            light_pos.x += light_speed;
        }
        if window.is_key_down(Key::Up) {
            light_pos.y += light_speed;
        }
        if window.is_key_down(Key::Down) {
            light_pos.y -= light_speed;
        }
        if window.is_key_down(Key::PageUp) {
            light_pos.z += light_speed;
        }
        if window.is_key_down(Key::PageDown) {
            light_pos.z -= light_speed;
        }

        // Orbit Left / Right (Q and D)
        if window.is_key_down(Key::Q) {
            angle -= speed;
        }
        if window.is_key_down(Key::D) {
            angle += speed;
        }

        // Zoom In / Out (Z and S)
        if window.is_key_down(Key::Z) {
            radius = 1.5_f64.max(radius - 0.1);
        }
        if window.is_key_down(Key::S) {
            radius += speed;
        }

        // Move Up/Down (Space/Shift)
        if window.is_key_down(Key::Space) {
            cam_height += speed;
        }
        if window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
            cam_height -= speed;
        }

        // Calculate camera position and look direction
        let cam_x = sphere_center.x + radius * angle.sin();
        let cam_z = sphere_center.z + radius * angle.cos();
        let cam_pos = Vec3::new(cam_x, cam_height, cam_z);
        let look_dir = (sphere_center - cam_pos).normalize();

        let camera = Camera::new(cam_pos, look_dir, Vec3::new(0.0, 1.0, 0.0), width, height);

        // --- Build Scene ---
        let mut scene = Scene::new(Light::new(light_pos, 1.0));
        scene.add(Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -5.0),
            1.0,
            Color::new(0.996, 0.310, 0.176),
        )));
        scene.add(Box::new(Sphere::new(
            Vec3::new(3.0, -1., -5.0),
            1.0,
            Color::new(0.9, 0.2, 0.2),
        )));
        scene.add(Box::new(Plane::new(
            Vec3::new(0., -1., 0.),
            Vec3::new(0., 1., 0.),
            Color::new(0.8, 0.8, 0.8),
        )));

        // --- Parallel Rendering Loop (Rayon) ---
        buffer = (0..height)
            .into_par_iter()
            .flat_map(|py| {
                let mut row_buffer = vec![0; width];
                for px in 0..width {
                    let mut sum = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..samples {
                        let ray = camera.get_ray(px as f64, py as f64);
                        let traced = trace(&ray, &scene, max_depth);
                        sum = sum + traced;
                    }

                    let c = sum * (1.0 / samples as f64);

                    // Gamma correction
                    let r = (c.x.sqrt().min(1.0) * 255.0) as u32;
                    let g = (c.y.sqrt().min(1.0) * 255.0) as u32;
                    let b = (c.z.sqrt().min(1.0) * 255.0) as u32;

                    row_buffer[px] = (r << 16) | (g << 8) | b;
                }
                row_buffer
            })
            .collect();

        window.update_with_buffer(&buffer, width, height)?;
    }

    Ok(())
}
