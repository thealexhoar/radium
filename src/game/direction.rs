#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    North,
    South,
    East,
    West
}

impl Direction {
    pub fn to_delta(self) -> (i32, i32, i32) {
        match self {
            Direction::Up    => (0, 0, 1),
            Direction::Down  => (0, 0, -1),
            Direction::North => (0, -1, 0),
            Direction::South => (0, 1, 0),
            Direction::East  => (1, 0, 0),
            Direction::West  => (-1, 0, 0)
        }
    }
}