#[path="defaults.rs"] mod defaults;
use defaults::types::{Scalar, Vec3};

#[path="linear.rs"] mod linear;



pub trait Mesh {
    fn position(&self) -> Vec3;
    fn orientation(&self) -> Vec3;
    fn intersects_with_ray(&self, ray: &Ray) -> Option<Ray>;
}

pub struct Ray {
    // Ray = position + t*orientation
    position: Vec3,
    orientation: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            position: defaults::linear::ORIGIN,
            orientation: defaults::linear::Z_UNIT_VEC,
        }
    }

    pub fn with_orientation(initial_orientation: Vec3) -> Self {
        Ray {
            position: defaults::linear::ORIGIN,
            orientation: initial_orientation,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn orientation(&self) -> Vec3 {
        self.orientation
    }

    pub fn move_to(&mut self, new_position: Vec3) {
        self.position = new_position;
    }

    pub fn point_at(&self, t: Scalar) -> Vec3 {
        linear::vector_sum(&self.position, &linear::scale_vector(&self.orientation, t))
    }
}


pub struct Sphere {
    position: Vec3,
    orientation: Vec3,
    radius: Scalar,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            position: defaults::linear::ORIGIN,
            orientation: defaults::linear::Z_UNIT_VEC,
            radius: defaults::meshes::SPHERE_RADIUS,
        }
    }

    pub fn with_radius(initial_radius: Scalar) -> Self {
        Sphere {
            position: defaults::linear::ORIGIN,
            orientation: defaults::linear::Z_UNIT_VEC,
            radius: initial_radius
        }
    }

    pub fn move_to(&mut self, new_position: Vec3) {
        self.position = new_position;
    }
}

impl Mesh for Sphere {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn orientation(&self) -> Vec3 {
        self.orientation
    }

    fn intersects_with_ray(&self, ray: &Ray) -> Option<Ray> {
        let sphere_point_diff = linear::vector_diff(&ray.position, &self.position);
        let a = linear::dot_product(&ray.orientation, &ray.orientation);
        let b = linear::dot_product(&linear::scale_vector(&ray.orientation, 2.0), &sphere_point_diff);
        let c = linear::dot_product(&sphere_point_diff, &sphere_point_diff) - (self.radius*self.radius);

        let mut t = f64::INFINITY;
        let discrm = (b*b) - (4.0*a*c);
        if discrm > 0.0 {
            t = (-b - discrm.sqrt()) / (2.0*a);
        } else if linear::approx_equal(discrm, 0.0) {
            t = -b / (2.0*a);
        }

        if t == f64::INFINITY {
            return None
        }

        let point = ray.point_at(t);
        let orientation = linear::normalized(&linear::vector_diff(&point, &self.position));
        let mut normal = Ray::with_orientation(orientation);
        normal.move_to(point);
        Some(normal)
    }
}