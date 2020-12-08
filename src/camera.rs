mod defaults;
mod linear;
use linear::{Vec3, Scalar};

pub struct Camera {
    position: Vec3,
    orientation: Vec3,
    h_fov: Scalar,
    v_fov: Scalar,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: defaults::meshes::POSITION,
            orientation: defaults::meshes::ORIENTATION,
            h_fov: defaults::camera::HORIZONTAL_FOV,
            v_fov: defaults::camera::VERTICAL_FOV,
        }
    }
}