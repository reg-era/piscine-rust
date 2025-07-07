#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub discount: i32,
    pub next: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Self { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new_node = Box::new(Person {
            name,
            discount,
            next: self.node.take(),
        });
        self.node = Some(new_node);
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut pos = &self.node;
        while let Some(person) = pos {
            if person.name == name {
                return Some((person.name.to_string(), person.discount));
            }
            pos = &person.next;
        }
        None
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current = self.node.as_mut().unwrap();

        if current.next.is_none() {
            let last = self.node.take().unwrap();
            return Some((last.name, last.discount));
        }

        while current.next.as_ref()?.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        let last = current.next.take().unwrap();
        Some((last.name, last.discount))
    }

    pub fn invert_queue(&mut self) {
        let mut prev = None;
        let mut current = self.node.take();

        while let Some(mut switch) = current {
            let next = switch.next.take();
            switch.next = prev;
            prev = Some(switch);
            current = next;
        }

        self.node = prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_works() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);
        println!("{:?}", list);

        println!("{:?}", list.search("Marie"));
        println!("{:?}", list.search("Alice"));
        println!("{:?}", list.search("someone"));

        println!("removed {:?}", list.rm());
        println!("list {:?}", list);
        list.invert_queue();
        println!("invert {:?}", list);
    }

    #[test]
    fn test_new() {
        let list = Queue::new();
        assert!(list.node.is_none());
    }

    #[test]
    fn test_one_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 14);
        list.rm();
        assert!(list.node.is_none());
    }

    #[test]
    fn test_two_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 13);
        list.add(String::from("Monica"), 54);
        list.rm();

        assert_eq!(list.node.as_ref().unwrap().name, "Monica");
        assert_eq!(list.node.as_ref().unwrap().discount, 54);
    }

    #[test]
    fn test_more_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);
        list.rm();

        assert_eq!(list.node.as_ref().unwrap().name, "Alice");
        assert_eq!(list.node.as_ref().unwrap().discount, 35);

        list.rm();
        list.rm();
        assert_eq!(list.node.as_ref().unwrap().name, "Alice");
        assert_eq!(list.node.as_ref().unwrap().discount, 35);
    }

    #[test]
    fn test_search() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);

        assert_eq!(list.search("Ana"), Some((String::from("Ana"), 5)));

        assert_eq!(list.search("Monica"), Some((String::from("Monica"), 15)));

        assert_eq!(list.search("Alice"), Some((String::from("Alice"), 35)));

        assert_eq!(list.search("someone_that_does_not_exist"), None);
    }

    #[test]
    fn test_invert() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);

        list.invert_queue();
        assert_eq!(list.node.as_ref().unwrap().name, "Marie");
        assert_eq!(list.node.as_ref().unwrap().discount, 20);

        list.rm();
        list.invert_queue();
        assert_eq!(list.node.as_ref().unwrap().name, "Ana");
        assert_eq!(list.node.as_ref().unwrap().discount, 5);
    }
}

// let mut current = self.node.as_mut()?;
// if current.next.is_none() {
// let only_node = self.node.take().unwrap();
// return Some((only_node.name, only_node.discount));
// }
// while current.next.as_ref()?.next.is_some() {
// current = current.next.as_mut().unwrap();
// }
// let last_node = current.next.take().unwrap();
// Some((last_node.name, last_node.discount))
