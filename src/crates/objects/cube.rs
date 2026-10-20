use std::mem::swap;

use crate::{Color, Vec3, objects::Hittable};

pub struct Cube {
    pub min: Vec3, // minimum corner of the cube
    pub max: Vec3, // maximum corner of the cube
    pub color: Color,
}

impl Cube {
    pub fn new(center: Vec3, size: f64, color: Color) -> Self {
        let h = size / 2.;
        Self {
            min: center - Vec3::new(h, h, h),
            max: center + Vec3::new(h, h, h),
            color,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &crate::ray::Ray) -> Option<super::HitRecord> {
        let o = [ray.origin.x, ray.origin.y, ray.origin.z];
        let d = [ray.direction.x, ray.direction.y, ray.direction.z];

        let lo = [self.min.x, self.min.y, self.min.z];
        let hi = [self.max.x, self.max.y, self.max.z];

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

        let (t, normal) = if t_enter > epsilon {
            (t_enter, enter_normal)
        } else if t_exit > epsilon {
            (t_exit, exit_normal)
        } else {
            return None;
        };

        Some(super::HitRecord {
            t,
            point: ray.origin + ray.direction * t,
            normal,
            color: self.color,
        })
    }
}
