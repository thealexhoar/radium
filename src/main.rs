extern crate sfml;
mod coremanager;
mod ecs;
mod game;
mod graphics;
mod util;
use game::GameCore;
use coremanager::CoreManager;

fn main() {
    let mut width = 2560;
    let mut height = 1440;
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