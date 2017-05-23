use control::Controller;
use ecs::*;
use graphics::*;


pub struct PlayerController {}

impl PlayerController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for PlayerController {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        window: &mut Window,
        entity: Entity,
        delta_time: u32
    ) -> u32 {
        match window.wait_for_event() {
            Event::None => 0,
            Event::KeyPress{code, alt, ctrl, shift} => {
                let position_component = component_manager
                    .get_mut::<PositionComponent>(entity)
                    .unwrap();
                let (x,y) = match (code, alt, ctrl, shift) {
                    ('a', false, false, false) => (-1, 0),
                    ('d', false, false, false) => (1, 0),
                    ('s', false, false, false) => (0, 1),
                    ('w', false, false, false) => (0, -1),
                    _ => {return 0;}
                
                };
                
                space.remove_entity(entity, position_component.point);
                position_component.point.x += x;
                position_component.point.y += y;
                space.add_entity_at(entity, position_component.point);
                0
            },
            _           => 0
        }
    }

}