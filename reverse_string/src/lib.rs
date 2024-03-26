pub fn rev_str(input: &str) -> String {
    let mut rev_strng = String::new();
    let len = input.len();
    for i in input.chars().rev() {
        rev_strng.push(i)
    }
    return rev_strng
}
