use std::ops::DerefMut;

use core_manager::CoreState;
use ecs::{Engine};
use game::{Blackboard};
use game::ui::{MouseInput, MouseInterface};
use graphics::{GlyphBatch, RGBColor, Window};
use menu::{MenuObject, MenuType};

pub struct Canvas {
    _x: u32,
    _y: u32,
    _selected: bool,
    _selection: Option<usize>,
    _children: Vec<Box<MenuObject>>
}

impl Canvas {
    pub fn new(
        x: u32, y: u32,
    ) -> Self {
        Self {
            _x: x,
            _y: y,
            _selected: false,
            _selection: None,
            _children: Vec::new()
        }
    }

    pub fn add(mut self, child: Box<MenuObject>) -> Self {
        self._children.push(child);
        self
    }
}

impl MenuObject for Canvas {
    fn select(&mut self) -> bool {
        match self._selection {
            Some(_) => {
                self._selected = true;
                true
            },
            None => {
                for i in 0..self._children.len() {
                    if self._children[i].select() {
                        self._selection = Some(i);
                        self._selected = true;
                        return true;
                    }
                }
                false
            }
        }
    }

    fn deselect(&mut self) -> bool {
        if !self._selected {
            return false;
        }
        self._selected = false;
        match self._selection {
            Some(index) => self._children[index].deselect(),
            None    => false
        }
    }

    fn update(
        &mut self,
        x: u32,
        y: u32,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        window: &mut Window
    ) -> Option<CoreState> {
        for child in self._children.iter_mut() {
            child.deref_mut().update(
                x + self._x,
                y + self._y,
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            );
        }
        None
    }

}

