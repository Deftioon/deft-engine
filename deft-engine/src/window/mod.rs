use minifb::{Key, Window, WindowOptions};
use crate::linalg;

pub struct DWindow {
    pub window: Window,
    pub pixels: linalg::Matrix,
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
            pixels: linalg::Matrix::zeros(height as u32, width as u32),
            width,
            height
        }
    }

    pub fn buffer(&self) -> Vec<u32> {
        self.pixels.flatten()
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
}

fn from_u8_rgb(r: u8, g: u8, b:u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}