mod err;
use err::{ ParseErr, ReadErr };
pub use json::{parse, stringify};
pub use std::error::Error;
use std::fs::File;
use std::io::Read;
use json::JsonValue;
#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}
#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}
impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(Box::new(ReadErr {child_err: Box::new(e)})),
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => {},
            Err(e) => return Err(Box::new(ReadErr {child_err: Box::new(e)})),
        };
        let parsed = match parse(&content) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        };
        let title = match parsed["title"].as_str() {
            None => return Err(Box::new(ParseErr::Empty)),
            Some(s) => s.to_string(),
        };
        if !parsed["tasks"].is_array() {
            return Err(Box::new(ParseErr::Empty));
        }
        let tasks = match json_array_to_tasks(&parsed["tasks"]) {
            None => return Err(Box::new(ParseErr::Empty)),
            Some(v) => v,
        };
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        Ok(TodoList {
            title,
            tasks,
        })
    }
}
fn json_array_to_tasks(arr: &JsonValue) -> Option<Vec<Task>> {
    let mut tasks: Vec<Task> = Vec::new();
    for i in 0.. {
        if arr[i].is_null() {
            break
        }
        let task = Task {
            id: arr[i]["id"].as_u32()?,
            description: arr[i]["description"].as_str()?.to_string(),
            level: arr[i]["level"].as_u32()?,
        };
        tasks.push(task);
    }
    Some(tasks)
}
#[cfg(test)]
mod tests {
    use std::{fs, io};
    use std::io::Write;
    use super::*;
    const CORRECT: &str = r#"{"title":"TODO TITLE","tasks":[{"id":0,"description":"do this","level":0},{"id":1,"description":"do that","level":5}]}"#;
    const MALFORMED: &str = r#"{"title":"TODO TITLE","tasks":[{"id":0,"description":"do this","level":0},{"id":1,"description":"do that","level":5},]}"#;
    const EMPTY: &str = r#"{"title":"TODO TITLE","tasks":[]}"#;
    #[test]
    fn open_test() {
        let err = TodoList::get_todo("file_does_not_exist.json").unwrap_err();
        assert_eq!("Fail to read todo file", err.to_string());
        let source: &io::Error = err.source().expect("Error doesn't have source")
            .downcast_ref().expect("Expected error to be io::Error");
        println!("{}", source);
        assert_eq!(io::ErrorKind::NotFound, source.kind(), "Error is not of the expected kind");
    }
    #[test]
    fn correct_test() {
        fs::write("test_correct.json", CORRECT).expect("Failed to write test file");
        let todos = TodoList::get_todo("test_correct.json").expect("Failed to get todos: ");
        assert_eq!("TODO TITLE", todos.title);
        let v = vec![
            Task {id: 0, description: "do this".to_string(), level: 0},
            Task {id: 1, description: "do that".to_string(), level: 5},
        ];
        assert_eq!(v, todos.tasks);
        fs::remove_file("test_correct.json").expect("Could not remove test file");
    }
    #[test]
    fn malformed_test() {
        fs::write("test_malformed.json", MALFORMED).expect("Failed to write test file");
        let err = TodoList::get_todo("test_malformed.json").expect_err("Shouldn't read invalid json");
        assert_eq!("Fail to parses todo", err.to_string()); // Typo here is intentional
        assert_eq!("Fail to parses todo", err.source().unwrap().to_string());
        fs::remove_file("test_malformed.json").expect("Could not remove test file");
    }
    #[test]
    fn empty_test() {
        fs::write("test_empty.json", EMPTY).expect("Failed to write test file");
        let err = TodoList::get_todo("test_empty.json").expect_err("Shouldn't read empty list");
        assert_eq!("Fail to parses todo", err.to_string()); // Typo here is intentional
        assert!(err.source().is_none());
        fs::remove_file("test_empty.json").expect("Could not remove test file");
    }
}