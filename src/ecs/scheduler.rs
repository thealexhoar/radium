use ecs:: {Entity, Event, EventType};
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

    pub fn push_event<T: Any + Sized>(
        &mut self, 
        event_type: EventType, 
        data: T,
        delta_time: u32
    ) {
        let new_event = Event{
            event_type: event_type,
            data: Box::new(data),
            delta_time: delta_time
        };
        let mut index:Option<usize> = None;
        for i in 0..self._event_queue.len() {
            if self._event_queue[i].delta_time > delta_time {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => self._event_queue.insert(i, new_event),
            None    => self._event_queue.push(new_event)
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

    fn subtract_time(&mut self, time:u32) {
        for scheduled_event in &mut self._event_queue {
            scheduled_event.delta_time -= time;
        }
    }
}