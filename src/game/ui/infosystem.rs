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
            _center_draw_target: DrawTarget::new(width - 1, height),
            _width: width,
            _height: height,
            _window_x: window_x,
            _window_y: window_y
        };

        //TODO: Draw real borders
        let border_tile = Tile::new(
            Some(RGBColor::black()),
            Some(RGBColor::gray()),
            '+' as u32
        );
        output._draw_target.set_tiles_rect(
            Some(border_tile),
            0, 0,
            1, height
        );
        output
    }

    fn draw_camera_info(&mut self, row:u32, blackboard:&mut Blackboard) {
        self._center_draw_target.draw_string_slice(
            "Camera Center",
            0, row,
            RGBColor::yellow(),
            Some(RGBColor::black())
        );
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        match blackboard.camera {
            Some(ref cam) => {
                x = cam.x;
                y = cam.y;
                z = cam.z;
            },
            None => {}
        };

        self._center_draw_target.draw_string_slice(
            &(x.to_string() + "," + &y.to_string() + "," + &z.to_string()),
            1, row + 1,
            RGBColor::yellow(),
            Some(RGBColor::black())
        );
    }

    fn draw_turn_info(&mut self, row:u32, blackboard:&mut Blackboard) {
        self._center_draw_target.draw_string_slice(
            "Turn Time",
            0, row,
            RGBColor::yellow(),
            Some(RGBColor::black())
        );

        let time = blackboard.current_action_time;
        let max_time = blackboard.max_action_time;

        self._center_draw_target.draw_string_slice(
            &(time.to_string() + "/" + &max_time.to_string()),
            1, row + 1,
            RGBColor::yellow(),
            Some(RGBColor::black())
        );
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
        self.draw_camera_info(1, blackboard);
        self.draw_turn_info(3, blackboard);

        self._draw_target.set_from_drawtarget(
            &self._center_draw_target,
            1, 0
        );
        glyphbatch.drawtarget.set_from_drawtarget(
            &self._draw_target,
            self._window_x, self._window_y
        );
    }
}