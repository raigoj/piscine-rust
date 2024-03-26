use std::cmp::Ordering;
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<_> = phrase.split(' ').collect();
    words.sort_by_key(|&a| {
        let i = a.find(NUMBERS).unwrap();
        &a[i..=i]
    });
    let mut result = String::with_capacity(phrase.len());
    let mut first = true;
    for word in words {
        if !first { result.push(' '); }
        word.chars().filter(|c| !NUMBERS.contains(c))
            .for_each(|c| result.push(c));
        first = false;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arrange_phrase_test() {
        assert_eq!("This is a Test", arrange_phrase("is2 Thi1s T4est 3a"))
    }
}