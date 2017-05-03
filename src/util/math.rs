use util::Point;

pub fn bresenham_line(p0: Point, p1: Point) -> Vec<(i32, i32)> {
    let mut delta = p1 - p0;

    Vec::new()

}

//from roguebasin
/*
  3D Distance = Max(D,H) + Min(D,H)/2
    D = 2D Distance (x,y)
    H = Difference in height
    */

pub fn distance_direct(p0: Point, p1: Point) -> f32 {
    let delta = p1 - p0;
    //let (dx, dy) = delta.tuple() as (f32, f32);
    0.
}