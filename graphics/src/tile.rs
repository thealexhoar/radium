use Color;

pub const TILE_ID_MAX_VALUE:u32 = 255;

#[derive(Clone, Copy)]
pub struct Tile {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub tile_id: u32
    
}

impl Tile {
    pub fn new(
        fg_color: Option<Color>, 
        bg_color: Option<Color>, 
        tile_id: u32
    ) -> Tile {
        Tile {
            fg_color: fg_color,
            bg_color: bg_color,
            tile_id: tile_id
        }
    }
    
    //fill everything where the other has value
    pub fn overwrite(&mut self, other:Tile) {
        self.fg_color = match other.fg_color {
            Some(color) => other.fg_color,
            None        => self.fg_color
        };

        self.bg_color = match other.bg_color {
            Some(color) => other.bg_color,
            None        => self.bg_color
        };

        self.tile_id = match other.fg_color {
            Some(color) => other.tile_id,
            None        => self.tile_id
        };
    }

    //fill everything where self has no value
    pub fn overwrite_soft(&mut self, other:Tile) {
        self.fg_color = match self.fg_color {
            Some(color) => self.fg_color,
            None        => other.fg_color
        };

        self.bg_color = match self.bg_color {
            Some(color) => self.bg_color,
            None        => other.bg_color
        };

        self.tile_id = match self.fg_color {
            Some(color) => self.tile_id,
            None        => other.tile_id
        };
    }
}