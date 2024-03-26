#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);
impl<T> Add<Self> for Vector<T>
where T: Scalar<Item=T>
{
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        Some(Self(
            self.0.into_iter().zip(rhs.0.into_iter())
                .map(|(r, l)| r + l)
                .collect()
        ))
    }
}
impl<T> Add<&Self> for Vector<T>
where T: Scalar<Item=T>
{
    type Output = Option<Self>;
    fn add(self, rhs: &Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        Some(Self(
            self.0.into_iter().zip(rhs.0.iter())
                .map(|(r, l)| r + *l)
                .collect()
        ))
    }
}
impl<T> Vector<T>
where T: Scalar<Item=T>
{
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        self.0.iter().cloned().zip(other.0.iter().cloned())
            .map(|(r, l)| r * l)
            .reduce(<T as Add>::add)
    }
}
use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Copy
+ Add<Output=Self>
+ Sub<Output=Self>
+ Mul<Output=Self>
+ Div<Output=Self>
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