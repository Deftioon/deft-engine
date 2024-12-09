pub mod linalg;
pub mod window;
use minifb::Key;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u64 = 120;

fn main() {
    let mut new_window = window::DWindow::new("Window Title (ESC to exit)", WIDTH, HEIGHT);
    new_window.set_fps(FPS);
    while new_window.is_open() && !new_window.is_key_down(Key::Escape) {
        for i in new_window.pixels.data.iter_mut() {
            *i = 0x000000;
        }
        new_window.update();
    }
}