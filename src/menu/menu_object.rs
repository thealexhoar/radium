use core_manager::CoreState;
use ecs::*;
use game::*;
use game::ui::*;
use graphics::*;
use menu::*;

//create a tree-based menu object system
//a leaf node would be a checkbox, "slider", nav buttons, or toggle buttons
//  checkboxes, sliders, and toggles for settings
//  nav buttons for menu navigations
//higher level tree nodes would be:
//  pages (for complete menu context switching)
//  vertical static lists
//  vertical scrolling lists
//  (maybe just make vertical lists have scrolling only when necessary)
//  horizontal lists

pub trait MenuObject {
    fn select(&mut self);
    fn deselect(&mut self);
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        window: &mut Window
    ) -> Option<CoreState>;
}