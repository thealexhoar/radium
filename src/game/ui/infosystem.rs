use ecs::*;
use graphics::*;
use game::Blackboard;

pub struct InfoSystem {
    _draw_target: DrawTarget,
    _center_draw_target: DrawTarget,
    _width: u32,
    _height: u32,
    _window_x: u32,
    _window_y: u32
}

impl InfoSystem {
    pub fn new(
        width:u32,
        height:u32,
        window_x:u32,
        window_y:u32
    ) -> Self {
        let mut output = Self {
            _draw_target: DrawTarget::new(width, height),
            _center_draw_target: DrawTarget::new(width - 2, height - 2),
            _width: width,
            _height: height,
            _window_x: window_x,
            _window_y: window_y
        };
        
        //TODO: Draw real borders
        let border_tile = Tile::new(
            Some(Color::black()),
            Some(Color::gray()),
            '+' as u32
        );
        output._draw_target.set_tiles_rect(
            Some(border_tile),
            0, 0,
            width, 1
        );
        output._draw_target.set_tiles_rect(
            Some(border_tile),
            0, height - 1,
            width, 1
        );
        output._draw_target.set_tiles_rect(
            Some(border_tile),
            0, 0,
            1, height
        );
        output._draw_target.set_tiles_rect(
            Some(border_tile),
            width - 1, 0,
            1, height
        );

        output
    }
}

impl PassiveSystem for InfoSystem {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &ComponentManager,
        space: &Space,
        glyphbatch: &mut GlyphBatch,
        window: &mut Window,
        delta_time: f32 
    ) {
        self._center_draw_target.clear();

        //TODO: implement console
        self._center_draw_target.draw_string_slice(
            "Info will go here",
            5, 1,
            Color::yellow(),
            Some(Color::black())
        );

        self._draw_target.set_from_drawtarget(
            &self._center_draw_target, 
            1, 1
        ); 
        glyphbatch.drawtarget.set_from_drawtarget(
            &self._draw_target,
            self._window_x, self._window_y
        );
    }
}