use ecs::{Entity, Event, EventType, EventResult};
use std::any::Any;


pub struct Scheduler {
    _event_queue: Vec<Event>,
    _elapsed_time: u64
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            _event_queue: Vec::new(),
            _elapsed_time: 0
        }
    }

    pub fn push_event(
        &mut self, 
        event: Event
    ) {
        let mut index:Option<usize> = None;
        for i in 0..self._event_queue.len() {
            if self._event_queue[i].delta_time > event.delta_time {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => self._event_queue.insert(i, event),
            None    => self._event_queue.push(event)
        };
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        match self._event_queue.len() {
            0 => None,
            _ => {
                let event = self._event_queue.remove(0);
                self.subtract_time(event.delta_time);
                self._elapsed_time += event.delta_time as u64;
                Some(event)
            }
        }
    }

    pub fn top_event_time(&self) -> Option<u32> {
        match self._event_queue.len() {
            0 => None,
            _ => Some(self._event_queue[self._event_queue.len() - 1]
                .delta_time)
        }
    }

    fn subtract_time(&mut self, time:u32) {
        for scheduled_event in &mut self._event_queue {
            scheduled_event.delta_time -= time;
        }
    }
}