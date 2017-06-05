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
    pub component_manager: ComponentManager,
    pub space: Space
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            passive_systems: Vec::new(),
            continuous_systems: Vec::new(),
            _blackboard: Blackboard::new(),
            component_manager: ComponentManager::new(),
            space: Space::new()
        }
    }

    pub fn load(&mut self) {
        for i in -10..10 {
            for j in -10..10 {
                self.space.load_chunk(i,j);
            }
        }

        let player = self.component_manager.create_entity();
        self.component_manager.set(player, PositionComponent::new(1, 1, 1));
        self.component_manager.set(player, TileComponent::new(
            Tile::new(
                Some(Color::new_from_rgb_f(0.6, 0.8, 1.0)),
                None,
                '@' as u32
            )
        ));
        self.component_manager.set(player, ColliderComponent::new(1));

        self.space.add_entity_at(player, Point::new(1, 1));

        //self._controllers.insert(player, Box::new(PlayerController::new()));
        //self._scheduler.push_entity(player, 0);
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
                let entity = self.component_manager.create_entity();
                self.component_manager.set(
                    entity,
                    PositionComponent::new(i, j, 0)
                );
                let mut tile = floor_tile;
                if i == 0 || j == 0 {
                    tile = wall_tile;
                    self.component_manager.set(
                        entity,
                        ColliderComponent::new(1)
                    );
                }
                self.component_manager.set(
                    entity,
                    TileComponent::new(tile)
                );
                self.space.add_entity_at(entity, Point::new(i, j));
            }
        }
    }

    pub fn update_passive_systems(
        &mut self,
        glyphbatch: &mut GlyphBatch,
        window: &mut Window,
        true_delta_time:f32
    ) {
        for passive_system in self.passive_systems.iter_mut() {
            passive_system.deref_mut().update(
                &mut self._blackboard,
                &mut self.component_manager,
                &mut self.space,
                glyphbatch,
                window,
                true_delta_time
            );
        }
    }

    pub fn update_continuous_systems(
        &mut self,
        delta_time:u32
    ) {
        for continuous_system in self.continuous_systems.iter_mut() {
            continuous_system.deref_mut().update(
                &mut self._blackboard,
                &mut self.component_manager,
                &mut self.space,
                delta_time
            );
        }
    }
}
