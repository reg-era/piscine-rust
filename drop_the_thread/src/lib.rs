use std::cell::{Cell, RefCell};
pub use std::rc::Rc;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.track_worker();
        self.states.borrow_mut().push(false);

        (id, Thread::new_thread(id, c, &self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow_mut()[id]
    }

    pub fn add_drop(&self, id: usize) {
        if self.states.borrow()[id] {
            panic!("{} is already dropped", id);
        } else {
            self.states.borrow_mut()[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Self {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
}
