use Entity;
use {Event, EventType};


struct ScheduledEvent {
    pub event_type: EventType,
    pub event: Box<Event>,
    pub delta_time: u32
}


pub struct Scheduler {
    _event_queue: Vec<ScheduledEvent>,
    _turn_taker_queue: Vec<Entity>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            _event_queue: Vec::new(),
            _turn_taker_queue: Vec::new()
        }
    }

    pub fn schedule_event<T: Event + Sized>(
        &mut self, 
        event_type: EventType, 
        event: T,
        delta_time: u32
    ) {
        let scheduled_event = ScheduledEvent{
            event_type: event_type,
            event: Box::new(event),
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
            Some(i) => self._event_queue.insert(i, scheduled_event),
            None    => self._event_queue.push(scheduled_event)
        };
    }

    pub fn pop(&mut self) -> Option<(EventType, Box<Event>)> {
        match self._event_queue.len() {
            0 => None,
            _ => {
                let scheduled_event = self._event_queue.remove(0);
                self.subtract_time(scheduled_event.delta_time);
                Some((scheduled_event.event_type, scheduled_event.event))
            }
        }
    }

    fn subtract_time(&mut self, time:u32) {
        for scheduled_event in &mut self._event_queue {
            scheduled_event.delta_time -= time;
        }
    }
}