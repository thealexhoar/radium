use util::Point;
use std::convert::Into;
use std::cmp::max;

pub fn bresenham(point0: Point, point1: Point) -> Vec<Point> {
    let mut p0 = point0;
    let mut p1 = point1;

    //calculate any transforms
    let mut dx = p1.x - p0.x;
    let mut dy = p1.y - p0.y;
    let steep = dy.abs() > dx.abs();
    if steep {
        p0.swap_in_place();
        p1.swap_in_place();
    }
    if p0.x > p1.x {
        let temp = p0;
        p0 = p1;
        p1 = temp;
    }

    //setup transformed conditions
    dx = p1.x - p0.x;
    dy = (p1.y - p0.y).abs();
    let mut err = dx / 2;
    let ystep = match p0.y < p1.y { true => 1, false => -1 };
    let mut y = p0.y;

    let mut line = Vec::new();
    for x in p0.x..(p1.x + 1) {
        //use i to maintain p0 -> p1 ordering in the output
        let i = match p0.x == point0.x {
            true  => (x - p0.x) as usize,
            false => 0
        };
        line.insert(i, match steep {
            true  => Point::new(y, x),
            false => Point::new(x, y)
        });
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    line
}



pub fn distance_direct(p0: Point, p1: Point) -> f32 {
    let delta = p1 - p0;
    let dx = delta.x as f32;
    let dy = delta.y as f32;
    (dx * dx + dy * dy).sqrt()
}

pub fn distance_manhattan(p0: Point, p1: Point) -> u32 {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    (dx.abs() + dy.abs()) as u32
}

//distance if diagonal movement is allowed
pub fn distance_diag(p0: Point, p1: Point) -> u32 {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    
    max(dx.abs(), dy.abs()) as u32
}