use crate::vec3::Vec3;

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: Vec3, intensity: f64) -> Self {
        Light {
            position,
            intensity,
        }
    }
}
