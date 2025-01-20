use crate::engine::image;
use crate::engine::physics;

pub enum DrawMode {
    Overlay,
    Override,
    Addition,
}

impl Copy for DrawMode {}
impl Clone for DrawMode {
    fn clone(&self) -> DrawMode {
        *self
    }
}

pub trait GameObjectCommon {
    fn update(&mut self);
    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool;
    fn coord(&self) -> (i32, i32, i32);
    fn velocity(&self) -> (f32, f32, f32);
    fn acceleration(&self) -> (f32, f32, f32);
    fn size(&self) -> (u32, u32, u32);
    fn collision(&self) -> &dyn physics::CollisionObjectCommon;
    fn image(&self) -> &image::Image;
    fn generate_image(&mut self);
    fn generate_image_hollow(&mut self);
    fn move_to(&mut self, x: i32, y: i32, z: i32);
    fn translate(&mut self, x: f32, y: f32, z: f32);
    fn set_velocity(&mut self, x: f32, y: f32, z: f32);
    fn add_velocity(&mut self, x: f32, y: f32, z: f32);
    fn set_acceleration(&mut self, x: f32, y: f32, z: f32);
    fn add_acceleration(&mut self, x: f32, y: f32, z: f32);
    fn mode(&self) -> &DrawMode;
    fn filled(&self) -> bool;
}



pub struct Rect {
    pub coord: (i32, i32, i32),
    pub velocity: (f32, f32, f32),
    pub acceleration: (f32, f32, f32),
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub image: image::Image,
    pub collision: physics::RectCollision,
    pub color: u32,
    pub draw_mode: DrawMode,
    pub filled: bool,
}

impl GameObjectCommon for Rect {
    fn update(&mut self) {
        self.coord.0 = (self.coord.0 as f32 + self.velocity.0) as i32;
        self.coord.1 = (self.coord.1 as f32 + self.velocity.1) as i32;
        self.coord.2 = (self.coord.2 as f32 + self.velocity.2) as i32;

        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;
    }

    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool {
        physics::check_collision(self.collision(), other.collision())
    }

    fn coord(&self) -> (i32, i32, i32) {
        self.coord
    }

    fn velocity(&self) -> (f32, f32, f32) {
        self.velocity
    }

    fn acceleration(&self) -> (f32, f32, f32) {
        self.acceleration
    }

    fn size(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.depth)
    }

    fn collision(&self) -> &dyn physics::CollisionObjectCommon {
        &self.collision
    }

    fn image(&self) -> &image::Image {
        &self.image
    }

    fn move_to(&mut self, x: i32, y: i32, z: i32) {
        self.coord.0 = x;
        self.coord.1 = y;
        self.coord.2 = z;
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.coord.0 = (self.coord.0 as f32 + x) as i32;
        self.coord.1 = (self.coord.1 as f32 + y) as i32;
        self.coord.2 = (self.coord.2 as f32 + z) as i32;
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity.0 = x;
        self.velocity.1 = y;
        self.velocity.2 = z;
    }

    fn add_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity.0 += x;
        self.velocity.1 += y;
        self.velocity.2 += z;
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        self.acceleration.0 = x;
        self.acceleration.1 = y;
        self.acceleration.2 = z;
    }

    fn add_acceleration(&mut self, x: f32, y: f32, z: f32) {
        self.acceleration.0 += x;
        self.acceleration.1 += y;
        self.acceleration.2 += z;
    }

    fn generate_image(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.image.set(j as usize, i as usize, self.color);
            }
        }
    }

    fn generate_image_hollow(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                if i == 0 || i == self.width - 1 || j == 0 || j == self.height - 1 {
                    self.image.set(j as usize, i as usize, self.color);
                }
            }
        }
    }

    fn mode(&self) -> &DrawMode {
        &self.draw_mode
    }

    fn filled(&self) -> bool {
        return self.filled;
    }
}

impl Rect {
    pub fn new2d(x: i32, y: i32, width: u32, height: u32, color: u32, draw_mode: DrawMode, filled: bool) -> Rect {
        Rect {
            coord: (x, y, 0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            width,
            height,
            depth: 0,
            image: image::Image::new_filled(0x000000, width as usize, height as usize),
            collision: physics::RectCollision {
                coord: (x, y, 0),
                width,
                height,
                depth: 0
            },
            color,
            draw_mode,
            filled,
        }
    }
}
pub struct Point {
    pub coord: (i32, i32, i32),
    pub velocity: (f32, f32, f32),
    pub acceleration: (f32, f32, f32)
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            coord: self.coord,
            velocity: self.velocity,
            acceleration: self.acceleration
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point {
            coord: (x, y, z),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0)
        }
    }
}

pub struct Points {
    pub points: Vec<Point>,
}

