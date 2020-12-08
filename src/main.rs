mod scene;
use scene::{Scene, Camera, Mesh, Material, PointLight, Sphere, Screen};

fn main() {
    let mut world = Scene::new();
    let mut console_scr = Screen::with_resolution(100, 50);

    let mut sphere_a = Sphere::with_radius(2.0);
    sphere_a.move_to((10.0, 0.0, 0.0));
    let mut mat_a = Material::new();
    mat_a.set_specular_coefficient(0.6);
    mat_a.set_diffuse_coefficient(1.0);

    let mut sphere_b = Sphere::with_radius(2.0);
    sphere_b.move_to((15.0, 10.0, 0.0));
    let mut mat_b = Material::new();
    mat_b.set_specular_coefficient(0.6);
    mat_b.set_diffuse_coefficient(1.0);

    let mut cam = Camera::new();
    cam.look_at((1.0, 1.0, 0.1));

    let mut light = PointLight::with_intensity(0.03);
    light.move_to((5.0, 5.0, -1.0));

    world.set_camera(cam);
    world.set_point_light(light);
    world.add_sphere(sphere_a, mat_a);
    world.add_sphere(sphere_b, mat_b);
    world.render_to(&mut console_scr);
}