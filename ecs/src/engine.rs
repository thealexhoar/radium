use {PassiveSystem, ReactiveSystem, ContinuousSystem};
use {Component, ComponentType};
use std::collections::{HashMap, BinaryHeap};

pub struct ComponentManager {
    _data: HashMap<ComponentType,Vec<Component>>
}

pub struct Scheduler {
}

pub struct Engine {
    _passive_systems:Vec<Box<PassiveSystem>>,
    _reactive_systems:Vec<Box<ReactiveSystem>>,
    _continuous_systems:Vec<Box<ContinuousSystem>>,
}
