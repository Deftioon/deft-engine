use minifb::{Key, Window, WindowOptions};

pub mod game;
pub mod image;
pub mod physics;

pub struct DWindow {
    pub window: Window,
    pub image: image::Image,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub objects: Vec<Box<dyn game::GameObjectCommon>>,
    pub render_queue: Vec<Box<dyn game::GameObjectCommon>>
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
            height,
            depth: 100,
            objects: Vec::new(),
            render_queue: Vec::new()
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
        for obj in self.objects.iter_mut() {
            if obj.filled(){
                self.image.draw_object_2d_filled(obj);
            } else {
                self.image.draw_object_2d_hollow(obj);
            }
        }
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
    pub fn draw_line(&mut self, point1: &game::Point, point2: &game::Point, color: u32) {
        self.image.draw_line(point1, point2, color);
    }

    pub fn draw_object_2d(&mut self, obj: &mut Box<dyn game::GameObjectCommon>, filled: bool) {
        if filled {
            self.image.draw_object_2d_filled(obj);
        } else {
            self.image.draw_object_2d_hollow(obj);
        }
    }
}

impl DWindow {
    pub fn set_velocity(&mut self, index: usize, x: f32, y: f32, z: f32) {
        let future_x = self.objects[index].coord().0 as f32 + x;
        let future_y = self.objects[index].coord().1 as f32 + y;
        let future_z = self.objects[index].coord().2 as f32 + z;

        let width = self.objects[index as usize].size().0 as f32;
        let height = self.objects[index as usize].size().1 as f32;
        let depth = self.objects[index as usize].size().2 as f32;

        if future_x >= 0.0 && future_x + width <= self.width as f32 - 1.0 {
            println!("{}, {}, {}", self.objects[index as usize].coord().0, self.objects[index as usize].coord().1, self.objects[index as usize].coord().2);
            self.objects[index].set_velocity(x, 0.0, 0.0);
            self.objects[index].update();
        }
        if future_y >= 0.0 && future_y + height <= self.height as f32 - 1.0 {
            println!("{}, {}, {}", self.objects[index as usize].coord().0, self.objects[index as usize].coord().1, self.objects[index as usize].coord().2);
            self.objects[index].set_velocity(0.0, y, 0.0);
            self.objects[index].update();
        }
        if future_z >= 0.0 && future_z + depth <= self.depth as f32 - 1.0 {
            println!("{}, {}, {}", self.objects[index as usize].coord().0, self.objects[index as usize].coord().1, self.objects[index as usize].coord().2);
            self.objects[index].set_velocity(0.0, 0.0, z);
            self.objects[index].update();
        }
    }
}

impl DWindow {
    pub fn add_object(&mut self, obj: Box<dyn game::GameObjectCommon>) {
        self.objects.push(obj);
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