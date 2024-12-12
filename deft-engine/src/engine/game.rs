use crate::engine::image;

pub trait GameObjectCommon {
    fn update(&mut self);
    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool;
    fn coord(&self) -> (u32, u32, u32);
    fn size(&self) -> (u32, u32, u32);
    fn collision(&self) -> &dyn CollisionObjectCommon;
    fn image(&self) -> &image::Image;
}

pub trait CollisionObjectCommon {
    fn coord(&self) -> (u32, u32, u32);
    fn check_collision(&self, other: &dyn CollisionObjectCommon) -> bool;
    fn size(&self) -> (u32, u32, u32);
}

pub struct Rect {
    pub coord: (u32, u32, u32),
    pub velocity: (f32, f32, f32),
    pub acceleration: (f32, f32, f32),
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub image: image::Image,
    pub collision: RectCollision
}

impl GameObjectCommon for Rect {
    fn update(&mut self) {
        self.coord.0 = (self.coord.0 as f32 + self.velocity.0) as u32;
        self.coord.1 = (self.coord.1 as f32 + self.velocity.1) as u32;
        self.coord.2 = (self.coord.2 as f32 + self.velocity.2) as u32;

        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;
    }

    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool {
        self.collision.check_collision(other.collision())
    }

    fn coord(&self) -> (u32, u32, u32) {
        self.coord
    }

    fn size(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.depth)
    }

    fn collision(&self) -> &dyn CollisionObjectCommon {
        &self.collision
    }

    fn image(&self) -> &image::Image {
        &self.image
    }
}

impl Rect {
    pub fn new2d(x: u32, y: u32, width: u32, height: u32, color: u32) -> Rect {
        Rect {
            coord: (x, y, 0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            width,
            height,
            depth: 0,
            image: image::Image::new_filled(color, width as usize, height as usize),
            collision: RectCollision {
                coord: (x, y, 0),
                width,
                height,
                depth: 0
            }
        }
    }
}
pub struct Point {
    pub coord: (u32, u32, u32),
    pub velocity: (f32, f32, f32),
    pub acceleration: (f32, f32, f32)
}

impl Point {
    pub fn new(x: u32, y: u32, z: u32) -> Point {
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

impl Points {
    pub fn new() -> Points {
        Points {
            points: Vec::new()
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn min_x(&self) -> u32 {
        let mut min = self.points[0].coord.0;
        for point in self.points.iter() {
            if point.coord.0 < min {
                min = point.coord.0;
            }
        }
        min
    }

    pub fn max_x(&self) -> u32 {
        let mut max = self.points[0].coord.0;
        for point in self.points.iter() {
            if point.coord.0 > max {
                max = point.coord.0;
            }
        }
        max
    }

    pub fn min_y(&self) -> u32 {
        let mut min = self.points[0].coord.1;
        for point in self.points.iter() {
            if point.coord.1 < min {
                min = point.coord.1;
            }
        }
        min
    }

    pub fn max_y(&self) -> u32 {
        let mut max = self.points[0].coord.1;
        for point in self.points.iter() {
            if point.coord.1 > max {
                max = point.coord.1;
            }
        }
        max
    }

    pub fn min_z(&self) -> u32 {
        let mut min = self.points[0].coord.2;
        for point in self.points.iter() {
            if point.coord.2 < min {
                min = point.coord.2;
            }
        }
        min
    }

    pub fn max_z(&self) -> u32 {
        let mut max = self.points[0].coord.2;
        for point in self.points.iter() {
            if point.coord.2 > max {
                max = point.coord.2;
            }
        }
        max
    }

    pub fn center(&self) -> (u32, u32, u32) {
        let min_x = self.min_x();
        let max_x = self.max_x();
        let min_y = self.min_y();
        let max_y = self.max_y();
        let min_z = self.min_z();
        let max_z = self.max_z();

        ((min_x + max_x) / 2, (min_y + max_y) / 2, (min_z + max_z) / 2)
    }

    pub fn width(&self) -> u32 {
        self.max_x() - self.min_x()
    }

    pub fn height(&self) -> u32 {
        self.max_y() - self.min_y()
    }

    pub fn depth(&self) -> u32 {
        self.max_z() - self.min_z()
    }

    pub fn coord(&self) -> (u32, u32, u32) {
        (self.min_x(), self.min_y(), self.min_z())
    }

    pub fn size(&self) -> (u32, u32, u32) {
        (self.width(), self.height(), self.depth())
    }
}

pub struct Polygon {
    pub points: Points,
    pub image: image::Image,
    pub collision: PolygonCollision,
    pub color: u32
}

impl GameObjectCommon for Polygon {
    fn update(&mut self) {
        for point in self.points.points.iter_mut() {
            point.coord.0 = (point.coord.0 as f32 + point.velocity.0) as u32;
            point.coord.1 = (point.coord.1 as f32 + point.velocity.1) as u32;
            point.coord.2 = (point.coord.2 as f32 + point.velocity.2) as u32;

            point.velocity.0 += point.acceleration.0;
            point.velocity.1 += point.acceleration.1;
            point.velocity.2 += point.acceleration.2;
        }
    }

    fn coord(&self) -> (u32, u32, u32) {
        self.points.coord()
    }

    fn size(&self) -> (u32, u32, u32) {
        self.points.size()
    }

    fn collision(&self) -> &dyn CollisionObjectCommon {
        &self.collision
    }

    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool {
        self.collision.check_collision(other.collision())
    }

    fn image(&self) -> &image::Image {
        &self.image
    }

}

impl Polygon {
    pub fn new(color: u32) -> Polygon {
        Polygon {
            points: Points::new(),
            image: image::Image::new(1, 1),
            collision: PolygonCollision {
                points: Points::new()
            },
            color
        }
    }

    pub fn new2d(color: u32) -> Polygon {
        Polygon {
            points: Points::new(),
            image: image::Image::new(1, 1),
            collision: PolygonCollision {
                points: Points::new()
            },
            color
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.add_point(point);
    }

    pub fn set_points(&mut self, points: Points) {
        self.points = points;
    }
}

impl Polygon {
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.coord.0 = (point.coord.0 as f32 + x) as u32;
            point.coord.1 = (point.coord.1 as f32 + y) as u32;
            point.coord.2 = (point.coord.2 as f32 + z) as u32;
        }
    }

    pub fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.velocity.0 = x;
            point.velocity.1 = y;
            point.velocity.2 = z;
        }
    }

    pub fn add_velocity(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.velocity.0 += x;
            point.velocity.1 += y;
            point.velocity.2 += z;
        }
    }

