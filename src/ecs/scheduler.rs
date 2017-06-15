use ecs::*;
use std::any::Any;

struct ScheduleItem {
    entity: Entity,
    delta_time: u32
}

pub struct Scheduler {
    _queue: Vec<ScheduleItem>,
    _elapsed_time: u64
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            _queue: Vec::new(),
            _elapsed_time: 0
        }
    }

    pub fn push_entity(
        &mut self,
        entity: Entity,
        delta_time: u32
    ) {
        let mut index:Option<usize> = None;
        for i in 0..self._queue.len() {
            if self._queue[i].delta_time > delta_time {
                index = Some(i);
                break;
            }
        }

        let item = ScheduleItem { entity, delta_time };

        match index {
            Some(i) => self._queue.insert(i, item),
            None    => self._queue.push(item)
        };
    }

    pub fn pop_entity(&mut self) -> Option<(Entity, u32)> {
        match self._queue.len() {
            0 => None,
            _ => {
                let dt = self._queue[0].delta_time;
                Some((self._queue.remove(0).entity, dt))
            }
        }
    }
    pub fn elapse_time(&mut self, time:u32) {
        for scheduled_item in &mut self._queue {
            scheduled_item.delta_time -= time;
        }
        self._elapsed_time += time as u64;
    }
}