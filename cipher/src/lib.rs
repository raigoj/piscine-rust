#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }
    
    let compare = original.chars().map(|ch| {
        if ch.is_ascii_alphabetic() {
            if ch.is_ascii_lowercase() {
                (b'a' + b'z' - ch as u8) as char
            } else {
                (b'A' + b'Z' - ch as u8) as char
            }
        } else {
            ch   
        }}).collect();

    if compare != ciphered {
                return Some(Err(CipherError::new(false, compare)))  
    }
    Some(Ok(true)) 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cipher_test() {
        assert_eq!(Some(Ok(true)), cipher("1Hello 2world!", "1Svool 2dliow!"));
        assert_eq!(Some(Err(CipherError::new(false, "1Svool 2dliow!".to_string()))), cipher("1Hello 2world!", "svool"));
        assert_eq!(None, cipher("", "svool"));
    }
}