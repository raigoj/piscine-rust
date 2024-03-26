use std::mem;
#[derive(Debug, Default)]
pub struct WorkEnvironment {
    pub grade: Link,
}
pub type Link = Option<Box<Worker>>;
#[derive(Debug)]
pub struct Worker {
    pub worker_type: String,
    pub worker_name: String,
    pub next_worker: Link,
}
impl Worker {
    fn new(t: String, name: String) -> Self {
        Self {
            worker_type: t,
            worker_name: name,
            next_worker: None,
        }
    }
}
impl WorkEnvironment {
    pub fn new() -> Self {
        Self {
            grade: None,
        }
    }
    pub fn add_worker(&mut self, t: String, name: String) {
        let old = mem::replace(
            &mut self.grade,
            Some(Box::new(Worker::new(t, name)))
        );
        self.grade.as_mut().unwrap().next_worker = old
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|mut worker| {
            self.grade = worker.next_worker.take();
            worker.worker_name
        })
    }
    pub fn search_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| (worker.worker_name.clone(), worker.worker_type.clone()))
    }
}
