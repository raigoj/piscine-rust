#[derive(Debug, PartialEq, Eq)]
pub struct Student (pub i32, pub String, pub String);

pub fn id(student: &Student) -> i32 {
    return student.0
}

pub fn first_name(student: &Student) -> String {
    return (&student.1).clone()
}

pub fn last_name(student: &Student) -> String {
    return (&student.2).clone()
}
