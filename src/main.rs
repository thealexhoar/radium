extern crate sfml;
mod behavior;
mod core;
mod ecs;
mod game;
mod graphics;
mod util;
use util::math;
use util::Point;
use core::Core;

fn main() {
    let mut width = 1920;
    let mut height = 1080;
    let args = util::clargs::collect();
    for (index, arg) in args.iter().enumerate() {
        if arg == "--size" || arg == "-S" {
            width = args[index + 1].parse::<u32>().unwrap();
            height = args[index + 2].parse::<u32>().unwrap();
        }
    }
    let mut core = Core::new(width, height);
    core.init();
    core.run();
    println!("b");
    let line = math::bresenham(Point::new(10, 10), Point::new(12, 4));
    for pos in line {
        println!("x: {} y: {}", pos.x, pos.y);
    }

}