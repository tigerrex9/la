use crate::matrix::Matrix;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Graph<const N: usize> ([[u8; N]; N]);
impl<const N: usize> From<[[u8; N]; N]> for Graph<N> {
    fn from(array: [[u8; N]; N]) -> Self {
        Graph(array)
    }
}
impl<const N: usize> Into<[[u8; N]; N]> for Graph<N> {
    fn into(self) -> [[u8; N]; N] {
        self.0
    }
}
impl<const N: usize> From<Matrix<u8, N, N>> for Graph<N> {
    fn from(matrix: Matrix<u8, N, N>) -> Self {
        Graph(matrix.into())
    }
}
impl<const N: usize> Into<Matrix<u8, N, N>> for Graph<N> {
    fn into(self) -> Matrix<u8, N, N> {
        Matrix::from(self.0)
    }
}