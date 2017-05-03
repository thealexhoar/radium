use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x,y}
    }

    pub fn tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn from_tuple(tuple:(i32, i32)) -> Self {
        let (x, y) = tuple;
        Self {x, y}
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { 
            x: self.x + other.x, 
            y: self.y + other.y
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { 
            x: self.x - other.x, 
            y: self.y - other.y
        }
    }
}