#[path="defaults.rs"] mod defaults;
use defaults::types::{Scalar, Vec3};

pub fn approx_equal(a: Scalar, b: Scalar) -> bool {
    (a-b).abs() < defaults::linear::EPSILON
}

pub fn scale_vector(x: &Vec3, c: Scalar) -> Vec3 {
    (
        x.0 * c,
        x.1 * c,
        x.2 * c
    )
}

pub fn vector_sum(x: &Vec3, y: &Vec3) -> Vec3 {
    (
        x.0 + y.0,
        x.1 + y.1,
        x.2 + y.2
    )
}

pub fn vector_diff(x: &Vec3, y: &Vec3) -> Vec3 {
    (
        x.0 - y.0,
        x.1 - y.1,
        x.2 - y.2
    )
}

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