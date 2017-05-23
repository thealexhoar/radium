use ecs::*;
use util::{Point, PriorityQueue};
use std::ops::Deref;
use graphics::{Color, Tile, Window};
use graphics::Event as WindowEvent;
use control::ControllerComponent;
use game::graphics::TileComponent;

use std::ops::DerefMut;

pub struct Engine {
    pub passive_systems: Vec<Box<PassiveSystem>>,
    pub continuous_systems: Vec<Box<ContinuousSystem>>,
    _entities: Vec<Entity>,
    _component_manager: ComponentManager,
    _scheduler: Scheduler,
    _space: Space
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            passive_systems: Vec::new(),
            continuous_systems: Vec::new(),
            _entities: Vec::new(),
            _component_manager: ComponentManager::new(),
            _scheduler: Scheduler::new(),
            _space: Space::new()
        }
    }

    pub fn load(&mut self) {
        self._space.load_chunk(0,0);

        let player = self._component_manager.create_entity();
        self._component_manager.set(player, PositionComponent::new(0, 0));
        self._component_manager.set(player, TileComponent::new(
            Tile::new(
                Some(Color::green()),
                Some(Color::black()),
                '@' as u32
            )
        ));

        self._space.add_entity_at(player, Point::new(0, 0));

    }

    pub fn update (
        &mut self, 
        window: &mut Window,
        true_delta_time:f32
    ) {
        for passive_system in self.passive_systems.iter_mut() {
            passive_system.deref_mut().update(
                &mut self._component_manager,
                &mut self._space,
                window,
                true_delta_time
            );
        }

        let (entity, dt) = match self._scheduler.pop_entity() {
            Some((entity, dt)) => (entity, dt),
            None               => { return; }
        };

        for continuous_system in self.continuous_systems.iter_mut() {
            continuous_system.deref_mut().update(
                &mut self._component_manager,
                &mut self._space,
                dt
            );
        }
        let controller_component_option = 
            self._component_manager.get_mut::<ControllerComponent>(entity);
        let controller = match controller_component_option {
            Some(controller_component) => &mut controller_component.controller,
            None                       => { return; }
        };
    }
}
