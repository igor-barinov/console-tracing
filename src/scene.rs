#[path="linear.rs"] mod linear;

#[path="meshes.rs"] mod meshes;
use meshes::{Sphere};

#[path="camera.rs"] mod camera;
use camera::{Camera};

#[path="screen.rs"] mod screen;
use screen::{Screen, PixelValue};

pub struct Scene {
    objects: Vec<Sphere>,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            camera: Camera::new(),
        }
    }

    pub fn render_to(&self, screen: &mut Screen) {
        for y in Screen.height {
            for x in Screen.width {
                intersection_dist = f64::INFINITY;
            }
        }

        /*
        for each pixel {
            cast_ray()
            for each object {
                get_intersections()
                intersections.get_closest()
            }

            compute_color()
        }
        */
    }
}