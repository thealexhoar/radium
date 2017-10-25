use ecs::*;
use graphics::*;
use game::Blackboard;

pub struct ConsoleSystem {
    _draw_target: DrawTarget,
    _center_draw_target: DrawTarget,
    _width: u32,
    _height: u32,
    _window_x: u32,
    _window_y: u32
}

impl ConsoleSystem {
    pub fn new(
        width:u32,
        height:u32,
        window_x:u32,
        window_y:u32
    ) -> Self {
        let mut output = Self {
            _draw_target: DrawTarget::new(width, height),
            _center_draw_target: DrawTarget::new(width, height - 1),
            _width: width,
            _height: height,
            _window_x: window_x,
            _window_y: window_y
        };
        
        //TODO: Draw real borders
        let border_tile = Tile::new(
            Some(RGBColor::gray()),
            Some(RGBColor::black()),
            751//'+' as u32
        );
        output._draw_target.set_tiles_rect(
            Some(border_tile),
            0, 0,
            width, 1
        );

        output
    }
}

impl PassiveSystem for ConsoleSystem {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &ComponentManager,
        space: &Space,
        glyph_batch: &mut GlyphBatch,
        window: &mut Window,
        delta_time: f32
    ) {
        self._center_draw_target.clear();

        //TODO: implement console
        self._center_draw_target.draw_string_slice(
            "Console will go here",
            5, 1,
            RGBColor::yellow(),
            Some(RGBColor::black())
        );

        self._draw_target.set_from_draw_target(
            &self._center_draw_target, 
            0, 1
        ); 
        glyph_batch.draw_target.set_from_draw_target(
            &self._draw_target,
            self._window_x, self._window_y
        );
    }
}

