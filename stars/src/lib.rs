pub fn stars(n: u32) -> String {
    "*".repeat(2_usize.pow(n))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stars_test() {
        assert_eq!("**", stars(1));
        assert_eq!("****************", stars(4));
        assert_eq!("********************************", stars(5));
    }
}
