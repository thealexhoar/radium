extern crate sfml;
mod core;
mod ecs;
mod game;
mod graphics;
mod util;
use core::Core;

fn main() {
    let mut width = 2400;
    let mut height = 1350;
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