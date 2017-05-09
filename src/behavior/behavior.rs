use ecs::{ComponentManager, Space, Entity, Event, EventBuilder};
use behavior::Result;
use std::ops::DerefMut;

pub trait Behavior {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &ComponentManager,
        space: &Space
    ) -> (Result, Option<Vec<Event>>);
}

//returns a single result, always
pub struct ResultBehavior {
    result: Result
}

impl ResultBehavior {
    pub fn new(result:Result) -> Self { Self { result } }
}

impl Behavior for ResultBehavior {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &ComponentManager,
        space: &Space
    ) -> (Result, Option<Vec<Event>>) {
        (self.result, None)
    }
}

//returns Success and a single event, always
pub struct EventBehavior {
    event_builder: Box<EventBuilder>
}

impl EventBehavior {
    pub fn new(event_builder: Box<EventBuilder>) -> Self { 
        Self { event_builder} 
    }
}

impl Behavior for EventBehavior {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &ComponentManager,
        space: &Space
    ) -> (Result, Option<Vec<Event>>) {
        let mut out = Vec::new();
        out.push(self.event_builder.build_event());
        (Result::Success, Some(out))
    }
}

//execute in order until one fails
//will loop forever unless terminated
pub struct SequenceBehavior {
    children: Vec<Box<Behavior>>,
    current_node: usize
}

impl SequenceBehavior {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            current_node: 0
        }
    }

    pub fn push_child(&mut self, child:Box<Behavior>) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index:usize, child:Box<Behavior>) {
        self.children.insert(index, child);
    }

    pub fn add_terminator(&mut self) {
        self.children.push(Box::new(ResultBehavior::new(Result::Failure)));
    }
}

impl Behavior for SequenceBehavior {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &ComponentManager,
        space: &Space
    ) -> (Result, Option<Vec<Event>>) {
        let mut out_events = Vec::new();
        loop {
            let (result, events_option) = self.children[self.current_node]
                .deref_mut()
                .execute(entity, component_manager, space);

            match events_option {
                Some(events) => {
                    for event in events {
                        out_events.push(event);
                    }
                },
                None         => {}
            }
            let break_at_end = false;
            match result {
                Result::Failure => {
                    self.current_node = 0;
                    return (Result::Success, Some(out_events));
                },
                Result::Running => {
                    return (Result::Running, Some(out_events));
                },
                Result::Success => {},
            };
            

            self.current_node += 1;
            self.current_node %= self.children.len();
        }
    }
}


//execute in order until one succeeds
//will loop forever unless terminated
pub struct SelectorBehavior {
    children: Vec<Box<Behavior>>,
    current_node: usize
}

impl SelectorBehavior {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            current_node: 0
        }
    }

    pub fn push_child(&mut self, child:Box<Behavior>) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index:usize, child:Box<Behavior>) {
        self.children.insert(index, child);
    }

    pub fn add_terminator(&mut self) {
        self.children.push(Box::new(ResultBehavior::new(Result::Success)));
    }
}

impl Behavior for SelectorBehavior {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &ComponentManager,
        space: &Space
    ) -> (Result, Option<Vec<Event>>) {
        let mut out_events = Vec::new();
        loop {
            let (result, events_option) = self.children[self.current_node]
                .deref_mut()
                .execute(entity, component_manager, space);

            match events_option {
                Some(events) => {
                    for event in events {
                        out_events.push(event);
                    }
                },
                None         => {}
            }
            let break_at_end = false;
            match result {
                Result::Success => {
                    self.current_node = 0;
                    return (Result::Success, Some(out_events));
                },
                Result::Running => {
                    return (Result::Running, Some(out_events));
                },
                Result::Failure => {},
            };
            

            self.current_node += 1;
            self.current_node %= self.children.len();
        }
    }
}
