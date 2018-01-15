use core_manager::CoreState;
use ecs::{Engine};
use game::{Blackboard};
use game::ui::{MouseInput, MouseInterface};
use graphics::{GlyphBatch, RGBColor, Window};
use menu::{MenuObject, MenuType};

pub struct Label {
    _x: u32,
    _y: u32,
    _text: String,
    _fg_color: RGBColor,
    _bg_color: Option<RGBColor>
}

impl Label {
    pub fn new(
        x: u32, y: u32,
        text: String,
        fg_color: RGBColor,
        bg_color: Option<RGBColor>
    ) -> Self {
        Self {
            _x: x,
            _y: y,
            _text: text,
            _fg_color: fg_color,
            _bg_color: bg_color
        }
    }
}

impl MenuObject for Label {
    fn select(&mut self) -> bool {
        true
    }

    fn deselect(&mut self) -> bool {
        true
    }

    fn update(
        &mut self,
        x: u32,
        y: u32,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        window: &mut Window
    ) -> Option<CoreState> {
        glyph_batch.draw_target.draw_string(
            &self._text,
            self._x + x,
            self._y + y,
            RGBColor::red(),
            Some(RGBColor::gray())
        );
        None
    }

}

