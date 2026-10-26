use std::mem::swap;

use crate::{Color, Vec3, objects::Hittable};

pub struct Cube {
    pub min: Vec3, // minimum corner of the cube
    pub max: Vec3, // maximum corner of the cube
    pub color: Color,
    pub reflectivity: f64,
    pub rotation: Vec3,
}

impl Cube {
    pub fn new(center: Vec3, size: f64, color: Color) -> Self {
        let h = size / 2.;
        Self {
            min: center - Vec3::new(h, h, h),
            max: center + Vec3::new(h, h, h),
            color,
            reflectivity: 0.,
            rotation: Vec3::ZERO,
        }
    }

    pub fn with_reflectivity(mut self, r: f64) -> Self {
        self.reflectivity = r.clamp(0.0, 1.);
        self
    }

    pub fn with_rotation(mut self, degrees: Vec3) -> Self {
        self.rotation = degrees;
        self
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &crate::ray::Ray) -> Option<super::HitRecord> {
        // # rotate
        let center = (self.min + self.max) * 0.5;
        let half_size = (self.max - self.min) * 0.5;

        let angles = Vec3::new(
            self.rotation.x.to_radians(),
            self.rotation.y.to_radians(),
            self.rotation.z.to_radians(),
        );

        // Move the ray relative to the cube's center
        // then undo the cube's rotation.

        let local_origin = rotate_inverse(ray.origin - center, angles);
        let local_direction = rotate_inverse(ray.direction, angles);

        // #

        // let o = [ray.origin.x, ray.origin.y, ray.origin.z];
        // let d = [ray.direction.x, ray.direction.y, ray.direction.z];

        let o = [local_origin.x, local_origin.y, local_origin.z];
        let d = [local_direction.x, local_direction.y, local_direction.z];

        // let lo = [self.min.x, self.min.y, self.min.z];
        // let hi = [self.max.x, self.max.y, self.max.z];

        let lo = [-half_size.x, -half_size.y, -half_size.z];
        let hi = [half_size.x, half_size.y, half_size.z];

        let mut t_enter = f64::NEG_INFINITY;
        let mut t_exit = f64::INFINITY;

        let mut enter_normal = Vec3::ZERO;
        let mut exit_normal = Vec3::ZERO;

        for i in 0..3 {
            if d[i] == 0.0 {
                if o[i] < lo[i] || o[i] > hi[i] {
                    return None;
                }

                continue;
            }

            let mut t0 = (lo[i] - o[i]) / d[i];
            let mut t1 = (hi[i] - o[i]) / d[i];

            let mut n0 = match i {
                0 => Vec3::new(-1.0, 0.0, 0.0),
                1 => Vec3::new(0.0, -1.0, 0.0),
                _ => Vec3::new(0.0, 0.0, -1.0),
            };

            let mut n1 = n0 * -1.0;

            if t0 > t1 {
                swap(&mut t0, &mut t1);
                swap(&mut n0, &mut n1);
            }

            if t0 > t_enter {
                t_enter = t0;
                enter_normal = n0;
            }

            if t1 < t_exit {
                t_exit = t1;
                exit_normal = n1;
            }

            if t_enter > t_exit {
                return None;
            }
        }

        let epsilon = 1e-6;

        let (t, local_normal) = if t_enter > epsilon {
            (t_enter, enter_normal)
        } else if t_exit > epsilon {
            (t_exit, exit_normal)
        } else {
            return None;
        };

        let normal = rotate_forward(local_normal, angles).normalize();

        Some(super::HitRecord {
            t,
            point: ray.origin + ray.direction * t,
            normal,
            color: self.color,
            reflectivity: self.reflectivity,
        })
    }
}

fn rotate_x(v: Vec3, angle: f64) -> Vec3 {
    let (s, c) = angle.sin_cos();
    return Vec3::new(v.x, c * v.y - s * v.z, s * v.y + c * v.z);
}

fn rotate_y(v: Vec3, angle: f64) -> Vec3 {
    let (s, c) = angle.sin_cos();

    Vec3::new(c * v.x + s * v.z, v.y, -s * v.x + c * v.z)
}

fn rotate_z(v: Vec3, angle: f64) -> Vec3 {
    let (s, c) = angle.sin_cos();

    Vec3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z)
}

/// Local coordinates → world orientation.
/// Apply X, then Y, then Z.
fn rotate_forward(v: Vec3, angles: Vec3) -> Vec3 {
    let v = rotate_x(v, angles.x);
    let v = rotate_y(v, angles.y);
    rotate_z(v, angles.z)
}

// World orientation → local coordinates.
// Undo Z, then Y, then X.
fn rotate_inverse(v: Vec3, angles: Vec3) -> Vec3 {
    let v = rotate_z(v, -angles.z);
    let v = rotate_y(v, -angles.y);

    rotate_x(v, -angles.x)
}
