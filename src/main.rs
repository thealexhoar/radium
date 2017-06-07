extern crate sfml;
mod core;
mod ecs;
mod game;
mod graphics;
mod util;
mod control;
use core::Core;

fn main() {
    let mut width = 3200;
    let mut height = 1800;
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
}