impl Clone for Points {
    fn clone(&self) -> Points {
        Points {
            points: self.points.clone()
        }
    }
}

impl Points {
    pub fn new() -> Points {
        Points {
            points: Vec::new()
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn min_x(&self) -> i32 {
        let mut min = self.points[0].coord.0;
        for point in self.points.iter() {
            if point.coord.0 < min {
                min = point.coord.0;
            }
        }
        min
    }

    pub fn max_x(&self) -> i32 {
        let mut max = self.points[0].coord.0;
        for point in self.points.iter() {
            if point.coord.0 > max {
                max = point.coord.0;
            }
        }
        max
    }

    pub fn min_y(&self) -> i32 {
        let mut min = self.points[0].coord.1;
        for point in self.points.iter() {
            if point.coord.1 < min {
                min = point.coord.1;
            }
        }
        min
    }

    pub fn max_y(&self) -> i32 {
        let mut max = self.points[0].coord.1;
        for point in self.points.iter() {
            if point.coord.1 > max {
                max = point.coord.1;
            }
        }
        max
    }

    pub fn min_z(&self) -> i32 {
        let mut min = self.points[0].coord.2;
        for point in self.points.iter() {
            if point.coord.2 < min {
                min = point.coord.2;
            }
        }
        min
    }

    pub fn max_z(&self) -> i32 {
        let mut max = self.points[0].coord.2;
        for point in self.points.iter() {
            if point.coord.2 > max {
                max = point.coord.2;
            }
        }
        max
    }

    pub fn center(&self) -> (i32, i32, i32) {
        let min_x = self.min_x();
        let max_x = self.max_x();
        let min_y = self.min_y();
        let max_y = self.max_y();
        let min_z = self.min_z();
        let max_z = self.max_z();

        ((min_x + max_x) / 2, (min_y + max_y) / 2, (min_z + max_z) / 2)
    }

    pub fn width(&self) -> u32 {
        (self.max_x() - self.min_x()) as u32
    }

    pub fn height(&self) -> u32 {
        (self.max_y() - self.min_y()) as u32
    }

    pub fn depth(&self) -> u32 {
        (self.max_z() - self.min_z()) as u32
    }

    pub fn coord(&self) -> (i32, i32, i32) {
        (self.min_x(), self.min_y(), self.min_z())
    }

    pub fn size(&self) -> (u32, u32, u32) {
        (self.width(), self.height(), self.depth())
    }
}

pub struct Polygon {
    pub points: Points,
    pub image: image::Image,
    pub collision: physics::PolygonCollision,
    pub color: u32,
    pub draw_mode: DrawMode,
    pub filled: bool,
}

impl GameObjectCommon for Polygon {
    fn update(&mut self) {
        for point in self.points.points.iter_mut() {
            point.coord.0 = (point.coord.0 as f32 + point.velocity.0) as i32;
            point.coord.1 = (point.coord.1 as f32 + point.velocity.1) as i32;
            point.coord.2 = (point.coord.2 as f32 + point.velocity.2) as i32;

            point.velocity.0 += point.acceleration.0;
            point.velocity.1 += point.acceleration.1;
            point.velocity.2 += point.acceleration.2;
        }
    }

    fn coord(&self) -> (i32, i32, i32) {
        self.points.coord()
    }

    fn velocity(&self) -> (f32, f32, f32) {
        // average velocity of points
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for point in self.points.points.iter() {
            x += point.velocity.0;
            y += point.velocity.1;
            z += point.velocity.2;
        }
        (x / self.points.points.len() as f32, y / self.points.points.len() as f32, z / self.points.points.len() as f32)
    }

    fn acceleration(&self) -> (f32, f32, f32) {
        // average acceleration of points
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for point in self.points.points.iter() {
            x += point.acceleration.0;
            y += point.acceleration.1;
            z += point.acceleration.2;
        }
        (x / self.points.points.len() as f32, y / self.points.points.len() as f32, z / self.points.points.len() as f32)
    }

    fn size(&self) -> (u32, u32, u32) {
        self.points.size()
    }

    fn collision(&self) -> &dyn physics::CollisionObjectCommon {
        &self.collision
    }

    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool {
        physics::check_collision(self.collision(), other.collision())
    }

    fn image(&self) -> &image::Image {
        &self.image
    }

    fn move_to(&mut self, x: i32, y: i32, z: i32) {
        let (min_x, min_y, min_z) = self.points.coord();
        let x = x - min_x;
        let y = y - min_y;
        let z = z - min_z;
        for point in self.points.points.iter_mut() {
            point.coord.0 = point.coord.0 + x;
            point.coord.1 = point.coord.1 + y;
            point.coord.2 = point.coord.2 + z;
        }
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.coord.0 = (point.coord.0 as f32 + x) as i32;
            point.coord.1 = (point.coord.1 as f32 + y) as i32;
            point.coord.2 = (point.coord.2 as f32 + z) as i32;
        }
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.velocity.0 = x;
            point.velocity.1 = y;
            point.velocity.2 = z;
        }
    }

