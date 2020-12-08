#[path="linear.rs"] mod linear;

#[path="meshes.rs"] mod meshes;
pub use meshes::{Mesh, Ray, Sphere};

#[path="shading.rs"] mod shading;
pub use shading::{Light, PointLight, Material};

#[path="camera.rs"] mod camera;
pub use camera::{Camera};

#[path="screen.rs"] mod screen;
pub use screen::{Screen, PixelValue};

#[path="defaults.rs"] mod defaults;
use defaults::types::{Scalar, Vec3};

pub struct Scene {
    objects: Vec<(Sphere, Material)>,
    light: PointLight,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            light: PointLight::new(),
            camera: Camera::new(),
        }
    }

    pub fn set_camera(&mut self, new_camera: Camera) {
        self.camera = new_camera;
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material: Material) {
        self.objects.push((sphere, material));
    }

    pub fn set_point_light(&mut self, point_light: PointLight) {
        self.light = point_light;
    }

    pub fn render_to(&self, screen: &mut Screen) {
        let mut pixel_vals = Vec::new();
        for y in 0..screen.vertical_res() {
            for x in 0..screen.horizontal_res() {
                let mut closest_normal = (f64::INFINITY, Ray::new());
                let mut closest_obj = (&Sphere::new(), &Material::new());
                let pixel_ray = self.ray_at_pixel(x, y, screen.horizontal_res(), screen.vertical_res());

                for obj in &self.objects {
                    let maybe_normal = obj.0.intersects_with_ray(&pixel_ray);

                    if maybe_normal.is_some() {
                        let normal = maybe_normal.unwrap();
                        let dist_to_cam = linear::magnitude_of(&linear::vector_diff(&normal.position(), &self.camera.position()));
                        if dist_to_cam < closest_normal.0 {
                            closest_normal.0 = dist_to_cam;
                            closest_normal.1 = normal;
                            closest_obj.0 = &obj.0;
                            closest_obj.1 = &obj.1;
                        }
                    }
                }

                if closest_normal.0 == f64::INFINITY {
                    pixel_vals.push(PixelValue::OFF);
                } else {
                    pixel_vals.push(self.pixel_ray_brightness(&pixel_ray, &closest_normal.1, &self.light, closest_obj.1))
                }
            }
        }

        screen.set_pixels(pixel_vals);
        screen.draw();
    }

    fn camera_basis(&self) -> (Vec3, Vec3, Vec3) {
        let n = linear::normalized(&linear::vector_diff(&self.camera.position(), &self.camera.target()));
        let u = linear::normalized(&linear::cross_product(&self.camera.orientation(), &n));
        let v = linear::cross_product(&n, &u);
        (n, u, v)
    }

    fn ray_at_pixel(&self, x: usize, y: usize, h_res: usize, v_res: usize) -> Ray {
        let cam_basis = self.camera_basis();
        let d = (v_res as Scalar) / (2.0*(self.camera.vertical_fov() / 2.0).tan());
        let C = linear::vector_diff(&self.camera.position(), &linear::scale_vector(&cam_basis.0, d));
        let L = linear::vector_diff(&linear::vector_diff(&C, &linear::scale_vector(&cam_basis.0, (h_res as Scalar)/2.0)), &linear::scale_vector(&cam_basis.2, (v_res as Scalar)/2.0));
        let ray_end = linear::vector_sum(&linear::vector_sum(&L, &linear::scale_vector(&cam_basis.1, x as Scalar)), &linear::scale_vector(&cam_basis.2, y as Scalar));
        let orientation = linear::vector_diff(&ray_end, &self.camera.position());
        let mut ray = Ray::with_orientation(orientation);
        ray.move_to(self.camera.position());
        ray
    }

    fn pixel_ray_brightness(&self, pixel_ray: &Ray, normal: &Ray, light_source: &dyn Light, material: &Material) -> PixelValue {
        let k_d = material.diffuse_coefficient();
        let I = light_source.intensity();
        let N = normal.orientation();
        let L = linear::vector_diff(&normal.position(), &light_source.position());
        let V = pixel_ray.orientation();
        let R = linear::vector_diff(&linear::scale_vector(&N, linear::dot_product(&N, &L) * 2.0), &L);
        let q = material.specular_coefficient();
        let k_s = material.specular_coefficient();

        let C_a = linear::scale_vector(&material.color(), k_d*I + &linear::dot_product(&N, &L)*k_d*I);
        let C_b = linear::scale_vector(&(1.0, 1.0, 1.0), linear::dot_product(&V, &R).powf(q)*k_s*I);
        let C = linear::vector_diff(&C_a, &C_b);

        let magn = linear::magnitude_of(&C);

        if magn >= 3.0 * (defaults::shading::MAX_COLOR_INTENISTY / 4.0) {
            PixelValue::BRIGHT
        } else if magn >= (defaults::shading::MAX_COLOR_INTENISTY / 2.0) {
            PixelValue::MEDIUM
        } else {
            PixelValue::DIM
        }
    }
}