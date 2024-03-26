pub fn factorial(num: u64) -> u64 {
    
    let mut sum: u64 = 1;
    // let number: u64 = 1;
    for i in 1..num+1 {
        sum *= i;
    }
    return sum
}
