extern crate sfml;
mod core;
mod ecs;
mod graphics;
mod util;
use core::Core;


fn main() {
    let mut core = Core::new(1920, 1080);
    core.init();
    core.run();
}