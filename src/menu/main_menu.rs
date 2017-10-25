use core_manager::CoreState;
use ecs::*;
use game::*;
use game::ui::*;
use graphics::*;
use menu::*;

pub struct MainMenu {
    _root_menu_object: Box<MenuObject>
}

impl MainMenu {
    pub fn new(root_menu_object: Box<MenuObject>) -> Self {
        let mut out = Self {
            _root_menu_object: root_menu_object
        };
        out._root_menu_object.select();
        out
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
        self._root_menu_object.update(
            0, 0,
            blackboard,
            engine,
            glyph_batch,
            mouse_interface,
            window
        );

        window.clear();
        glyph_batch.flush_tiles();
        window.draw_glyph_batch(&glyph_batch);

        let events = window.events();
        let mut out_state = CoreState::Menu(MenuType::Main);
        //update state here

        //test

        for event in events.iter() {
            match event {
                &Event::KeyPress{code, alt, ctrl, shift} => {
                    match (code, alt, ctrl, shift) {
                        (Key::Return, false, false, false) => {
                            out_state = CoreState::Game;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }


        window.update_event_queue();

        out_state
    }
}