use engine::game::GameObjectCommon;
use minifb::Key;
pub mod engine;
pub mod linalg;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const FPS: u64 = 120;

fn main() {
    let mut new_window = engine::DWindow::new("hi", WIDTH, HEIGHT);
    let new_box = engine::game::Rect::new2d(200, 100, 50, 50, 0x00FF00, engine::game::DrawMode::Override, true);
    let mut new_polygon = engine::game::Polygon::new2d(0xFF00FF, engine::game::DrawMode::Addition, true);
    new_polygon.add_point(engine::game::Point::new(50, 50, 0));
    new_polygon.add_point(engine::game::Point::new(50, 100, 0));
    new_polygon.add_point(engine::game::Point::new(100, 150, 0));
    new_polygon.add_point(engine::game::Point::new(150, 150, 0));
    new_polygon.add_point(engine::game::Point::new(100, 50, 0));
    new_polygon.add_point(engine::game::Point::new(100, 100, 0));
    new_polygon.add_point(engine::game::Point::new(309, 134, 0));
    new_window.set_fps(FPS);
    new_window.add_object(Box::new(new_box));
    new_window.add_object(Box::new(new_polygon));

    let mut velocity = 5.0;
    while new_window.is_open() && !new_window.is_key_down(Key::Escape) {
        if new_window.is_key_down(Key::Space) {
            new_window.clear();
        }

        if new_window.is_key_down(Key::LeftShift) {
            velocity = 20.0;
        }
        else {
            velocity = 5.0;
        }

        if new_window.is_key_down(Key::W) {
            new_window.set_velocity(1, 0.0, -velocity, 0.0);
        }
        else if new_window.is_key_down(Key::S) {
            new_window.set_velocity(1, 0.0, velocity, 0.0);
        }
        else if new_window.is_key_down(Key::A) {
            new_window.set_velocity(1, -velocity, 0.0, 0.0);
        }
        else if new_window.is_key_down(Key::D) {
            new_window.set_velocity(1, velocity, 0.0, 0.0);
        }
        new_window.clear();
        new_window.update();
    }
}