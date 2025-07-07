#[derive(Debug, Clone)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, age: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person::new("Leo");

        println!("Person = {:?}", person);
    }
}
