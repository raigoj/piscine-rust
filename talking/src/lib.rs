pub fn talking(text: &str) -> &str {
    println!("question: {}", text);
    if text.trim().is_empty() {
        return "Just say something!"
    }
    if !text.chars().any(|ch| ch.is_lowercase()) && text.chars().any(|ch| ch.is_uppercase()) {
        if text.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!("There is no need to yell, calm down!", talking("JUST DO IT!"));
        assert_eq!("Sure.", talking("Hello how are you?"));
        assert_eq!("Quiet, I am thinking!", talking("WHAT'S GOING ON?"));
        assert_eq!("Interesting", talking("something"));
        assert_eq!("Just say something!", talking(""));
    }
}