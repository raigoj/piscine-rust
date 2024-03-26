use std::cell::{RefCell, Cell};
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}
impl Workers {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(vec![])
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        self.states.borrow_mut().push(false);
        let pid = self.track_worker();
        (pid, Thread::new_thread(pid, c, self))
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len() - 1
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }
    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            // panic!("Cannot drop {}, because its already dropped", id)
            panic!("{} is already dropped", id)
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    command: String,
    workers: &'a Workers,
}
impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Self {
        Self {
            pid: p,
            command: c,
            workers: t,
        }
    }
    pub fn skill(self) {}
}
impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.workers.add_drop(self.pid)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    #[test]
    fn test_is_dropped_and_drops() {
        let worker = Workers::new();
        let (pid, thread) = worker.new_worker(String::from("gnome-shell"));
        let (pid0, thread0) = worker.new_worker(String::from("i3"));
        let (pid1, thread1) = worker.new_worker(String::from("shell"));
        let (pid2, thread2) = worker.new_worker(String::from("spotify"));
        thread.skill();
        assert_eq!(worker.drops.get(), 1_usize);
        thread0.skill();
        assert!(worker.is_dropped(pid), "{} should have been dropped", pid);
        assert!(worker.is_dropped(pid0), "{} should have been dropped", pid0);
        assert!(!worker.is_dropped(pid1), "{} should not have been dropped", pid1);
        assert!(!worker.is_dropped(pid2), "{} should not have been dropped", pid2);
        assert_eq!(worker.drops.get(), 2_usize);
        thread1.skill();
        thread2.skill();
        assert_eq!(worker.drops.get(), 4_usize);
    }
    #[test]
    fn test_using_rc() {
        // will create a new reference to the thread
        // this will test the following
        // if we drop the cloned value the RC will decrease
        // but the thread will not be dropped!
        let worker = Workers::new();
        let (_, thread) = worker.new_worker(String::from("Xorg"));
        let thread = Rc::new(thread);
        let thread_clone = thread.clone();
        assert_eq!(Rc::strong_count(&thread), 2);
        drop(thread_clone);
        assert_eq!(Rc::strong_count(&thread), 1);
    }
    #[test]
    #[should_panic(expected = "0 is already dropped")]
    fn test_drop_same_thread() {
        // test if we drop the same thread after it was already been dropped
        let worker = Workers::new();
        let (pid, thread) = worker.new_worker(String::from("gsd-rfkill"));
        let thread_clone = thread.clone();
        thread.skill();
        thread_clone.skill();
    }
}