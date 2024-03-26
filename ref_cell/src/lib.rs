
pub mod messenger;
use std::collections::HashMap;
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use messenger::*;
#[derive(Debug)]
pub struct Worker<T> {
    pub track_value: Rc<T>,
    pub mapped_messages: RefCell<HashMap<&'static str, String>>,
    pub all_messages: RefCell<Vec<String>>,
}
impl<T> Worker<T> {
    pub fn new(value: T) -> Self {
        Self {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(Default::default()),
            all_messages: RefCell::new(vec![])
        }
    }
}
impl<T> Logger for Worker<T> {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning", msg[9..].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info", msg[6..].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error", msg[7..].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}
