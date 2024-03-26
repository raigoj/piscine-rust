
pub use std::rc::Rc;
pub struct Node {
    pub value: Vec<Rc<String>>,
}
impl Node {
    pub fn new(value: Vec<Rc<String>>) -> Node {
        Node { value }
    }
    pub fn add_ele(&mut self, v: Rc<String>) {
        self.value.push(v)
    }
    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.value.retain(|rc| !Rc::ptr_eq(rc, &v))
    }
}
pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(value)
}