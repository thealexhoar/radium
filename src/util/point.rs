use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: u32
}

impl Point {
    pub fn new(x: i32, y: i32, z:u32) -> Self {
        Self {x, y, z}
    }

    pub fn from_tuple(tuple:(i32, i32, u32)) -> Self {
        let (x, y, z) = tuple;
        Self {x, y, z}
    }
    
    pub fn tuple(&self) -> (i32, i32, u32) {
        (self.x, self.y, self.z)
    }

    pub fn swap(&self) -> Self {
        Self::new(self.y, self.x, self.z)
    }

    pub fn swap_in_place(&mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: match self.z < other.z {
                true  => 0,
                false => self.z - other.z
            }
        }
    }
}