use std::collections::HashMap;
pub fn contain(h: &HashMap<&str, i32>, s: &str) -> bool {
    h.contains_key(s)
}
pub fn remove(h: &mut HashMap<&str, i32>, s: &str) {
    h.remove(s);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn simple_hash_test() {
        let mut h = HashMap::from([
            ("Katie", 334),
            ("Daniel", 122),
            ("Ashley", 333),
        ]);
        assert!(contain(&h, "Katie"));
        remove(&mut h, "Katie");
        assert!(!contain(&h, "Katie"));
    }
}