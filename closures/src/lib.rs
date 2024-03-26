pub fn first_fifty_even_square() -> Vec<i32> {
    (1_i32..)
        .filter(|n| n % 2 == 0)
        .map(|n| n.pow(2))
        .take(50)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("Hello, world!");
        let v1 = first_fifty_even_square();
        println!("All elements in {:?}, len = {}", v1, v1.len());
    }
}
