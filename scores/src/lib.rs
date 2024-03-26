use std::collections::HashMap;
fn value_map() -> HashMap<char, u64> {
    let values = vec![
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ];
    values.into_iter()
        .flat_map(|(v, chs)|
            chs.into_iter()
            .map(move |ch| (ch, v))
        ).collect()
}
pub fn score(s: &str) -> u64 {
    let map = value_map();
    s.chars()
        .map(|ch| ch.to_ascii_uppercase())
        .fold(0, |acc, ch| acc + map.get(&ch).unwrap_or(&0))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(1, score("a"));
        assert_eq!(0, score("ã ê Á?"));
        assert_eq!(14, score("ThiS is A Test"));
    }
}
