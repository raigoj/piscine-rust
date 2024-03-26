pub fn fibonacci(n: u32) -> u32 {
    if n % 2 != 0 {
        let mut a = 0;
        let mut b = 1;
        for _ in 1..(n+1)/2 {
            a += b;
            b += a;
        }
        return b
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 1..(n+2)/2 {
            a += b;
            b += a;
        }
        return a
    }
}
