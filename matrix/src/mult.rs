use std::ops::Mul;
use crate::{Matrix, Scalar};
impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }
    pub fn number_of_rows(&self) -> usize {
        self.0.first().map(|v| v.len()).unwrap_or(0)
    }
    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }
    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter()
            .map(|v| &v[n])
            .cloned()
            .collect()
    }
    fn rows(&self) -> impl Iterator<Item=Vec<T>> + '_ {
        self.0.iter().cloned()
    }
    fn cols(&self) -> impl Iterator<Item=Vec<T>> + '_ {
        (0..self.number_of_cols()).map(|i| self.col(i))
    }
}
impl<T: Scalar<Item=T>> Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None
        }
        let res = self.rows()
            .map(|l_row| {
                rhs.cols()
                    .map(|r_col| dot_product(&l_row, &r_col))
                    .collect()
            })
            .collect();
        Some(Self(res))
    }
}
fn dot_product<T> (l_row: &[T], r_col: &[T]) -> T
where T: Scalar<Item=T> {
    l_row.iter().zip(r_col.iter())
        .fold(T::zero(), |acc, (&a, &b)| a * b + acc)
}