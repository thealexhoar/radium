use control::*;
use ecs::*;
use util::{Point, PriorityQueue};
use graphics::{Color, GlyphBatch, Tile, Window};
use graphics::Event as WindowEvent;
use game::graphics::TileComponent;
use game::player::PlayerController;
use game::components::ColliderComponent;
use game::Blackboard;
use control::Controller;
use std::ops::{Deref, DerefMut};
use std::collections::{HashMap, HashSet};

pub struct Engine {
    pub passive_systems: Vec<Box<PassiveSystem>>,
    pub continuous_systems: Vec<Box<ContinuousSystem>>,
    _blackboard: Blackboard,
    _controllers: HashMap<Entity, Box<Controller>>,
    _component_manager: ComponentManager,
    _entities: Vec<Entity>,
    _scheduler: Scheduler,
    _space: Space
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            passive_systems: Vec::new(),
            continuous_systems: Vec::new(),
            _blackboard: Blackboard::new(),
            _controllers: HashMap::new(),
            _component_manager: ComponentManager::new(),
            _entities: Vec::new(),
            _scheduler: Scheduler::new(),
            _space: Space::new()
        }
    }

    pub fn load(&mut self) {
        for i in -10..10 {
            for j in -10..10 {
                self._space.load_chunk(i,j);
            }
        }

        let player = self._component_manager.create_entity();
        self._component_manager.set(player, PositionComponent::new(1, 1, 1));
        self._component_manager.set(player, TileComponent::new(
            Tile::new(
                Some(Color::new_from_rgb_f(0.6, 0.8, 1.0)),
                None,
                '@' as u32
            )
        ));
        self._component_manager.set(player, ColliderComponent::new(1));

        self._space.add_entity_at(player, Point::new(1, 1));

        self._controllers.insert(player, Box::new(PlayerController::new()));
        self._scheduler.push_entity(player, 0);
        self._blackboard.player = Some(player);

        let tile_fg = Color::new_from_rgb(50,50,50);
        let tile_bg = Color::new_from_rgb(11,11,22);
        let floor_tile = Tile::new(
            Some(tile_fg),
            Some(tile_bg),
            '.' as u32
        );
        let wall_tile = Tile::new(
            Some(Color::white()),
            Some(tile_bg),
            '#' as u32
        );

        for i in 0..40 {
            for j in 0..25 {
                let entity = self._component_manager.create_entity();
                self._component_manager.set(
                    entity,
                    PositionComponent::new(i, j, 0)
                );
                let mut tile = floor_tile;
                if i == 0 || j == 0 {
                    tile = wall_tile;
                    self._component_manager.set(
                        entity,
                        ColliderComponent::new(1)
                    );
                }
                self._component_manager.set(
                    entity,
                    TileComponent::new(tile)
                );
                self._space.add_entity_at(entity, Point::new(i, j));
            }
        }
    }

    pub fn update (
        &mut self,
        glyphbatch: &mut GlyphBatch,
        window: &mut Window,
        true_delta_time:f32
    ) {
        for passive_system in self.passive_systems.iter_mut() {
            passive_system.deref_mut().update(
                &mut self._blackboard,
                &mut self._component_manager,
                &mut self._space,
                glyphbatch,
                window,
                true_delta_time
            );
        }
        window.clear();
        glyphbatch.flush_tiles();
        window.draw_glyphbatch(&glyphbatch);

        let (entity, dt) = match self._scheduler.pop_entity() {
            Some((entity, dt)) => (entity, dt),
            None               => { return; }
        };

        for continuous_system in self.continuous_systems.iter_mut() {
            continuous_system.deref_mut().update(
                &mut self._blackboard,
                &mut self._component_manager,
                &mut self._space,
                glyphbatch,
                window,
                dt
            );
        }
        let controller = match self._controllers.get_mut(&entity) {
            Some(controller_box) => controller_box.deref_mut(),
            None                 => { return; }
        };      

        let next_dt = controller.update(
            &mut self._blackboard,
            &mut self._component_manager,
            &mut self._space,
            window,
            entity
        );

        self._scheduler.push_entity(entity, next_dt);
    }
}
