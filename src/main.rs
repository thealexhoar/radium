extern crate sfml;
mod core_manager;
mod ecs;
mod game;
mod graphics;
mod menu;
mod util;
use core_manager::CoreManager;

fn main() {
    let mut width = 1280;
    let mut height = 720;
    let args = util::clargs::collect();
    for (index, arg) in args.iter().enumerate() {
        if arg == "--size" || arg == "-S" {
            width = args[index + 1].parse::<u32>().unwrap();
            height = args[index + 2].parse::<u32>().unwrap();
        }
    }

    let mut core_manager = CoreManager::new(width, height);
    core_manager.run();
}