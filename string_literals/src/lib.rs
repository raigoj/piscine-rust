pub fn is_empty(v: &str) -> bool {
    v.is_empty()
    // if v.as_bytes()[v.len()] == 0 {
    //     return true
    // } else {
    //     return false
    // }
}


pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
    // let is_asc: bool;
    // let len = v.len();
    // let mut i = 0;
    // loop {
    //     if len < i {
    //         is_asc = true;
    //         break;
    //     } else {
    //         if v.as_bytes()[i] > 127 {
    //             is_asc = false;
    //             break;
    //         }
    //         i += 1;
    //     }
    // }
    // return is_asc;

}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat) {
        return true 
    } else {
        return false
    }
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    let patrn = v.find(pat);
    let _ = match patrn {
        Some(x) => return x,
        None => return 0,
      };
    
    // let n = match patrn {
    //     Ok(i) => patrn,
    //     Err(n) => panic!("bad")
    // }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_test() {
        assert!(is_empty(""));
        assert!(!is_empty("hello!"))
    }

    #[test]
    fn is_ascii_test() {
        assert!(is_ascii("hello"));
        assert!(!is_ascii("😊"))
    }

    #[test]
    fn contains_test() {
        let s = String::from("Hello, world!");

        assert!(contains(&s, "llo, w"));
        assert!(!contains(&s, "kitty"))
    }

    #[test]
    fn split_at_test() {
        let s = String::from("Hello, world!");

        assert_eq!(("Hel", "lo, world!"), split_at(&s, 3));
        assert_eq!(("Hello, ", "world!"), split_at(&s, 7))
    }

    #[test]
    fn find_test() {
        let s = String::from("Hello, world!");

        assert_eq!(4, find(&s, 'o'));
        assert_eq!(7, find(&s, 'w'));
    }
}
