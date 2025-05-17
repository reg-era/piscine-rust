#[derive(Debug,PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u32,
}

impl Boss {
    pub fn new(field: &str, age: u32) -> Self {
        Self {
            name: String::from(field),
            age,
        }
    }
}