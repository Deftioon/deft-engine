use minifb::{Key, Window, WindowOptions};

pub mod game;
pub mod image;

pub struct DWindow {
    pub window: Window,
    pub image: image::Image,
    pub width: usize,
    pub height: usize
}

impl DWindow {
    pub fn new(title: &str, width: usize, height: usize) -> DWindow {
        DWindow {
            window: Window::new(
                title,
                width,
                height,
                WindowOptions::default()
            ).unwrap_or_else(|e| {
                panic!("{}", e)
            }),
            image: image::Image::new(width, height),
            width,
            height
        }
    }

    pub fn clear(&mut self) {
        for i in self.image.pixels.data.iter_mut() {
            *i = 0x000000;
        }
    }

    pub fn set_color(&mut self, color: u32) {
        for i in self.image.pixels.data.iter_mut() {
            *i = color;
        }
    }

    pub fn buffer(&self) -> Vec<u32> {
        self.image.flatten()
    }

    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.buffer(), self.width, self.height).unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(1/fps * 1000 * 1000)));
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.image.pixels.set(y, x, color);
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        self.image.pixels.get(y, x)
    }
}

impl DWindow {
    pub fn draw_object_2d(&mut self, obj: &dyn game::GameObjectCommon) {
        self.image.draw_object_2d(obj);
    }
}

pub fn main_loop(title: &str, fps: u64, width: usize, height: usize) {
    let mut new_window = DWindow::new(title, width, height);
    new_window.set_fps(fps);
    while new_window.is_open() && !new_window.is_key_down(Key::Escape) {
        new_window.clear();
        new_window.update();
    }
}