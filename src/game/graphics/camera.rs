use ecs::*;

pub struct Camera {
    pub x: i32, 
    pub y: i32,
    pub z: u32
}

impl Camera {
    pub fn new(x: i32, y: i32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn set(&mut self, x: i32, y: i32, z: u32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn move_north(&mut self) {
        self.y -= 1;
    }

    pub fn move_south(&mut self) {
        self.y += 1;
    }

    pub fn move_west(&mut self) {
        self.x -= 1;
    }

    pub fn move_east(&mut self) {
        self.x += 1;
    }

    pub fn move_up(&mut self) {
        self.z += 1;
        println!("{}", self.z);
    }

    pub fn move_down(&mut self) {
        if self.z > 0 {
            self.z -= 1;
        }
        println!("{}", self.z);
    }
}