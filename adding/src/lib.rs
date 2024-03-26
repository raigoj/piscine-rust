use std::ops::Add;
pub fn add_curry<T>(a: T) -> impl Fn(T) -> T
where T: Add<Output=T> + Copy {
    move |b: T| a + b
}