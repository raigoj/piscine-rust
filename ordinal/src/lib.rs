
pub fn num_to_ordinal(x: u32) -> String {
    let ordinal = match x {
        11..=19 => "th",
        _ => match x % 10 {
            0 => "th",
            1 => "st",
            2 => "nd",
            3 => "rd",
            4..=9 => "th",
            _ => unreachable!(),
        }
    };
    x.to_string() + ordinal
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(num_to_ordinal(0), "0th");
        assert_eq!(num_to_ordinal(1), "1st");
        assert_eq!(num_to_ordinal(12), "12th");
        assert_eq!(num_to_ordinal(22), "22nd");
        assert_eq!(num_to_ordinal(43), "43rd");
        assert_eq!(num_to_ordinal(67), "67th");
        assert_eq!(num_to_ordinal(1901), "1901st");
    }
}
