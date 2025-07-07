#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: match self.head.take() {
                Some(ne) => Some(Box::new(ne)),
                None => None,
            },
        };

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        let new_node = match self.head.take() {
            Some(nex) => match nex.next {
                Some(nex_nex) => Some(Node {
                    value: nex_nex.value,
                    next: nex_nex.next,
                }),
                None => None,
            },
            None => None,
        };

        self.head = new_node;
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut posi = self.head.as_ref().map(|nd| &*nd);
        while let Some(next_node) = posi {
            count += 1;
            posi = next_node.next.as_ref().map(|nd| &**nd);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut new_list_str = List::new();
        new_list_str.push("String Test 1");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 2");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 3");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.pop();
        println!("The size of the list is {}", new_list_str.len());
    }
}
