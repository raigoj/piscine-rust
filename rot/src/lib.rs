pub fn rotate(input: &str, key: i8) -> String {
    let key = key.rem_euclid(26) as u8;
    input.chars()
        .map(|ch| {
            if ch.is_ascii_lowercase() {
                ((ch as u8 - b'a' + key).rem_euclid(b'z' - b'a' + 1) + b'a') as char
            } else if ch.is_ascii_uppercase() {
                ((ch as u8 - b'A' + key).rem_euclid(b'Z' - b'A' + 1) + b'A') as char
            } else {
                ch
            }
        })
        .collect()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!("z", rotate("m", 13));
        assert_eq!("n", rotate("m", 1));
        assert_eq!("a", rotate("a", 26));
        assert_eq!("M J Q Q T", rotate("H E L L O", 5));
        assert_eq!("Gur svir obkvat jvmneqf whzc dhvpxyl.", rotate("The five boxing wizards jump quickly.", 13));
        assert_eq!("Xiwxmrk amxl ryqfivw 1 2 3", rotate("Testing with numbers 1 2 3", 4));
        assert_eq!("Fqefuzs", rotate("Testing", -14));
    }
}