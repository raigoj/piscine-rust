pub fn first_subword(s: String) -> String {
    let i = s[1..].chars().position(|c| c.is_uppercase() || c == '_').unwrap_or(s.len()-1);
    s[0..i+1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_subword_test() {
        let s1 = String::from("helloWorld");
        let s2 = String::from("snake_case");
        let s3 = String::from("CamelCase");
        let s4 = String::from("just");
    
        assert_eq!("hello", first_subword(s1));
        assert_eq!("snake", first_subword(s2));
        assert_eq!("Camel", first_subword(s3));
        assert_eq!("just", first_subword(s4));
    }
}