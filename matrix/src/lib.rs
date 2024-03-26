
pub mod ops;
pub mod mult;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Scalar<Item=T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Self(
            vec![
                vec![T::zero()]
            ]
        )
    }
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Self(
            vec![vec![T::zero(); col]; row]
        )
    }
    pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Self::zero(n, n);
        for i in 0..m.0.len() {
            m.0[i][i] = T::one()
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        println!("{:?}", m);
        println!("{:?}", Matrix::<i32>::identity(4));
        println!("{:?}", Matrix::<f64>::zero(3, 4));
    }
}
use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Copy
+ Add<Output=Self::Item>
+ Sub<Output=Self::Item>
+ Mul<Output=Self::Item>
+ Div<Output=Self::Item>
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl Scalar for u32 {
    type Item = Self;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}
impl Scalar for u64 {
    type Item = Self;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}
impl Scalar for i32 {
    type Item = Self;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}
impl Scalar for i64 {
    type Item = Self;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}
impl Scalar for f32 {
    type Item = Self;
    fn zero() -> Self::Item { 0.0 }
    fn one() -> Self::Item { 1.0 }
}
impl Scalar for f64 {
    type Item = Self;
    fn zero() -> Self::Item { 0.0 }
    fn one() -> Self::Item { 1.0 }
}