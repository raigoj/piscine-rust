use chrono::Duration;
#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}
#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}
use std::fmt;
use std::fmt::{Formatter};
impl fmt::Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, \x1b[38;2;{};{};{}m{}\x1b[0m)", self.position, self.size, self.color.0, self.color.1, self.color.2, self.content)
    }
}
pub use Event::*;
impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(s) => Notification{
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: s.to_string(),
            },
            Registration(d) => Notification{
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!("You have {}H:{}M:{}S left before the registration ends", d.num_hours(), d.num_minutes() % 60, d.num_seconds() % 60),
            },
            Appointment(s) => Notification{
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: s.to_string(),
            },
            Holiday => Notification{
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let remainder = Remainder("Go to the doctor");
        println!("{}", remainder.notify());
        let registration = Registration(Duration::seconds(49094));
        println!("{}", registration.notify());
        let appointment = Appointment("Go to the doctor");
        println!("{}", appointment.notify());
        let holiday = Holiday;
        println!("{}", holiday.notify());
    }
}