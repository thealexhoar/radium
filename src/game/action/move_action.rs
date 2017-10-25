use ecs::{ComponentManager, Entity, PositionComponent, Space};
use game::action::Action;
use game::Blackboard;
use game::component::{ColliderComponent, FloorComponent};
use game::Direction;
use util::Point;
use std::cmp::max;

pub struct MoveAction {
    _entity: Entity,
    _direction: Direction,
    _time: u32,
    _force: bool
}

impl MoveAction {
    pub fn new(
        entity: Entity,
        direction: Direction,
        time: u32
    ) -> Self {
        Self {
            _entity: entity,
            _direction: direction,
            _time: time,
            _force: false
        }
    }

    pub fn new_force(
        entity: Entity,
        direction: Direction,
        time: u32
    ) -> Self {
        Self {
            _entity: entity,
            _direction: direction,
            _time: time,
            _force: true
        }
    }
}

impl Action for MoveAction {
    fn execute(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        blackboard: &mut Blackboard,
        delta_time: f32
    ) -> (bool, bool, u32) {
        let current_point = match component_manager
            .get_mut::<PositionComponent>(self._entity)
        {
            Some(pc) => pc.point,
            None     => { return (false, false, 0) }
        };

        let (dx, dy, dz) = self._direction.to_delta();

        
        let next_point = Point::new (
            current_point.x + dx,
            current_point.y + dy,
            max(0, (current_point.z as i32) + dz) as u32
        );

        let mut allowed = true;
        //check if allowed
        let collision_bits = match component_manager
            .get::<ColliderComponent>(self._entity)
        {
            Some(cc) => cc.collision_bits,
            None     => 0
        };
        
        let stairs_needed = dz != 0;
        let mut stairs_present = false;

        match space.entities_at(next_point) {
            Some(entities) => {
                for entity in entities {
                    //check if something is in the way
                    let other_collision_bits = match component_manager
                        .get::<ColliderComponent>(*entity) 
                    {
                        Some(cc) => cc.collision_bits,
                        None     => { continue; }
                    };
                    if collision_bits & other_collision_bits > 0 {
                        allowed = false;
                        break;
                    }

                    let (allow_up, allow_down) = match component_manager
                        .get::<FloorComponent>(*entity)
                    {
                        Some(fc) => (fc.allow_up, fc.allow_down),
                        None      => (false, false)
                    };

                    if stairs_needed {
                        if dz > 0 {
                            stairs_present = stairs_present || allow_up;
                        }
                        else {
                            stairs_present = stairs_present || allow_down;
                        }
                    }

                }
            },
            None           => {}
        };
        if stairs_needed {
            allowed = allowed && stairs_present;
        }

        if !self._force {
            //check for "risky" movement
            let mut floor_present = false;

            match space.entities_at(next_point) {
                Some(entities) => {
                    for entity in entities {
                        //check if there's a floor
                        let has_floor = match component_manager
                            .get::<FloorComponent>(*entity) 
                        {
                            Some(fc) => true,
                            None     => false
                        };
                        
                        floor_present = floor_present || has_floor;

                    }
                },
                None           => {}
            };

            if !floor_present {
                allowed = false;
            }
        }



        if allowed {
            space.move_entity(
                self._entity,
                current_point,
                next_point
            );
            {
                let position_component =
                    component_manager
                    .get_mut::<PositionComponent>(self._entity)
                    .unwrap();
                
                position_component.point = next_point;
            }
            (true, false, self._time)
        }
        else {
            (true, false, 0)
        }
    }
}