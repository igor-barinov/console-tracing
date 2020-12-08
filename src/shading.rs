#[path="defaults.rs"] mod defaults;
use defaults::types::{Scalar, Vec3};

pub trait Light {
    fn position(&self) -> Vec3;
    fn intensity(&self) -> Scalar;
}

pub struct PointLight {
    position: Vec3,
    intensity: Scalar,
}

impl PointLight {
    pub fn new() -> Self {
        PointLight {
            position: defaults::linear::ORIGIN,
            intensity: defaults::shading::POINT_LIGHT_INTENSITY
        }
    }

    pub fn with_intensity(initial_intensity: Scalar) -> Self {
        PointLight {
            position: defaults::linear::ORIGIN,
            intensity: initial_intensity
        }
    }

    pub fn move_to(&mut self, new_position: Vec3) {
        self.position = new_position;
    }
}

impl Light for PointLight {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn intensity(&self) -> Scalar {
        self.intensity
    }
}

pub struct Material {
    color: Vec3,
    diffuse: Scalar,
    specular: Scalar,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: defaults::shading::COLOR,
            diffuse: defaults::shading::DIFFUSE_COEF,
            specular: defaults::shading::SPECULAR_COEF,
        }
    }

    pub fn color(&self) -> Vec3 {
        self.color
    }

    pub fn diffuse_coefficient(&self) -> Scalar {
        self.diffuse
    }

    pub fn specular_coefficient(&self) -> Scalar {
        self.specular
    }

    pub fn set_color(&mut self, r: Scalar, g: Scalar, b: Scalar) {
        self.color = (r, g, b);
    }

    pub fn set_diffuse_coefficient(&mut self, new_diffuse_coef: Scalar) {
        self.diffuse = new_diffuse_coef;
    }

    pub fn set_specular_coefficient(&mut self, new_specular_coef: Scalar) {
        self.specular = new_specular_coef;
    }

}