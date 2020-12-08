mod linear;
use linear::{Vec3, Scalar};

mod defaults;

pub struct Sphere {
    position: Vec3,
    orientation: Vec3,
    radius: Scalar,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            position: defaults::meshes::POSITION,
            orientation: defaults::meshes::ORIENTATION,
            radius: defaults::meshes::SPHERE_RADIUS,
        }
    }

    pub fn with_radius(initial_radius: Scalar) -> Self {
        Sphere {
            position: defaults::meshes::POSITION,
            orientation: defaults::meshes::ORIENTATION,
            radius: initial_radius
        }
    }
}