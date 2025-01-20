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

    pub fn add_block(&mut self, start_row: usize, start_col: usize, block: &Image) {
        self.pixels.add_block(start_row as u32, start_col as u32, &block.pixels);
    }

    pub fn overlay_block(&mut self, start_row: usize, start_col: usize, block: &Image) {
        self.pixels.overlay_block(start_row as u32, start_col as u32, &block.pixels);
    }
}

impl Image {
    pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn draw_object_2d_filled(&mut self, obj: &mut Box<dyn game::GameObjectCommon>) {
        let (x, y, _) = obj.coord();
        obj.generate_image();
        let image = obj.image();
        match obj.mode() {
            game::DrawMode::Addition => self.add_block(y as usize, x as usize, &image),
            game::DrawMode::Overlay => self.overlay_block(y as usize, x as usize, &image),
            game::DrawMode::Override => self.set_block(y as usize, x as usize, &image),
        }
    }

    pub fn draw_object_2d_hollow(&mut self, obj: &mut Box<dyn game::GameObjectCommon>) {
        let (x, y, _) = obj.coord();
        obj.generate_image_hollow();
        let image = obj.image();
        match obj.mode() {
            game::DrawMode::Addition => self.add_block(y as usize, x as usize, &image),
            game::DrawMode::Overlay => self.overlay_block(y as usize, x as usize, &image),
            game::DrawMode::Override => self.set_block(y as usize, x as usize, &image),
        }
    }

    pub fn draw_line(&mut self, point1: &game::Point, point2: &game::Point, color: u32) {
        let (x1, y1, _) = point1.coord;
        let (x2, y2, _) = point2.coord;
        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;
        let mut x = x1 as i32;
        let mut y = y1 as i32;

        let x_inc = if dx < 0 { -1 } else { 1 };
        let y_inc = if dy < 0 { -1 } else { 1 };
        let dx = dx.abs();
        let dy = dy.abs();

        if dx >= dy {
            let mut p = 2 * dy - dx;
            let two_dy = 2 * dy;
            let two_dy_dx = 2 * (dy - dx);
            for _ in 0..dx {
                self.set(y as usize, x as usize, color);
                x += x_inc;
                if p < 0 {
                    p += two_dy;
                } else {
                    y += y_inc;
                    p += two_dy_dx;
                }
            }
        } else {
            let mut p = 2 * dx - dy;
            let two_dx = 2 * dx;
            let two_dx_dy = 2 * (dx - dy);
            for _ in 0..dy {
                self.set(y as usize, x as usize, color);
                y += y_inc;
                if p < 0 {
                    p += two_dx;
                } else {
                    x += x_inc;
                    p += two_dx_dy;
                }
            }
        }
    }
}

impl Image {
    pub fn fill_triangle(&mut self, points: Vec<&game::Point>, color: u32) {
        if points.len() != 3 {
            return;
        }

        let mut points = points.clone();
        points.sort_by(|a, b| a.coord.1.cmp(&b.coord.1));

        let (p1, p2, p3) = (points[0], points[1], points[2]);

        let fill_bottom_flat_triangle = |image: &mut Image, v1: &game::Point, v2: &game::Point, v3: &game::Point, color: u32| {
            let inv_slope1 = (v2.coord.0 as f32 - v1.coord.0 as f32) / (v2.coord.1 as f32 - v1.coord.1 as f32);
            let inv_slope2 = (v3.coord.0 as f32 - v1.coord.0 as f32) / (v3.coord.1 as f32 - v1.coord.1 as f32);

            let mut curx1 = v1.coord.0 as f32;
            let mut curx2 = v1.coord.0 as f32;

            for scanline_y in v1.coord.1..=v2.coord.1 {
                image.draw_line(&game::Point::new(curx1 as i32, scanline_y, 0), &game::Point::new(curx2 as i32, scanline_y, 0), color);
                curx1 += inv_slope1;
                curx2 += inv_slope2;
            }
        };

        let fill_top_flat_triangle = |image: &mut Image, v1: &game::Point, v2: &game::Point, v3: &game::Point, color: u32| {
            let inv_slope1 = (v3.coord.0 as f32 - v1.coord.0 as f32) / (v3.coord.1 as f32 - v1.coord.1 as f32);
            let inv_slope2 = (v3.coord.0 as f32 - v2.coord.0 as f32) / (v3.coord.1 as f32 - v2.coord.1 as f32);

            let mut curx1 = v3.coord.0 as f32;
            let mut curx2 = v3.coord.0 as f32;

            for scanline_y in (v1.coord.1..=v3.coord.1).rev() {
                image.draw_line(&game::Point::new(curx1 as i32, scanline_y, 0), &game::Point::new(curx2 as i32, scanline_y, 0), color);
                curx1 -= inv_slope1;
                curx2 -= inv_slope2;
            }
        };

        if p2.coord.1 == p3.coord.1 {
            fill_bottom_flat_triangle(self, p1, p2, p3, color);
        } else if p1.coord.1 == p2.coord.1 {
            fill_top_flat_triangle(self, p1, p2, p3, color);
        } else {
            let p4 = game::Point::new(
                p1.coord.0 + ((p2.coord.1 - p1.coord.1) as f32 / (p3.coord.1 - p1.coord.1) as f32 * (p3.coord.0 - p1.coord.0) as f32) as i32,
                p2.coord.1,
                0,
            );
            fill_bottom_flat_triangle(self, p1, p2, &p4, color);
            fill_top_flat_triangle(self, p2, &p4, p3, color);
        }
    }

    pub fn fill_convex_polygon(&mut self, polygon: &game::Polygon, color: u32) {
        let points = polygon.points();

        // Fan Triangulation
        let mut triangles = Vec::new();
        for i in 1..points.len() - 1 {
            triangles.push(vec![points[0], points[i], points[i + 1]]);
        }

        for triangle in triangles {
            self.fill_triangle(triangle, color);
        }
        
    }
}

