use crate::engine::game;

pub trait CollisionObjectCommon {
    fn coord(&self) -> (i32, i32, i32);
    fn check_collision(&self, other: &dyn CollisionObjectCommon) -> bool;
    fn size(&self) -> (u32, u32, u32);
    fn out_of_bounds(&self, width: usize, height: usize, _depth: usize) -> bool {
        let (x, y, _z) = self.coord();
        let (w, h, _d) = self.size();
        if x - 5 < 0 || x + w as i32 + 5 > width as i32 || y - 5 < 0 || y + h as i32 + 5 > height as i32 {
            println!("Out of bounds");
            return true;
        }
        false
    }
}

pub struct PolygonCollision {
    pub points: game::Points,
}

impl CollisionObjectCommon for PolygonCollision {
    fn coord(&self) -> (i32, i32, i32) {
        self.points.coord()
    }

    fn size(&self) -> (u32, u32, u32) {
        self.points.size()
    }

    fn check_collision(&self, other: &dyn CollisionObjectCommon) -> bool {
        self.points.min_x() < other.coord().0 + other.size().0 as i32 &&
        self.points.max_x() > other.coord().0 &&
        self.points.min_y() < other.coord().1 + other.size().1 as i32 &&
        self.points.max_y() > other.coord().1 &&
        self.points.min_z() < other.coord().2 + other.size().2 as i32 &&
        self.points.max_z() > other.coord().2
    }
}

pub struct RectCollision {
    pub coord: (i32, i32, i32),
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl CollisionObjectCommon for RectCollision {
    fn coord(&self) -> (i32, i32, i32) {
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

        if x1 < x2 + w2 as i32 && x1 + w1 as i32 > x2 &&
           y1 < y2 + h2 as i32 && y1 + h1 as i32 > y2 &&
           z1 < z2 + d2 as i32 && z1 + d1 as i32 > z2 {
            return true;
        }
        false
    }
}

impl RectCollision {
    pub fn new(x: i32, y: i32, z: i32, width: u32, height: u32, depth: u32) -> RectCollision {
        RectCollision {
            coord: (x, y, z),
            width,
            height,
            depth
        }
    }
}

pub fn check_collision(obj1: &dyn CollisionObjectCommon, obj2: &dyn CollisionObjectCommon) -> bool {
    obj1.check_collision(obj2)
}