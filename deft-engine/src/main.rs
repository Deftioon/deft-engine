use engine::game::GameObjectCommon;
use minifb::{Key, Window, WindowOptions};
pub mod linalg;
pub mod engine;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u64 = 5;

fn main() {
    let mut new_window = engine::DWindow::new("hi", WIDTH, HEIGHT);
    let mut new_box = engine::game::Box::new2d(200, 100, 50, 50, 0x00FF00);
    new_window.set_fps(FPS);
    while new_window.is_open() && !new_window.is_key_down(Key::Escape) {
        new_window.clear();
        new_window.draw_object_2d(&new_box);

        if new_window.is_key_down(Key::W) {
            new_box.velocity.1 = -1.0;
        } else if new_window.is_key_down(Key::S) {
            new_box.velocity.1 = 1.0;
        } else {
            new_box.velocity.1 = 0.0;
        }

        new_box.update();

        new_window.update();
    }
}