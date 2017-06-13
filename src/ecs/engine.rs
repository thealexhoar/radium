use ecs::*;
use util::{Point, PriorityQueue};
use graphics::{Color, GlyphBatch, Tile, Window};
use graphics::Event as WindowEvent;
use game::graphics::TileComponent;
use game::components::ColliderComponent;
use game::Blackboard;
use std::ops::{Deref, DerefMut};
use std::collections::{HashMap, HashSet};

pub struct Engine {
    pub passive_systems: Vec<Box<PassiveSystem>>,
    pub continuous_systems: Vec<Box<ContinuousSystem>>,
    pub component_manager: ComponentManager,
    pub space: Space
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            passive_systems: Vec::new(),
            continuous_systems: Vec::new(),
            component_manager: ComponentManager::new(),
            space: Space::new()
        }
    }

    pub fn load (
        &mut self,
        blackboard: &mut Blackboard
    ) {
        for i in -10..10 {
            for j in -10..10 {
                self.space.load_chunk(i,j);
            }
        }

        let player = self.component_manager.create_entity();
        self.component_manager.set(player, PositionComponent::new(1, 1, 0, 1));
        self.component_manager.set(player, TileComponent::new(
            Tile::new(
                Some(Color::blue()),
                None,
                '@' as u32
            )
        ));
        self.component_manager.set(player, ColliderComponent::new(1));

        self.space.add_entity_at(player, Point::new(1, 1, 0));

        let tile_fg = Color::new_from_rgb(30, 30, 30);
        let tile_bg = Color::new_from_rgb(0,0,0);

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
                    PositionComponent::new(i, j, 0, 0)
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
                self.space.add_entity_at(entity, Point::new(i, j, 0));
            }
        }
        for i in 0..20 {
            for j in 0..10 {
                let entity = self.component_manager.create_entity();
                self.component_manager.set(
                    entity,
                    PositionComponent::new(i, j, 2, 0)
                );
                self.component_manager.set(
                    entity,
                    TileComponent::new(wall_tile)
                );
                self.space.add_entity_at(entity, Point::new(i, j, 2));
            }
        }
    }

    pub fn update_passive_systems(
        &mut self,
        glyphbatch: &mut GlyphBatch,
        window: &mut Window,
        blackboard: &mut Blackboard,
        true_delta_time:f32
    ) {
        for passive_system in self.passive_systems.iter_mut() {
            passive_system.deref_mut().update(
                blackboard,
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
        blackboard: &mut Blackboard,
        delta_time:u32
    ) {
        for continuous_system in self.continuous_systems.iter_mut() {
            continuous_system.deref_mut().update(
                blackboard,
                &mut self.component_manager,
                &mut self.space,
                delta_time
            );
        }
    }
}
