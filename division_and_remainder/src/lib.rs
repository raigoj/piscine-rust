pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let i = x/y;
    let j = x%y;
    return (i, j)
}