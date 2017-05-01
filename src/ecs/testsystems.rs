use ecs::{
    ComponentManager, Space, ReactiveSystem, 
    Event, EventType, EventResult
};
use std::boxed::Box;
use std::result::Result;
use std::ops::Deref;


pub struct TestReactor {
    _input: u32,
    _output: u32,
    _reject: u32
}

impl TestReactor {
    pub fn new(input:u32, output:u32, reject:u32) -> TestReactor {
        TestReactor {
            _input: input,
            _output: output,
            _reject: reject
        }
    }
}

impl ReactiveSystem for TestReactor {
    fn update(
            &mut self,
            component_manager: &mut ComponentManager,
            space: &mut Space,
            event: &Event
        ) -> EventResult {
            let name = match (&event).event_type {
                EventType::Named(ref string) => string
            };

            println!("reacting");

            if name == "test" {

                let input:u32 = match event.data.deref().downcast_ref::<u32>() {
                    Some(val) => *val,
                    None      => {
                        println!("could not deref");
                        return EventResult::empty(true)
                    }
                };
                println!("Checking input against {}", self._input);
                println!("Input = {}", input);

                let results = match input == self._input {
                    true  => Some({
                        println!("reacting with {}", self._output);
                        let mut v = Vec::new();
                        v.push(Event::new(
                            EventType::Named(String::from("test")),
                            self._output,
                            1
                        ));
                        v
                    }),
                    false => None
                };

                EventResult {
                    resulting_events: results,
                    allow_event: input != self._reject
                }
            }
            else {
                println!("no reaction");
                EventResult::empty(true)
            }
        }
}