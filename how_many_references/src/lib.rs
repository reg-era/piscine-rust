pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut to_deref: Vec<Rc<String>> = Vec::new(); // 🐱
        for ele in self.ref_list.iter() {
            if !Rc::ptr_eq(ele, &element) {
                to_deref.push(Rc::clone(ele));
            }
        }
        self.ref_list = to_deref;
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
