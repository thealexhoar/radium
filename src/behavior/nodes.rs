use ecs::{ComponentManager, Space, Entity, Event, EventBuilder};
use behavior::{BehaviorTreeNode, Result};

//returns a single result, always
pub struct ResultNode {
    result: Result
}

impl ResultNode {
    pub fn new(result:Result) -> Self { Self { result } }
}

impl BehaviorTreeNode for ResultNode {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> (Result, Option<Vec<Event>>) {
        (self.result, None)
    }
}

//returns Success and a single event, always
pub struct EventNode {
    event_builder: Box<EventBuilder>
}

impl EventNode {
    pub fn new(event_builder: Box<EventBuilder>) -> Self { 
        Self { event_builder} 
    }
}

impl BehaviorTreeNode for EventNode {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> (Result, Option<Vec<Event>>) {
        let mut out = Vec::new();
        out.push(self.event_builder.build_event());
        (Result::Success, Some(out))
    }
}

//execute in order until one fails
//will loop forever unless terminated
pub struct SequenceNode {
    children: Vec<Box<BehaviorTreeNode>>,
    current_node: usize
}

impl SequenceNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            current_node: 0
        }
    }

    pub fn push_child(&mut self, child:Box<BehaviorTreeNode>) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index:usize, child:Box<BehaviorTreeNode>) {
        self.children.insert(index, child);
    }

    pub fn add_terminator(&mut self) {
        self.children.push(Box::new(ResultNode::new(Result::Failure)));
    }
}

impl BehaviorTreeNode for SequenceNode {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> (Result, Option<Vec<Event>>) {
        let mut out_events = Vec::new();
        loop {
            let (result, events_option) = self.children[self.current_node]
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
pub struct SelectorNode {
    children: Vec<Box<BehaviorTreeNode>>,
    current_node: usize
}

impl SelectorNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            current_node: 0
        }
    }

    pub fn push_child(&mut self, child:Box<BehaviorTreeNode>) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index:usize, child:Box<BehaviorTreeNode>) {
        self.children.insert(index, child);
    }

    pub fn add_terminator(&mut self) {
        self.children.push(Box::new(ResultNode::new(Result::Success)));
    }
}

impl BehaviorTreeNode for SelectorNode {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> (Result, Option<Vec<Event>>) {
        let mut out_events = Vec::new();
        loop {
            let (result, events_option) = self.children[self.current_node]
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