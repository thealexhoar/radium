use Data;

pub enum ComponentType {
    //default components
    Position,
    Tile,
    Named(String)
}

pub struct Component {
    pub component_type: ComponentType,
    pub data: Data
}