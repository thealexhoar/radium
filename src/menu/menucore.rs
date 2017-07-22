use coremanager::CoreState;
use ecs::*;
use game::*;
use game::ui::*;
use graphics::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuType {
    Main,
    Game,
    Config
}

pub struct MenuCore {

}


impl MenuCore {
    pub fn new() -> Self {
        Self { 
             
        }
    }

    pub fn init(
        &mut self,
        blackboard: &mut Blackboard
    ) {

    }

    pub fn update(
        &mut self,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyphbatch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        scheduler: &mut Scheduler,
        window: &mut Window
    ) -> CoreState {
        CoreState::Menu(MenuType::Main)
    }
}