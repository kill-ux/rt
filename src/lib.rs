pub mod crates;
pub use crates::*;

pub use camera::Camera;
pub use color::Color;
pub use light::Light;
pub use objects::{cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere};
pub use scene::Scene;
pub use vec3::Vec3;

use clap::Parser;

/// A ray tracer that writes a PPM image to stdout
#[derive(Parser)]
#[command(name = "rt")]
pub struct Config {
    /// Image width in pixels
    #[arg(short = 'w', long, default_value_t = 200)]
    pub width: usize,

    /// Image height in pixels
    #[arg(short = 'H', long, default_value_t = 150)]
    pub height: usize,

    /// Samples per pixel (more = smoother but slower)
    #[arg(short, long, default_value_t = 50)]
    pub samples: u32,

    /// sky light
    #[arg(long, default_value_t = 0.3, value_parser = unit_range)]
    pub sky: f64,

    #[arg(short, long, default_value_t = 1.0, value_parser = non_negative)]
    pub brightness: f64,

    /// Which scene to render: 1 sphere, 2 plane + cube (dimmer), 3 all objects, 4 same as 3 from another angle
    #[arg(long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(1..=4))]
    pub scene: u32,

    /// Camera position as x,y,z (overrides the scene's default camera)
    #[arg(long, value_parser = parse_vec3, allow_hyphen_values = true)]
    pub cam_pos: Option<Vec3>,

    /// Point the camera looks at, as x,y,z (overrides the default)
    #[arg(long, value_parser = parse_vec3, allow_hyphen_values = true)]
    pub look_at: Option<Vec3>,
}

pub fn unit_range(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if (0.0..=1.0).contains(&v) {
        Ok(v)
    } else {
        Err(format!("{v} is not between 0 and 1"))
    }
}

fn non_negative(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if v >= 0.0 {
        Ok(v)
    } else {
        Err(format!("{v} must be 0 or more"))
    }
}

fn parse_vec3(s: &str) -> Result<Vec3, String> {
    let p: Vec<&str> = s.split(',').collect();
    if p.len() != 3 {
        return Err(format!("expected x,y,z but got `{s}`"));
    }
    let n = |i: usize| {
        p[i].trim()
            .parse::<f64>()
            .map_err(|_| format!("`{}` is not a number", p[i]))
    };
    Ok(Vec3::new(n(0)?, n(1)?, n(2)?))
}
