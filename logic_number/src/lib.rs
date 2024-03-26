pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    num == num.to_string().chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .map(|n| n.pow(len))
        .reduce(|a, b| a + b)
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_test() {
        assert!(number_logic(0));
        assert!(number_logic(5));
        assert!(number_logic(9));
        assert!(!number_logic(10));
        assert!(number_logic(153));
        assert!(!number_logic(100));
        assert!(number_logic(9474));
        assert!(!number_logic(9475));
    }
}