use graphics::{Color, Tile};

use std::cmp::min;
use std::str::Chars;

pub struct DrawTarget {
    _width:u32,
    _height:u32,
    _data:Vec<Option<Tile>>
}

impl DrawTarget {
    pub fn new(width:u32, height:u32) -> DrawTarget {
        let size:usize = (width * height) as usize;
        DrawTarget {
            _width:width,
            _height:height,
            _data: (0..size).map(|i| None).collect()
        }
    }

    pub fn width(&self) -> u32 {
        self._width
    }

    pub fn height(&self) -> u32 {
        self._height
    }

    pub fn clear(&mut self) {
        for i in 0..self._data.len() {
            self._data[i] = None;
        }
    }

    pub fn get_tile(&self, x:u32, y:u32) -> Option<Tile> {
        let index = self.get_index(x, y);
        self._data[index]
    }

    //the "set" functions will do a full overwrite
    pub fn set_tile(&mut self, tile:Option<Tile>, x:u32, y:u32) {
        let index = self.get_index(x, y);
        self._data[index] = tile;
    }

    pub fn set_tiles_rect(
        &mut self,
        tile:Option<Tile>,
        x:u32, y:u32,
        width:u32, height:u32
     ) {
        let x_bound = min(x+width, self._width);
        let y_bound = min(y+height, self._height);
        for i in x..x_bound {
            for j in y..y_bound {
                self.set_tile(tile, i, j);
            }
        }
    }

    //the "overlay" functions will only overwrite if a source value exists
    pub fn overlay_tile(&mut self, tile:Option<Tile>, x:u32, y:u32) {
        let index = self.get_index(x, y);
        match tile {
            Some(other) => self._data[index] = match self._data[index] {
                Some(mut tile) => Some(tile.overlay(other)),
                None           => Some(other)
            },
            None        => return
        }
    }

    pub fn overlay_tiles_rect(
        &mut self,
        tile:Option<Tile>,
        x:u32, y:u32,
        width:u32, height:u32

    ) {
        let x_bound = min(x+width, self._width);
        let y_bound = min(y+height, self._height);
        for i in x..x_bound {
            for j in y..y_bound {
                self.overlay_tile(tile, i, j);
            }
        }
    }

    //the "overlay soft" functions will only overwrite
    //if a source value exists AND no destination value exists
    pub fn overlay_tile_soft(&mut self, tile:Option<Tile>, x:u32, y:u32) {
        let index = self.get_index(x, y);
        match tile {
            Some(other) => self._data[index] =  match self._data[index] {
                Some(mut tile) => Some(tile.overlay_soft(other)),
                None           => Some(other)
            },
            None        => return
        }
    }

    pub fn overlay_tiles_rect_soft(
        &mut self,
        tile:Option<Tile>,
        x:u32, y:u32,
        width:u32, height:u32
    ) {
        let x_bound = min(x+width, self._width);
        let y_bound = min(y+height, self._height);
        for i in x..x_bound {
            for j in y..y_bound {
                self.overlay_tile_soft(tile, i, j);
            }
        }
    }

    pub fn set_from_drawtarget(
        &mut self,
        drawtarget:&DrawTarget,
        x:u32, y:u32
    ) {
        let width = drawtarget.width();
        let height = drawtarget.height();
        self.set_from_drawtarget_subrect(drawtarget, x, y, 0, 0, width, height);
    }

    pub fn set_from_drawtarget_subrect(
        &mut self,
        drawtarget:&DrawTarget,
        x:u32, y:u32,
        sub_x:u32, sub_y:u32,
        width:u32, height:u32
    ) {
        let x_bound = min(width, self._width - x);
        let y_bound = min(height, self._height - y);
        for i in 0..x_bound {
            for j in 0..y_bound {
                self.set_tile(
                    drawtarget.get_tile(sub_x + i, sub_y + j),
                    x + i, y + j
                );
            }
        }
    }

    pub fn overlay_from_drawtarget(
        &mut self,
        drawtarget:&DrawTarget,
        x:u32, y:u32
    ) {
        let width = drawtarget.width();
        let height = drawtarget.height();
        self.overlay_from_drawtarget_subrect(
            drawtarget,
            x, y,
            0, 0,
            width, height
        );
    }

    pub fn overlay_from_drawtarget_subrect(
        &mut self,
        drawtarget:&DrawTarget,
        x:u32, y:u32,
        sub_x:u32, sub_y:u32,
        width:u32, height:u32
    ) {
        let x_bound = min(width, self._width - x);
        let y_bound = min(height, self._height - y);
        for i in 0..x_bound {
            for j in 0..y_bound {
                self.set_tile(
                    drawtarget.get_tile(sub_x + i, sub_y + j),
                    x + i, y + j
                );
            }
        }
    }

    pub fn overlay_soft_from_drawtarget(
        &mut self,
        drawtarget:&DrawTarget,
        x:u32, y:u32
    ) {
        let width = drawtarget.width();
        let height = drawtarget.height();
        self.overlay_soft_from_drawtarget_subrect(
            drawtarget,
            x, y,
            0, 0,
            width, height
        );
    }

    pub fn overlay_soft_from_drawtarget_subrect(
        &mut self,
        drawtarget:&DrawTarget,
        x:u32, y:u32,
        sub_x:u32, sub_y:u32,
        width:u32, height:u32
    ) {
        let x_bound = min(width, self._width - x);
        let y_bound = min(height, self._height - y);
        for i in 0..x_bound {
            for j in 0..y_bound {
                self.overlay_tile_soft(
                    drawtarget.get_tile(sub_x + i, sub_y + j),
                    x + i,
                    y + j
                );
            }
        }
    }

    pub fn draw_string(
        &mut self,
        text:&String,
        x:u32, y:u32,
        fg_color:Color,
        bg_color:Option<Color>
    ) {
        self.draw_string_slice(text.as_str(), x, y, fg_color, bg_color);
    }

    pub fn draw_string_slice(
        &mut self,
        text:&str,
        x:u32, y:u32,
        fg_color:Color,
        bg_color:Option<Color>
    ) {
        let mut chars = text.chars();
        self.draw_chars(chars, x, y, fg_color, bg_color);
    }

    fn draw_chars(
        &mut self,
        mut chars:Chars,
        x:u32, y:u32,
        fg_color:Color,
        bg_color:Option<Color>
    ) {
        if x >= self._width || y >= self._height {
            return;
        }
        match chars.next() {
            Some(char_val) => {
                let mut val = char_val as u32;
                let tile = Tile::new(Some(fg_color), bg_color, val);
                self.set_tile(Some(tile), x, y);
                self.draw_chars(chars, x+1, y, fg_color, bg_color);
            },
            None           => return
        }
    }

    fn get_index(&self, x:u32, y:u32) -> usize {
        (x + y * self._width) as usize
    }
}