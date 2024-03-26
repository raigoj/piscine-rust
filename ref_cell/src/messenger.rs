use std::cell::Cell;
use std::rc::Rc;
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}
#[derive(Debug)]
pub struct Tracker<'l, L: Logger> {
    logger: &'l L,
    value: Cell<usize>,
    max: usize,
}
impl<'l, L: Logger> Tracker<'l, L> {
    pub fn new(logger: &'l L, max: usize) -> Self {
        Self {
            logger,
            value: Cell::new(0),
            max,
        }
    }
    pub fn set_value<T>(&self, rc: &Rc<T>) {
        self.value.set(Rc::strong_count(rc));
        let percent = self.value.get() * 100 / self.max;
        if percent >= 100 {
            self.logger.error("Error: you are over your quota!")
            // self.logger.error("you are over your quota!")
        } else {
            self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", percent))
            // self.logger.warning(&format!("you have used up over {}% of your quota! Proceeds with precaution", percent))
        }
    }
    pub fn peek<T>(&self, rc: &Rc<T>) {
        let percent = Rc::strong_count(rc) * 100 / self.max;
        self.logger.info(&format!("Info: you are using up to {}% of your quota", percent))
        // self.logger.info(&format!("you are using up too {}% of your quote", percent))
    }
}
