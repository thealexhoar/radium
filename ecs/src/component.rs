pub struct Component {
    pub floats:Option<Vec<f32>>,
    pub ints:Option<Vec<i32>>,
    pub strings:Option<Vec<String>>
}

pub enum ComponentType {
    //default components
    Position,
    Tile,
    Named(String)
}