    pub fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.acceleration.0 = x;
            point.acceleration.1 = y;
            point.acceleration.2 = z;
        }
    }

    pub fn add_acceleration(&mut self, x: f32, y: f32, z: f32) {
        for point in self.points.points.iter_mut() {
            point.acceleration.0 += x;
            point.acceleration.1 += y;
            point.acceleration.2 += z;
        }
    }

    pub fn translate_point(&mut self, index: usize, x: f32, y: f32, z: f32) {
        self.points.points[index].coord.0 = (self.points.points[index].coord.0 as f32 + x) as u32;
        self.points.points[index].coord.1 = (self.points.points[index].coord.1 as f32 + y) as u32;
        self.points.points[index].coord.2 = (self.points.points[index].coord.2 as f32 + z) as u32;
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

pub struct PolygonCollision {
    pub points: Points,
}

impl CollisionObjectCommon for PolygonCollision {
    fn coord(&self) -> (u32, u32, u32) {
        self.points.coord()
    }

    fn size(&self) -> (u32, u32, u32) {
        self.points.size()
    }

    fn check_collision(&self, other: &dyn CollisionObjectCommon) -> bool {
        self.points.min_x() < other.coord().0 + other.size().0 &&
        self.points.max_x() > other.coord().0 &&
        self.points.min_y() < other.coord().1 + other.size().1 &&
        self.points.max_y() > other.coord().1 &&
        self.points.min_z() < other.coord().2 + other.size().2 &&
        self.points.max_z() > other.coord().2
    }
}

pub struct RectCollision {
    pub coord: (u32, u32, u32),
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl CollisionObjectCommon for RectCollision {
    fn coord(&self) -> (u32, u32, u32) {
        self.coord
    }

    fn size(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.depth)
    }

    fn check_collision(&self, other: &dyn CollisionObjectCommon) -> bool {
        let (x1, y1, z1) = other.coord();
        let (x2, y2, z2) = self.coord();
        let (w1, h1, d1) = other.size();
        let (w2, h2, d2) = self.size();

        if x1 < x2 + w2 && x1 + w1 > x2 &&
           y1 < y2 + h2 && y1 + h1 > y2 &&
           z1 < z2 + d2 && z1 + d1 > z2 {
            return true;
        }
        false
    }
}

impl RectCollision {
    pub fn new(x: u32, y: u32, z: u32, width: u32, height: u32, depth: u32) -> RectCollision {
        RectCollision {
            coord: (x, y, z),
            width,
            height,
            depth
        }
    }
}
