use ecs::*;
use graphics::{GlyphBatch, Window};
use game::Blackboard;
use std::ops::{DerefMut};

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

    pub fn initialize(&mut self, blackboard: &mut Blackboard) {
        for i in -10..10 {
            for j in -10..10 {
                self.space.load_chunk(i,j);
            }
        }
    }

    //TODO 10/23/17: add a "level definition" object, and load here
    pub fn load(&mut self) {

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
