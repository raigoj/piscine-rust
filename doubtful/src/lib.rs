pub fn doubtful(s: &mut String ) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubtful_test() {
        let mut s = String::from("Hello");
        doubtful(&mut s);

        assert_eq!("Hello?", s);
    }
}