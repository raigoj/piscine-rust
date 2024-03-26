use std::collections::HashMap;
pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}
pub fn median(list: &Vec<i32>) -> i32 {
    let mut list = list.clone();
    list.sort_unstable();
    let half = list.len() / 2;
    if list.len() % 2 == 1 {
        list[half]
    } else {
        (list[half] + list[half - 1]) / 2
    }
}
pub fn mode(list: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for &n in list {
        *hm.entry(n).or_insert(0) += 1;
    }
    hm.into_iter().max_by_key(|(_, count)| *count).unwrap().0
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn hashing_test() {
        let vec = vec![4, 7, 5, 2, 5, 1, 3];
        assert_eq!(3.857142857142857, mean(&vec));
        assert_eq!(4, median(&vec));
        assert_eq!(5, mode(&vec));
    }
}