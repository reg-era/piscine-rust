pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: Target,
    next: Link,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}
impl Field {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, target: Target) {
        let new_head = Some(Box::new(Node {
            elem: target,
            next: self.head.take(),
        }));
        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<Target> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&Target> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut field = Field::new();
        field.push(Target { size: 10, xp: 2 });
        field.push(Target { size: 5, xp: 1 });
        field.push(Target { size: 20, xp: 4 });

        assert_eq!(field.pop().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 5, xp: 1 });
        assert_eq!(field.pop().unwrap(), Target { size: 10, xp: 2 });
        assert_eq!(field.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut field = Field::new();
        field.push(Target { size: 10, xp: 2 });
        field.push(Target { size: 5, xp: 1 });
        field.push(Target { size: 20, xp: 4 });

        assert_eq!(*field.peek().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 5, xp: 1 });
        assert_eq!(*field.peek().unwrap(), Target { size: 10, xp: 2 });
        assert_eq!(*field.peek().unwrap(), Target { size: 10, xp: 2 });
    }

    #[test]
    fn test_peek_mut() {
        let mut field = Field::new();
        field.push(Target { size: 10, xp: 2 });
        field.push(Target { size: 5, xp: 1 });
        field.push(Target { size: 20, xp: 4 });

        assert_eq!(*field.peek().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 5, xp: 1 });
        assert_eq!(*field.peek_mut().unwrap(), Target { size: 10, xp: 2 });
        let last_target = field.peek_mut();
        last_target.map(|target| *target = Target { size: 50, xp: 44 });
        assert_eq!(*field.peek().unwrap(), Target { size: 50, xp: 44 });
    }
}