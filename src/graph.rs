use crate::matrix::Matrix;

pub struct Graph<const N: usize> (Matrix<u8, N, N>);