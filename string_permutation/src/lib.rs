use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let s1hm = get_char_counts(s1);
    let s2hm = get_char_counts(s2);
    s1hm.len() == s2hm.len() &&
    s1hm.iter().all(|(ch, count)| s2hm.get(&ch).unwrap_or(&0) == count)
}
fn get_char_counts(source: &str) -> HashMap<char, u32> {
    let mut hm = HashMap::new();
    for ch in source.chars() {
        *hm.entry(ch).or_insert(0) += 1;
    }
    hm
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_permutation_test() {
        assert!(is_permutation("thought", "thougth"));
        assert!(is_permutation("dog", "god"));
        assert!(!is_permutation("dog", "cat"));
        assert!(is_permutation("✊🖐✌️", "🖐✊✌️"));
        assert!(!is_permutation("✊🖐✌️", "🔫🔫🔫"));
        assert!(!is_permutation("cde", "edbca"));
    }
}
