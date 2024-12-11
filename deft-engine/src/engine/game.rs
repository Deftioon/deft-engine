use crate::engine::image;

pub trait GameObjectCommon {
    fn update(&mut self);
    fn check_collision(&self, other: &dyn GameObjectCommon) -> bool;
    fn coord(&self) -> (u32, u32, u32);
    fn size(&self) -> (u32, u32, u32);
    fn collision(&self) -> &dyn CollisionObjectCommon;
    fn image(&self) -> &image::Image;
}



pub struct Box {
    pub coord: (u32, u32, u32),
    pub velocity: (f32, f32, f32),
    pub acceleration: (f32, f32, f32),
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub image: image::Image,
    pub collision: BoxCollision
}

impl Box {
    pub fn new2d(x: u32, y: u32, width: u32, height: u32, color: u32) -> Box {
        Box {
            coord: (x, y, 0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            width,
            height,
            depth: 0,
            image: image::Image::new_filled(color, width as usize, height as usize),
            collision: BoxCollision {
                coord: (x, y, 0),
                width,
                height,
                depth: 0
            }
        }
    }
}

impl GameObjectCommon for Box {
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

pub trait CollisionObjectCommon {
    fn coord(&self) -> (u32, u32, u32);
    fn check_collision(&self, other: &dyn CollisionObjectCommon) -> bool;
    fn size(&self) -> (u32, u32, u32);
}

pub struct BoxCollision {
    pub coord: (u32, u32, u32),
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl BoxCollision {
    pub fn new(x: u32, y: u32, z: u32, width: u32, height: u32, depth: u32) -> BoxCollision {
        BoxCollision {
            coord: (x, y, z),
            width,
            height,
            depth
        }
    }
}

impl CollisionObjectCommon for BoxCollision {
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