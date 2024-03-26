
use crate::{Matrix};
use std::ops::{ Add, Sub };
impl<T> Matrix<T> {
    pub(crate) fn same_dimension(&self, rhs: &Self) -> bool {
        self.0.len() == rhs.0.len() ||
            self.0.iter().zip(rhs.0.iter())
                .all(|(a, b)| a.len() == b.len())
    }
    pub(crate) fn zip_map(self, other: Self, func: impl Fn(T, T) -> T) -> impl Iterator<Item = Vec<T>> {
        self.0.into_iter().zip(other.0.into_iter())
            .map(move |(a_vec, b_vec)| {
                a_vec.into_iter().zip(b_vec.into_iter())
                    .map(|(a, b)| func(a, b))
                    .collect::<Vec<_>>()
            })
    }
}
impl<T: Add<Output=T>> Add for Matrix<T> {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        if !self.same_dimension(&rhs) {
            return None
        }
        let vec = self.zip_map(rhs, |a, b| a + b).collect();
        Some(Self(vec))
    }
}
impl<T: Sub<Output=T>> Sub for Matrix<T> {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        if !self.same_dimension(&rhs) {
            return None
        }
        let vec = self.zip_map(rhs, |a, b| a - b).collect();
        Some(Self(vec))
    }
}