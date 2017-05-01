extern crate sfml;
mod core;
mod ecs;
mod graphics;
mod util;
use core::Core;
use util::clargs;


fn main() {
    let mut width = 1920;
    let mut height = 1080;
    let args = clargs::collect();
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