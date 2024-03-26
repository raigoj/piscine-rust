pub fn capitalize_first(input: &str) -> String {
    let mut string = input.to_string();
    if string.is_empty() {
        return string;
    }
    string.replace_range(0..1, &string[0..1].to_uppercase());
    string
}
pub fn title_case(input: &str) -> String {
    let mut string = String::with_capacity(input.len());
    let mut first = true;
    input.chars()
         .map(|char|
             if first {
                 first = false;
                 char.to_ascii_uppercase()
             } else {
                 if char == ' ' { first = true }
                 char
             }
         )
         .for_each(|char| string.push(char));
    string
}
pub fn change_case(input: &str) -> String {
    let mut string = String::with_capacity(input.len());
    input.chars()
         .map(|char|
             if char.is_ascii_uppercase() {
                 char.to_ascii_lowercase()
             } else {
                 char.to_ascii_uppercase()
             }
         )
         .for_each(|char| string.push(char));
    string
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capitalizing_test() {
        println!("{}", "heLLo THere".len());
        assert_eq!("Joe is missing", capitalize_first("joe is missing"));
        assert_eq!("Jill Is Leaving A", title_case("jill is leaving A"));
        assert_eq!("HEllO thERE", change_case("heLLo THere"));
    }
}