    fn add_velocity(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.velocity.0 += x;
            point.velocity.1 += y;
            point.velocity.2 += z;
        }
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.acceleration.0 = x;
            point.acceleration.1 = y;
            point.acceleration.2 = z;
        }
    }

    fn add_acceleration(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.acceleration.0 += x;
            point.acceleration.1 += y;
            point.acceleration.2 += z;
        }
    }


    fn generate_image(&mut self) {
        let (min_x, min_y, _min_z) = self.points.coord();
        let (width, height, _) = self.points.size();
        self.image = image::Image::new_filled(0x000000, width as usize + 1, height as usize + 1);
        let points = self.points();
        let mut transformed_points = Vec::new();
        for i in 0..points.len() {
            let point1 = points[i];
            let x1 = point1.coord.0 as i32 - min_x as i32;
            let y1 = point1.coord.1 as i32 - min_y as i32;
            let point = Point::new(x1 as i32, y1 as i32, 0);
            transformed_points.push(point);
        }

        let mut new_polygon = Polygon::new2d(self.color, self.draw_mode, self.filled);
        new_polygon.set_points(Points { points: transformed_points });

        self.image.fill_convex_polygon(&new_polygon, self.color);
    }

    fn generate_image_hollow(&mut self) {
        let (min_x, min_y, _min_z) = self.points.coord();
        let (width, height, _) = self.points.size();
        self.image = image::Image::new_filled(0x000000, width as usize + 1, height as usize + 1);
        let points = self.points();
        let mut transformed_points = Vec::new();
        for i in 0..points.len() {
            let point1 = points[i];
            let x1 = point1.coord.0 as i32 - min_x as i32;
            let y1 = point1.coord.1 as i32 - min_y as i32;
            let point = Point::new(x1 as i32, y1 as i32, 0);
            transformed_points.push(point);
        }

        for i in 0..transformed_points.len() {
            let point1 = &transformed_points[i];
            let point2 = &transformed_points[(i + 1) % transformed_points.len()];
            self.image.draw_line(point1, point2, self.color);
        }
    }

    fn mode(&self) -> &DrawMode {
        &self.draw_mode
    }

    fn filled(&self) -> bool {
        return self.filled;
    }

}

impl Polygon {
    pub fn new(color: u32, draw_mode: DrawMode, filled: bool) -> Polygon {
        Polygon {
            points: Points::new(),
            image: image::Image::new(1, 1),
            collision: physics::PolygonCollision {
                points: Points::new()
            },
            color,
            draw_mode,
            filled,
        }
    }

    pub fn new2d(color: u32, draw_mode: DrawMode, filled: bool) -> Polygon {
        Polygon {
            points: Points::new(),
            image: image::Image::new(1, 1),
            collision: physics::PolygonCollision {
                points: Points::new()
            },
            color,
            draw_mode,
            filled,
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.add_point(point);
        self.collision.points.add_point(point);
    }

    pub fn set_points(&mut self, points: Points) {
        self.points = points;
    }

    pub fn points(&self) -> Vec<&Point> {
        let mut output = Vec::new();
        for point in self.points.points.iter() {
            output.push(point);
        }
        output
    }
}

impl Polygon {
    pub fn translate_point(&mut self, index: usize, x: f32, y: f32, z: f32) {
        self.points.points[index].coord.0 = (self.points.points[index].coord.0 as f32 + x) as i32;
        self.points.points[index].coord.1 = (self.points.points[index].coord.1 as f32 + y) as i32;
        self.points.points[index].coord.2 = (self.points.points[index].coord.2 as f32 + z) as i32;
    }

    pub fn set_velocity_point(&mut self, index: usize, x: f32, y: f32, z: f32) {
        self.points.points[index].velocity.0 = x;
        self.points.points[index].velocity.1 = y;
        self.points.points[index].velocity.2 = z;
    }

    pub fn add_velocity_point(&mut self, index: usize, x: f32, y: f32, z: f32) {
        self.points.points[index].velocity.0 += x;
        self.points.points[index].velocity.1 += y;
        self.points.points[index].velocity.2 += z;
    }

    pub fn set_acceleration_point(&mut self, index: usize, x: f32, y: f32, z: f32) {
        self.points.points[index].acceleration.0 = x;
        self.points.points[index].acceleration.1 = y;
        self.points.points[index].acceleration.2 = z;
    }

    pub fn add_acceleration_point(&mut self, index: usize, x: f32, y: f32, z: f32) {
        self.points.points[index].acceleration.0 += x;
        self.points.points[index].acceleration.1 += y;
        self.points.points[index].acceleration.2 += z;
    }
}