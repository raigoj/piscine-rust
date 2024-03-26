pub fn get_diamond(c: char) -> Vec<String> {
    if !c.is_ascii_uppercase() { panic!() }
    let n = c as usize - 'A' as usize;
    let width = 1 + n * 2;
    let mid = width / 2;
    let mut res = Vec::with_capacity(width);
    for i in (0..=n).chain((0..n).rev()) {
        let mut row = vec![' '; width];
        row[mid - i] = (i as u8 + b'A') as char;
        row[mid + i] = (i as u8 + b'A') as char;
        res.push(row.into_iter().collect())
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(vec!["A"], get_diamond('A'));
        assert_eq!(vec!["  A  ", " B B ", "C   C", " B B ", "  A  "], get_diamond('C'));
    }
}
