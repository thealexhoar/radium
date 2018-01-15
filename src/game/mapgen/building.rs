use game::mapgen::TerrainType;
use graphics::Tile;


pub struct Building {
    pub width: u32,
    pub height: u32,
    _data: Vec<Vec<TerrainType>>
}

impl Building {
    pub fn new(width: u32, height: u32, floors: usize) -> Self {
        let mut out = Self {
            width,
            height,
            _data: Vec::new()
        };

        out._data.resize(floors, Vec::new());

        for i in 0..floors {
            out._data[i].resize((width * height) as usize, TerrainType::None);
        }

        out
    }

    fn floor_index(&self, x: u32, y: u32) -> usize {
        return (y * self.width + x) as usize
    }
}