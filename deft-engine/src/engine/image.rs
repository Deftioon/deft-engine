
use crate::linalg;
use crate::engine::game;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: linalg::Matrix,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: linalg::Matrix::zeros(height as u32, width as u32),
        }
    }

    pub fn new_filled(rgb: u32, width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: linalg::Matrix::from_data(height as u32, width as u32, vec![rgb; width * height]),
        }
    }

    pub fn from_data(width: usize, height: usize, data: Vec<u32>) -> Image {
        Image {
            width,
            height,
            pixels: linalg::Matrix::from_data(height as u32, width as u32, data),
        }
    }

    pub fn flatten(&self) -> Vec<u32> {
        self.pixels.data.clone()
    }

    pub fn print(&self) {
        linalg::Matrix::print(&self.pixels);
    }

    pub fn get(&self, row: usize, col: usize) -> u32 {
        self.pixels.get(row as u32, col as u32)
    }

    pub fn set(&mut self, row: usize, col: usize, value: u32) {
        self.pixels.set(row as u32, col as u32, value);
    }

    pub fn get_block(&self, start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> Image {
        Image {
            width: end_col - start_col,
            height: end_row - start_row,
            pixels: self.pixels.get_block(start_row as u32, start_col as u32, end_row as u32, end_col as u32),
        }
    }

    pub fn set_block(&mut self, start_row: usize, start_col: usize, block: &Image) {
        self.pixels.set_block(start_row as u32, start_col as u32, &block.pixels);
    }
}

impl Image {
    pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn draw_object_2d(&mut self, obj: &dyn game::GameObjectCommon) {
        let (x, y, _) = obj.coord();
        let image = obj.image();
        self.set_block(y as usize, x as usize, image);
    }
}