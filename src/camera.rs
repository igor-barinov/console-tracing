#[path="defaults.rs"] mod defaults;
use defaults::types::{Scalar, Vec3};

#[path="linear.rs"] mod linear;

use std::fmt;

pub struct Camera {
    position: Vec3,
    orientation: Vec3,
    target: Vec3,
    h_fov: Scalar,
    v_fov: Scalar,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: defaults::linear::ORIGIN,
            orientation: defaults::linear::Z_UNIT_VEC,
            target: defaults::linear::X_UNIT_VEC,
            h_fov: defaults::camera::HORIZONTAL_FOV,
            v_fov: defaults::camera::VERTICAL_FOV,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn orientation(&self) -> Vec3 {
        self.orientation
    }

    pub fn horizontal_fov(&self) -> Scalar {
        self.h_fov
    }

    pub fn vertical_fov(&self) -> Scalar {
        self.v_fov
    }

    pub fn move_to(&mut self, new_position: Vec3) {
        self.position = new_position;
    }

    pub fn look_at(&mut self, new_target: Vec3) {
        self.target = new_target;
    }
}

impl fmt::Debug for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Camera")
            .field("located at ", &self.position)
            .field("looking at ", &self.target)
            .field("orientation is", &self.orientation)
            .field("with VFOV of", &self.v_fov)
            .field("and HFOV of", &self.h_fov)
            .finish()
    }
}