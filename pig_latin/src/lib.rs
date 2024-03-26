const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut new = word.trim_start_matches(|ch| !VOWELS.contains(&ch));
            let mut removed = word.len() - new.len();
            if &word[removed.saturating_sub(1)..=removed] == "qu" {
                removed += 1;
                new = &word[removed..];
            }
            let consonants = &word[0..removed];
            String::with_capacity(word.len() + 2) + new + consonants + "ay"
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pig_latin_test() {
        assert_eq!("iglooay", pig_latin("igloo"));
        assert_eq!("appleay", pig_latin("apple"));
        assert_eq!("ellohay", pig_latin("hello"));
        assert_eq!("aresquay", pig_latin("square"));
        assert_eq!("enonxay", pig_latin("xenon"));
        assert_eq!("airchay", pig_latin("chair"));
    }
}
