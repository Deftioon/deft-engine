use engine::game::GameObjectCommon;
use minifb::Key;
pub mod linalg;
pub mod engine;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u64 = 5;

fn main() {
    let mut new_window = engine::DWindow::new("hi", WIDTH, HEIGHT);
    let mut new_box = engine::game::Rect::new2d(200, 100, 50, 50, 0x00FF00);
    let mut new_polygon = engine::game::Polygon::new2d(0xFF00FF);
    new_polygon.add_point(engine::game::Point::new(50, 50, 0));
    new_polygon.add_point(engine::game::Point::new(50, 100, 0));
    new_polygon.add_point(engine::game::Point::new(100, 150, 0));
    new_polygon.add_point(engine::game::Point::new(150, 150, 0));
    new_polygon.add_point(engine::game::Point::new(100, 100, 0));
    new_polygon.add_point(engine::game::Point::new(100, 50, 0));
    new_window.set_fps(FPS);
    while new_window.is_open() && !new_window.is_key_down(Key::Escape) {
        new_window.clear();
        new_window.draw_object_2d(&new_box);
        new_window.draw_polygon_2d(&new_polygon);

        new_box.update();
        new_polygon.update();
        new_window.update();
    }
}