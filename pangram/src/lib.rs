const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
pub fn is_pangram(s: &str) -> bool {
    ALPHABET.chars().all(|ch| s.contains(ch) || s.contains(ch.to_ascii_uppercase()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pangram_test() {
        assert!(is_pangram("The quick brown fox jumps over the lazy dog"));
        assert!(!is_pangram("The quick brown fox jumps over the lazy cat"));
    }
}
