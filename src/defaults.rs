pub mod types {
    pub type Scalar = f64;
    pub type Vec3 = (Scalar, Scalar, Scalar);
}

pub mod screen {
    pub const WIDTH: usize = 100;
    pub const HEIGHT: usize = 100;
    pub const BLANK_PIXEL: char = ' ';
}

pub mod camera {
    use super::types::{Scalar};
    pub const HORIZONTAL_FOV: Scalar = 90.0;
    pub const VERTICAL_FOV: Scalar = 90.0;
}

pub mod meshes {
    use super::types::{Scalar};
    pub const SPHERE_RADIUS: Scalar = 1.0;
}

pub mod shading {
    use super::types::{Scalar, Vec3};
    pub const POINT_LIGHT_INTENSITY: Scalar = 1.0;
    pub const MAX_COLOR_INTENISTY: Scalar = 1.73205;
    pub const COLOR: Vec3 = (0.5, 0.5, 0.5);
    pub const DIFFUSE_COEF: Scalar = 1.0;
    pub const SPECULAR_COEF: Scalar = 1.0;
}

pub mod linear {
    use super::types::{Scalar, Vec3};
    pub const EPSILON: Scalar = 0.001;
    pub const ORIGIN: Vec3 = (0.0, 0.0, 0.0);
    pub const X_UNIT_VEC: Vec3 = (1.0, 0.0, 0.0);
    pub const Y_UNIT_VEC: Vec3 = (0.0, 1.0, 0.0);
    pub const Z_UNIT_VEC: Vec3 = (0.0, 0.0, 1.0);
}