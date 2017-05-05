use std::slice::Iter;
use std::ops::{Index, IndexMut};

struct Entry<T: Sized> {
    priority: i32,
    value:T
}

pub struct PriorityQueue<T: Sized> {
    _data: Vec<Entry<T>>
}

impl<T: Sized> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            _data: Vec::new()
        }
    }

    pub fn with_capacity(capacity:usize) -> PriorityQueue<T> {
        PriorityQueue {
            _data: Vec::with_capacity(capacity)
        }
    }
    //high priority objects go at the end
    pub fn insert(&mut self, value:T, priority: i32) -> &T{
        let entry = Entry {priority, value};
        let mut index:usize = 0;
        loop {
            if self._data.len() == index  {
                break;
            }
            if self._data[index].priority >= priority {
                break;
            }
            index += 1;
        }
        self._data.insert(index, entry);
        &(self._data[index].value)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self._data.pop() {
            None        => None,
            Some(entry) => Some(entry.value)
        }
    }

    pub fn len(&self) -> usize {
        self._data.len()
    }

}

impl<T: Sized> Index<usize> for PriorityQueue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        let target_index = self.len() - index;
        &self._data[target_index].value
    }
}

impl<T: Sized> IndexMut<usize> for PriorityQueue<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        let target_index = self.len() - 1 - index;
        &mut self._data[target_index].value
    }
}
