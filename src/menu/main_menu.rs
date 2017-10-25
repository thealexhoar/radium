use core_manager::CoreState;
use ecs::*;
use game::*;
use game::ui::*;
use graphics::*;
use menu::*;

pub struct MainMenu {

}

impl MainMenu {
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

        //render to drawtarget here

        window.clear();
        glyph_batch.flush_tiles();
        window.draw_glyph_batch(&glyph_batch);

        //update state here

        CoreState::Menu(MenuType::Main)
    }
}