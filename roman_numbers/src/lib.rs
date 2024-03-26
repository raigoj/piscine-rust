use crate::RomanDigit::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);
impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!(),
        }
    }
}
const DECODE: &[&[&[RomanDigit]]] = &[
    &[&[], &[I], &[I, I], &[I, I, I], &[I, V], &[V], &[V,I], &[V, I, I], &[V, I, I, I], &[I, X]],
    &[&[], &[X], &[X, X], &[X, X, X], &[X, L], &[L], &[L,X], &[L, X, X], &[L, X, X, X], &[X, C]],
    &[&[], &[C], &[C, C], &[C, C, C], &[C, D], &[D], &[D,C], &[D, C, C], &[D, C, C, C], &[C, M]],
    &[&[], &[M], &[M, M], &[M, M, M]],
];
fn decode(tuple: (usize, usize)) -> &'static [RomanDigit]{
    let (radix, n) = tuple;
    DECODE[radix][n]
}
impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n > 3999 {
            panic!("Given number {} is too big", n)
        }
        let len = n.to_string().len();
        let mut res: Vec<_> = n.to_string().chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .enumerate()
            .map(|(i, v)| (len - 1 - i, v))
            .flat_map(decode)
            .copied()
            .collect();
        
        if res.is_empty() {
            res.push(Nulla);
        }
        
        Self(res)
    }
}
