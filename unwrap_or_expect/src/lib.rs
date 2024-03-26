pub fn odd_to_even(data: Vec<u32>) -> Result<Vec<u32>, (String, Vec<u32>)> {
    let mut a = Vec::new();
    a.extend(data.iter().filter(|&value| value % 2 == 0));
    if a.len() != 0 {
        return Err(("There is a even value in the vector!".to_string(), a));
    }
    a.extend(data.iter().map(|&value| {
        value + 1
    }));
    Ok(a)
}
pub fn expect(v: Vec<u32>) -> Vec<u32> {
    let odd_result = odd_to_even(v);
    let _expected = match odd_result {
        Ok(vec) => return vec,
        Err(error) => panic!("ERROR : {:?}", error),
    };
}
pub fn unwrap_or(v: Vec<u32>) -> Vec<u32> {
    let odd_result = odd_to_even(v);
    let empty: Vec<u32> = Vec::new();
    let _expected = match odd_result {
        Ok(vec) => return vec,
        Err(_error) => return empty,
    };
}
pub fn unwrap_err(v: Vec<u32>) -> (String, Vec<u32>) {
    let odd_result = odd_to_even(v);
    let _expected = match odd_result {
        Ok(vec) =>  panic!("called `Result::unwrap_err()` on an `Ok` value: {:?}", vec),
        Err(error) => return error, 
    };
}
pub fn unwrap(v: Vec<u32>) -> Vec<u32> {
    odd_to_even(v).unwrap()
}
pub fn unwrap_or_else(v: Vec<u32>) -> Vec<u32> {
    odd_to_even(v).unwrap_or_else(|error| { return error.1})
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR : (\"There is a even value in the vector!\", [2, 4, 6])")]
    fn expect_test() {
        let even = vec![2, 4, 6];
        expect(even);
    }

    #[test]
    fn unwrap_or_test() {
        let even = vec![2, 4, 6];
        let empty = unwrap_or(even);
        assert_eq!(Vec::<u32>::new(), empty);
    }

    #[test]
    fn unwrap_err_test() {
        let even = vec![2, 4, 6];
        let (err_str, err_vec) = unwrap_err(even);
        assert_eq!("There is a even value in the vector!", err_str);
        assert_eq!(vec![2, 4, 6], err_vec);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap_err()` on an `Ok` value: [2, 4, 6]")]
    fn unwrap_err_panic() {
        let odd = vec![1, 3, 5];
        unwrap_err(odd);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: (\"There is a even value in the vector!\", [2, 4, 6])")]
    fn unwrap_test() {
        let even = vec![2, 4, 6];
        unwrap(even);
    }

    #[test]
    fn unwrap_or_else_test() {
        let mixed = vec![1, 2, 3, 4, 5, 6];
        let res = unwrap_or_else(mixed);
        assert_eq!(vec![2, 4, 6], res)
    }
}
