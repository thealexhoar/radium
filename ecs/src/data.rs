
pub struct Data {
    pub floats:Option<Vec<f32>>,
    pub ints:Option<Vec<i32>>,
    pub strings:Option<Vec<String>>
}

impl Data {
    pub fn empty() -> Data {
        Data {
            floats: None,
            ints: None,
            strings: None
        }
    }
}