use ecs::*;
use graphics::*;
use util::Point;
use game::graphics::TileComponent;
use game::Blackboard;
use game::graphics::Camera;

pub struct SelectionRenderSystem {
    width: u32,
    height: u32,
    accumulator: f32,
    pulse_up: bool,
    pulse_time: f32
}

impl SelectionRenderSystem {
    pub fn new(
        width:u32,
        height:u32
    ) -> Self {
        Self {
            width,
            height,
            accumulator: 0.0,
            pulse_up: true,
            pulse_time: 1.0,
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
        self.accumulator += delta_time;
        while self.accumulator > self.pulse_time {
            self.accumulator -= self.pulse_time;
            self.pulse_up = !self.pulse_up;
        }

        let (world_x, world_y, world_z) = match blackboard.camera {
            Some(ref camera) => {
                (
                    camera.x - (self.width as i32) / 2,
                    camera.y - (self.height as i32) / 2,
                    camera.z
                )
            },
            None => { return; }
        };

        let color_val = match self.pulse_up {
            true  => 20 + (40.0 * self.accumulator / self.pulse_time) as u8,
            false => 60 - (40.0 * self.accumulator / self.pulse_time) as u8
        };
        let tile = Tile::new(
            None,
            Some(Color::new_from_rgb(color_val, color_val, color_val)),
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
            || position_component.point.x >= world_x + self.width as i32
            || position_component.point.y >= world_y + self.height as i32
            || position_component.point.z != world_z
        {
            return;
        }

        let x = (position_component.point.x - world_x) as u32;
        let y = (position_component.point.y - world_y) as u32;

        glyphbatch.drawtarget.overlay_tile(Some(tile), x, y);

    }
}