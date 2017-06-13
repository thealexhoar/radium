use ecs::*;
use game::action::Action;
use game::Blackboard;
use game::Direction;
use util::Point;
use std::cmp::max;

pub struct MoveAction {
    entity: Entity,
    direction: Direction,
    time: u32
}

impl MoveAction {
    pub fn new(entity: Entity, direction: Direction, time: u32) -> Self {
        Self {entity, direction, time}
    }
}

impl Action for MoveAction {
    fn execute(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        blackboard: &mut Blackboard,
        delta_time: f32
    ) -> (bool, u32) {
        let position_component =
            component_manager
            .get_mut::<PositionComponent>(self.entity)
            .unwrap();

        let (dx, dy, dz) = self.direction.to_delta();

        let mut allowed = true;
        //check if allowed

        if allowed {
            let next_point = Point::new (
                position_component.point.x + dx,
                position_component.point.y + dy,
                max(0, (position_component.point.z as i32) + dz) as u32
            );
            space.move_entity(
                self.entity,
                position_component.point,
                next_point
            );
            position_component.point = next_point;
            (true, self.time)
        }
        else {
            (true, 0)
        }
    }
}