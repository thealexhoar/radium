extern crate sfml;
mod core;
mod graphics;
use core::Core;


fn main() {
    let mut core = Core::new(1920, 1080);
    core.init();
    core.run();
}