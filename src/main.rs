mod scene;
use scene::{Scene};

mod screen;
use screen::{Screen};

fn main() {
    let mut world = Scene::new();
    let mut console_scr = Screen::with_resolution(100, 100);

    world.render_to(console_scr);
}