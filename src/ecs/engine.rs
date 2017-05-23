use control::*;
use ecs::*;
use util::{Point, PriorityQueue};
use graphics::{Color, Tile, Window};
use graphics::Event as WindowEvent;
use game::graphics::TileComponent;
use game::player::PlayerController;
use control::Controller;
use std::ops::{Deref, DerefMut};
use std::collections::{HashMap, HashSet};

pub struct Engine {
    pub passive_systems: Vec<Box<PassiveSystem>>,
    pub continuous_systems: Vec<Box<ContinuousSystem>>,
    _entities: Vec<Entity>,
    _controllers: HashMap<Entity, Box<Controller>>,
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
            _controllers: HashMap::new(),
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
                Some(Color::white()),
                Some(Color::blue()),
                '@' as u32
            )
        ));

        self._space.add_entity_at(player, Point::new(0, 0));

        self._controllers.insert(player, Box::new(PlayerController::new()));
        self._scheduler.push_entity(player, 0);

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
        let controller = match self._controllers.get_mut(&entity) {
            Some(controller_box) => controller_box.deref_mut(),
            None                 => { return; }
        };      

        let next_dt = controller.update(
            &mut self._component_manager,
            &mut self._space,
            window,
            entity,
            dt
        );

        self._scheduler.push_entity(entity, next_dt);
        //then return entity to the scheduler
    }
}
