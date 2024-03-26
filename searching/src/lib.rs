pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().position(|&n| n == key)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_test() {
        let ar = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(None, search(&ar, 10));
        assert_eq!(Some(3), search(&ar, 6));
    }
}
