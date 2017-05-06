use ecs::{
    PassiveSystem, ReactiveSystem, ContinuousSystem,
    Component, ComponentManager,
    Entity,
    EventType, Event,
    Scheduler,
    Space
};
use util::PriorityQueue;
use std::ops::Deref;
use graphics::{Window};
use graphics::Event as WindowEvent;


pub struct Engine {
    _passive_systems: PriorityQueue<Box<PassiveSystem>>,
    _reactive_systems: PriorityQueue<Box<ReactiveSystem>>,
    _continuous_systems: PriorityQueue<Box<ContinuousSystem>>,
    _component_manager: ComponentManager,
    _scheduler: Scheduler,
    _space: Space
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            _passive_systems: PriorityQueue::new(),
            _reactive_systems: PriorityQueue::new(),
            _continuous_systems: PriorityQueue::new(),
            _component_manager: ComponentManager::new(),
            _scheduler: Scheduler::new(),
            _space: Space::new()
        }
    }

    pub fn load(&mut self) {
        //initializes startup
        self._scheduler.push_event(
            Event::new(
                EventType::Initialize,
                None,
                Box::new(0),
                0
            )
        );
    }

    pub fn add_passive_system<T: 'static + PassiveSystem + Sized>(
        &mut self,
        system:T,
        priority:i32
    ) {
        let system_box = Box::new(system);

        let x = 0;
        self._passive_systems.insert(system_box, priority);
    }

    pub fn add_reactive_system<T: 'static + ReactiveSystem + Sized> (
        &mut self,
        system:T,
        priority:i32
    ) {
        let system_box = Box::new(system);
        self._reactive_systems.insert(system_box, priority);
    }

    pub fn add_continuous_system<T: 'static + ContinuousSystem + Sized>(
        &mut self,
        system:T,
        priority:i32
    ) {
        let system_box = Box::new(system);
        self._continuous_systems.insert(system_box, priority);
    }

    pub fn update (
        &mut self, 
        window: &mut Window,
        true_delta_time:f32
    ) {
        //update passive systems
        //TODO: move passive system updates to an asynchronous updating system?
        for i in 0..self._passive_systems.len() {
            let out = self._passive_systems[i].update(
                & self._component_manager,
                & self._space,
                window,
                true_delta_time
            );
            match out {
                Some(event_vec) => {
                    for event in event_vec {
                        self._scheduler.push_event(event);
                    }
                }
                None      => {}
            };
        }


        let mut window_event:Option<WindowEvent> = None;
        'event_check: for event in window.events() {
            match event {
                WindowEvent::Closed   => {
                    window.close();
                    break 'event_check;
                },
                WindowEvent::KeyPress { code, alt, ctrl, shift } => {
                    window_event = Some(event);
                    break 'event_check;
                },
                _                     => {}
            }
        }

        match window_event {
            Some(window_event) => self._scheduler.push_event(
                Event::new(
                    EventType::WindowEvent,
                    None,
                    window_event,
                    0
                )
            ),
            None => {
                return; 
            }
        }; ;

        //handle events off the scheduler
        //first event SHOULD have non-zero delta time
        //the following events with matching delta times will have dt = 0
        //so events that are "concurrent" will all occur in one update
        'outer: loop {
            let next_event = match self._scheduler.pop_event() {
                Some(event) => event,
                None        => break 'outer
            };
            let mut event_accepted = true;
            
            //run event against the reactive systems
            'inner: for i in 0..self._reactive_systems.len() {
                let results = self._reactive_systems[i].update(
                    &mut self._component_manager,
                    &mut self._space,   
                    &next_event
                );
                match results.resulting_events {
                    Some(result_events) => for response_event in result_events {
                        self._scheduler.push_event(response_event);
                    },
                    None               => {}
                }
                if !results.allow_event {
                    event_accepted = false;
                    break 'inner;
                }
            }

            if event_accepted {
                let dt = next_event.delta_time;
                self._scheduler.elapse_time(dt);

                //if any time passes
                //update game-time based systems 
                if dt > 0 {
                    for i in 0..self._continuous_systems.len() {
                        self._continuous_systems[i].update(
                            &mut self._component_manager,
                            &mut self._space,
                            next_event.delta_time
                        );
                    }
                }
            }
            //if any time WILL pass with next event end the event looping
            //return to beginning of game loop to update rendering and input
            match self._scheduler.top_event_time(){
                None       => {break;}
                Some(time) => {
                    if time != 0 {break;}
                }
            }
        }
    }
}
