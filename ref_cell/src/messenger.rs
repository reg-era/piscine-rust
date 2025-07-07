use std::{cell::RefCell, rc::Rc};

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Self {
            logger,
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, refer: &Rc<usize>) {
        *self.value.borrow_mut() = Rc::strong_count(refer);

        let percentage = ((*self.value.borrow() as f64 / self.max as f64) * 100.).floor() as u8;
        if percentage >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, refer: &Rc<usize>) {
        let percentage = ((Rc::strong_count(refer) as f64 / self.max as f64) * 100.).floor() as u8;

        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage
        ));
    }
}