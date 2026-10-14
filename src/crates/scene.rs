use crate::{light::Light, objects::Hittable};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub light: Light,
}

impl Scene {
    fn new(light: Light) -> Self {
        Self {
            objects: Vec::new(),
            light,
        }
    }
}
