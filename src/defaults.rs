pub mod screen {
    pub const WIDTH: usize = 100;
    pub const HEIGHT: usize = 100;
}

pub mod camera {
    #[path = "../linear.rs"] mod linear;
    use linear::{Scalar};
    pub const HORIZONTAL_FOV: Scalar = 90.0;
    pub const VERTICAL_FOV: Scalar = 90.0;
}

pub mod meshes {
    #[path = "../linear.rs"] mod linear;
    use linear::{Vec3, Scalar};
    pub const POSITION: Vec3 = (0.0, 0.0, 0.0);
    pub const ORIENTATION: Vec3 = (0.0, 0.0, 1.0);
    pub const SPHERE_RADIUS: Scalar = 1.0;
}

pub mod linear {

}