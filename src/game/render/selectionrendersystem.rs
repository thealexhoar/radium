use ecs::*;
use graphics::*;
use game::Blackboard;

pub struct SelectionRenderSystem {
    _width: u32,
    _height: u32,
    _accumulator: f32,
    _pulse_up: bool,
    _pulse_time: f32
}

impl SelectionRenderSystem {
    pub fn new(
        width:u32,
        height:u32
    ) -> Self {
        Self {
            _width: width,
            _height: height,
            _accumulator: 0.0,
            _pulse_up: true,
            _pulse_time: 1.0,
        }
    }
}

impl PassiveSystem for SelectionRenderSystem {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &ComponentManager,
        space: &Space,
        glyphbatch: &mut GlyphBatch,
        window: &mut Window,
        delta_time: f32
    ) {
        self._accumulator += delta_time;
        while self._accumulator > self._pulse_time {
            self._accumulator -= self._pulse_time;
            self._pulse_up = !self._pulse_up;
        }

        let (world_x, world_y, world_z) = match blackboard.camera {
            Some(ref camera) => {
                (
                    camera.x - (self._width as i32) / 2,
                    camera.y - (self._height as i32) / 2,
                    camera.z
                )
            },
            None => { return; }
        };

        let color_val = match self._pulse_up {
            true  => 20 + (120.0 * self._accumulator / self._pulse_time) as u8,
            false => 140 - (120.0 * self._accumulator / self._pulse_time) as u8
        };
        let tile = Tile::new(
            None,
            Some(RGBColor::new_from_rgb(color_val, color_val, color_val)),
            0
        );

        let entity_option = blackboard.current_entity;
        let entity = match entity_option {
            Some(e) => e,
            None    => { return; }
        };

        let position_component_option = component_manager
            .get::<PositionComponent>(entity);
        let position_component = match position_component_option {
            Some(pc) => pc,
            None     => { return; }
        };

        if position_component.point.x < world_x
            || position_component.point.y < world_y
            || position_component.point.x >= world_x + self._width as i32
            || position_component.point.y >= world_y + self._height as i32
            || position_component.point.z != world_z
        {
            return;
        }

        let x = (position_component.point.x - world_x) as u32;
        let y = (position_component.point.y - world_y) as u32;

        glyphbatch.drawtarget.overlay_tile(Some(tile), x, y);

    }
}