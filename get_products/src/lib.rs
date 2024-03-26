pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return vec![];
    }
    let total = arr.iter()
        .copied()
        .reduce(|a, b| a * b)
        .unwrap();
    arr.into_iter()
        .map(|n| total / n)
        .collect()
}