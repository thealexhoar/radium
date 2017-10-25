use core_manager::CoreState;
use ecs::*;
use game::*;
use game::ui::*;
use graphics::*;
use menu::*;

pub struct GameMenu {

}

impl GameMenu {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn update (
        &mut self,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        window: &mut Window
    ) -> CoreState {
        CoreState::Menu(MenuType::Game)
    }
}