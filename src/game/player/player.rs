use control::Controller;
use ecs::*;
use graphics::*;
use game::Blackboard;
use game::components::ColliderComponent;
use util::Point;


pub struct PlayerController {}

impl PlayerController {
    pub fn new() -> Self {
        Self {}
    }
    
    fn move_player(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        player: Entity,
        dx: i32,
        dy: i32
    ) -> u32 {
        
        let mut new_point: Point = {
            let position_component = component_manager
                .get_mut::<PositionComponent>(player)
                .unwrap();

            Point::new(
                position_component.point.x + dx,
                position_component.point.y + dy
            )
        };

        let mut move_allowed = {
            let collider_component = component_manager
                .get::<ColliderComponent>(player)
                .unwrap();
            
            let collider_bits = collider_component.collision_bits;

            match space.entities_at(new_point) {
                Some(vector) => vector.iter()
                    .map(|entity: &Entity| -> bool {
                        match component_manager
                            .get::<ColliderComponent>(*entity) {
                            Some(cc) => cc.collision_bits & collider_bits == 0,
                            None     => true
                        }
                    })
                    .fold(true, |accum: bool, allow: bool| accum && allow),
                None => true
            }
        };

        let position_component = component_manager
            .get_mut::<PositionComponent>(player)
            .unwrap();

        if move_allowed {
            space.move_entity(
                player,
                position_component.point,
                new_point
            );
            position_component.point = new_point;
            return 1000;
        }
        else {
            return 0;
        }
    }
}

impl Controller for PlayerController {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        window: &mut Window,
        entity: Entity
    ) -> (u32, bool) {
        let mut event_found = false;
        let mut out = 0;
        for event in window.events() {
            out = match event {
                Event::None => 0,
                Event::KeyPress{code, alt, ctrl, shift} => {
                    event_found = true;
                    match (code, alt, ctrl, shift) {
                        ('a', false, false, false) => self.move_player(
                            blackboard,
                            component_manager,
                            space,
                            entity,
                            -1, 0
                        ),
                        ('d', false, false, false) => self.move_player(
                            blackboard,
                            component_manager,
                            space,
                            entity,
                            1, 0
                        ),
                        ('s', false, false, false) => self.move_player(
                            blackboard,
                            component_manager,
                            space,
                            entity,
                            0, 1
                        ),
                        ('w', false, false, false) => self.move_player(
                            blackboard,
                            component_manager,
                            space,
                            entity,
                            0, -1
                        ),
                        _ => { event_found = false; 0}
                    }
                },
                _ => 0
            };
            if event_found {
                break;
            }
        }
        (out, event_found)
    }

}