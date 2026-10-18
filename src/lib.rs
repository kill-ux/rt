pub mod crates;
pub use crates::*;

pub use camera::Camera;
pub use color::Color;
pub use light::Light;
pub use objects::{plane::Plane, sphere::Sphere};
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
}

pub fn unit_range(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if (0.0..=1.0).contains(&v) {
        Ok(v)
    } else {
        Err(format!("{v} is not between 0 and 1"))
    }
}
