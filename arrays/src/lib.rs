pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}
pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrays_test() {
        let a32 = thirtytwo_tens();
        let a1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let a2: Vec<i32> = (1..11).into_iter().collect();
        let b = [5; 10];

        assert_eq!(320, sum(&a32));
        assert_eq!(55, sum(&a1));
        assert_eq!(55, sum(&a2));
        assert_eq!(50, sum(&b));
    }
}