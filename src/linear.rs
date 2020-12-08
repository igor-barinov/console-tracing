use std::num::Float;

pub type Scalar = f64;
pub type Vec3 = (Scalar, Scalar, Scalar);

pub fn cross_product(x: &Vec3, y: &Vec3) -> Vec3 {
    (
        x.0*y.2 - x.2*y.1,
        x.2*y.0 - x.0*y.2,
        x.0*y.1 - x.1*y.0
    )
}

pub fn dot_product(x: &Vec3, y: &Vec3) -> Scalar {
    x.0*y.0 + x.1*y.1 + x.2*y.2
}

pub fn magnitude_of(x: &Vec3) -> Scalar {
    dot_product(x, x).sqrt()
}

pub fn normalized(x: &Vec3) -> Vec3 {
    let mag = magnitude_of(x);
    (
        x.0 / mag,
        x.1 / mag,
        x.2 / mag
    )
}