use std::cell::{Cell, RefCell};
pub use std::rc::Rc;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Blog {
    pub fn new() -> Blog {
        Blog {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_article(&self, body: String) -> (usize, Article) {
        let id = self.new_id();
        let new_art = Article::new(id, body, self);

        self.states.borrow_mut().push(false);

        (id, new_art)
    }

    pub fn new_id(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        if self.states.borrow()[id] == true {
            panic!("{} is already dropped", id);
        }
        self.drops.set(self.drops.get() + 1);
        self.states.borrow_mut()[id] = true;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    pub id: usize,
    pub body: String,
    pub parent: &'a Blog,
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, blog: &'a Blog) -> Self {
        Self {
            id,
            body,
            parent: blog,
        }
    }
    pub fn discard(self) {
        self.parent.add_drop(self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dropped_and_drops() {
        let blog = Blog::new();
        let (pid, article) = blog.new_article(String::from("gnome-shell"));
        let (pid0, article0) = blog.new_article(String::from("i3"));
        let (pid1, article1) = blog.new_article(String::from("shell"));
        let (pid2, article2) = blog.new_article(String::from("spotify"));

        article.discard();
        assert_eq!(blog.drops.get(), 1_usize);
        article0.discard();
        assert_eq!(blog.drops.get(), 2_usize);

        assert!(blog.is_dropped(pid), "{} should have been dropped", pid);
        assert!(blog.is_dropped(pid0), "{} should have been dropped", pid0);
        assert!(
            !blog.is_dropped(pid1),
            "{} should not have been dropped",
            pid1
        );
        assert!(
            !blog.is_dropped(pid2),
            "{} should not have been dropped",
            pid2
        );

        article1.discard();
        article2.discard();
        assert_eq!(blog.drops.get(), 4_usize);
    }

    #[test]
    fn test_using_rc() {
        // will create a new reference to the article
        // this will test the following
        // if we drop the cloned value the RC will decrease
        // but the article will not be dropped!
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("Xorg"));
        let article = Rc::new(article);
        let article_clone = article.clone();

        assert_eq!(Rc::strong_count(&article), 2);

        drop(article_clone);

        assert_eq!(Rc::strong_count(&article), 1);
    }

    #[test]
    #[should_panic(expected = "0 is already dropped")]
    fn test_drop_same_article() {
        // test if we drop the same article after it was already been dropped
        let blog = Blog::new();
        let (_pid, article) = blog.new_article(String::from("gsd-rfkill"));
        let article_clone = article.clone();
        article.discard();
        article_clone.discard();
    }
}
