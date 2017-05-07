use ecs::{Component, Entity};
use std::any::TypeId;

pub struct Family {
    pub all_components: Vec<TypeId>,
    pub one_components: Vec<TypeId>,
    pub none_components: Vec<TypeId>
}

impl Family {
    pub fn new() -> Self {
        Self {
            all_components: Vec::new(),
            one_components: Vec::new(),
            none_components: Vec::new()
        }
    }

    pub fn all(mut self, components:Vec<TypeId>) -> Self {
        self.all_components = components;
        self
    }

    pub fn one(mut self, components:Vec<TypeId>) -> Self {
        self.one_components = components;
        self
    }

    pub fn none(mut self, components:Vec<TypeId>) -> Self {
        self.none_components = components;
        self
    }
}