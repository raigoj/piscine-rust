pub struct Message {
    content: String,
    user: String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
    let new_message = Message {
        content: ms,
        user: u,
    };
    new_message
  }
  pub fn send_ms(&self) -> Option<&str> {
    let content_from = &self.content;
    let empty : Option::<&str> = None;
    if content_from.contains("stupid") {
        return empty
    } else if content_from == "" {
        return empty
    } else {
        return Some(&content_from)
    }
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    let checked = ms.send_ms();
    if checked == None {
        return (false, "ERROR: illegal")
    } else {
        return (true, &ms.content)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profanity_test() {
        let m0 = Message::new("hello there".to_string(), "toby".to_string());
        assert_eq!((true, "hello there"), check_ms(&m0));

        let m1 = Message::new("".to_string(), "toby".to_string());
        assert_eq!((false, "ERROR: illegal"), check_ms(&m1));

        let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
        assert_eq!((false, "ERROR: illegal"), check_ms(&m2));

        let m3 = Message::new("stupid".to_string(), "toby".to_string());
        assert_eq!((false, "ERROR: illegal"), check_ms(&m3));
    }
}