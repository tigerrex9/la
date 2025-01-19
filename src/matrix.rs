use num::Num;
use std::ops;
use crate::vector::Vector;
use crate::vector::dot;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<F: Num + Copy, const R: usize, const C: usize> ([[F; C]; R]); 
impl<F: Num + Copy, const R: usize, const C: usize> ops::Deref for Matrix<F, R, C> {
    type Target = [[F; C]; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> ops::DerefMut for Matrix<F, R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> ops::Add<Matrix<F, R, C>> for Matrix<F, R, C> {
    type Output = Matrix<F, R, C>;

    fn add(self, right: Matrix<F, R, C>) -> Matrix<F, R, C> {
        madd(self, right)
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> ops::Sub<Matrix<F, R, C>> for Matrix<F, R, C> {
    type Output = Matrix<F, R, C>;

    fn sub(self, right: Matrix<F, R, C>) -> Matrix<F, R, C> {
        msub(self, right)
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> ops::Mul<F> for Matrix<F, R, C> {
    type Output = Matrix<F, R, C>;

    fn mul(self, left: F) -> Matrix<F, R, C> {
        mscale(left, self)
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> ops::Div<F> for Matrix<F, R, C> {
    type Output = Matrix<F, R, C>;

    fn div(self, left: F) -> Matrix<F, R, C> {
        mdiv(left, self)
    }
}
impl<F: Num + Copy, const L: usize, const M: usize, const N: usize> ops::Mul<Matrix<F, M, N>> for Matrix<F, L, M> {
    type Output = Matrix<F, L, N>;

    fn mul(self, right: Matrix<F, M, N>) -> Matrix<F, L, N> {
        mmul(self, right)
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> ops::Mul<Vector<F, C>> for Matrix<F, R, C> {
    type Output = Vector<F, R>;

    fn mul(self, right: Vector<F, C>) -> Vector<F, R> {
        vmul(self, right)
    }
}
impl<F: Num + Copy, const R: usize, const C: usize> Matrix<F, R, C> {
    pub fn from(array: [[F; C]; R]) -> Matrix<F, R, C> {
        Matrix(array)
    }

    pub fn to_array(&self) -> [[F; C]; R] {
        self.0
    }

    pub fn rows(&self) -> usize {
        R
    }

    pub fn columns(&self) -> usize {
        C
    }

    pub fn get_row(&self, row_index: usize) -> Vector<F, C> {
        Vector::from(self[row_index])
    }

    pub fn get_column(&self, column_index: usize) -> Vector<F, R> {
        let mut column: [F; R] = [F::zero(); R];

        for i in 0..R {
            column[i] = self[i][column_index];
        }

        Vector::from(column)
    }

    pub fn transpose(&self) -> Matrix<F, C, R> {
        let mut result: [[F; R]; C] = [[F::zero(); R]; C];

        for i in 0..R {
            for j in 0..C {
                result[j][i] = self[i][j];
            }
        }

        Matrix::from(result)
    }

    pub fn swap(&self, first_row: usize, second_row: usize) -> Matrix<F, R, C> { // could improve function by not using temp and cloning from rows of self
        let mut result: [[F; C]; R] = self.0.clone();
        let temp: [F; C] = result[first_row];
        result[first_row] = result[second_row];
        result[second_row] = temp;

        Matrix::from(result)
    }

    pub fn reduce(&self) -> Matrix<F, R, C> { // I am getting some pretty serious error with this function
        // I considered a recursive approach to this, 
        // but the conceptual simplicity comes at the cost of performance.

        let mut result: [[F; C]; R] = self.0.clone();
        let mut row: usize = 0;

        for col in 0..C { // for every column
            if row >= R {
                break;
            }

            // find a row with a non-zero entry
            let mut pivot_row: usize = R; // invalid row
            for i in row..R {
                if result[i][col] != F::zero() {
                    pivot_row = i;
                    break;
                }
            }

            // if no rows have a non-zero pivot, go to next column
            if pivot_row == R {
                continue;
            }

            // move row with non-zero column to top
            let temp: [F; C] = result[row];
            result[row] = result[pivot_row];
            result[pivot_row] = temp;

            // scale row to have 1 in pivot column
            let scale: F = result[row][col];
            for j in col..C { // could iterate from (col + 1) and set result[row][col] = F::one() for optimization
                result[row][j] = result[row][j] / scale;
            }

            // eliminate the column from all other rows
            for i in 0..R {
                if i == row {
                    continue;
                }

                let scale: F = result[i][col];
                for j in col..C { // start at pivot column because all columns before are 0
                    result[i][j] = result[i][j] - (scale * result[row][j]);
                }
            }

            row += 1;
        }

        Matrix::from(result)
    }

    pub fn rank(&self) -> usize {
        let is_empty = |row: [F; C]| -> bool { // idk if this is how you are supposed to use closures but whatever
            for i in row {
                if i != F::zero() {
                    return false;
                }
            }

            true
        };

        let reduced: Matrix<F, R, C> = self.reduce();

        let mut first_empty_row: usize = 0;
        while first_empty_row < R { // find first empty row (if one exists)
            if is_empty(reduced.0[first_empty_row]) {
                break;
            }

            first_empty_row += 1;
        }
        
        first_empty_row
    }
}

pub fn mscale<F: Num + Copy, const R: usize, const C: usize>(scalar: F, matrix: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = scalar * matrix[i][j];
        }
    }

    Matrix::from(result)
}

pub fn mdiv<F: Num + Copy, const R: usize, const C: usize>(scalar: F, matrix: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = matrix[i][j] / scalar;
        }
    }

    Matrix::from(result)
}

pub fn madd<F: Num + Copy, const R: usize, const C: usize>(left: Matrix<F, R, C>, right: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = left[i][j] + right[i][j];
        }
    }

    Matrix::from(result)
}

pub fn msub<F: Num + Copy, const R: usize, const C: usize>(left: Matrix<F, R, C>, right: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = left[i][j] - right[i][j];
        }
    }

    Matrix::from(result)
}

pub fn maugment<F: Num + Copy, const R: usize, const C1: usize, const C2: usize, const C1C2: usize> (left: Matrix<F, R, C1>, right: Matrix<F, R, C2>) -> Matrix<F, R, C1C2> { // :(
    // Matrix<F, R, (C1 + C2)> isn't legal without #![feature(generic_const_exprs)], 
    // so have to use assert for now.
    const { assert!((C1 + C2) == C1C2) }

    let mut result: [[F; C1C2]; R] = [[F::zero(); C1C2]; R];
    for i in 0..R {
        for j in 0..C1 {
            result[i][j] = left[i][j];
        }

        for j in 0..C2 {
            result[i][j + C1] = right[i][j];
        }
    }

    Matrix::from(result)
}

pub fn vaugment<F: Num + Copy, const R: usize, const C: usize, const C1: usize> (left: Matrix<F, R, C>, right: Vector<F, R>) -> Matrix<F, R, C1> { // :(
    // Matrix<F, R, (C + 1)> isn't legal without #![feature(generic_const_exprs)], 
    // so have to use assert for now.
    const { assert!(C1 == C + 1) }

    let mut result: [[F; C1]; R] = [[F::zero(); C1]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = left[i][j];
        }
        result[i][C] = right[i];
    }

    Matrix::from(result)
}

pub fn mmul<F: Num + Copy, const L: usize, const M: usize, const N: usize>(left: Matrix<F, L, M>, right: Matrix<F, M, N>) -> Matrix<F, L, N> { // try strassen for large?
    let mut result:[[F; N]; L] = [[F::zero(); N]; L];
    for i in 0..N {
        for j in 0..L {
            result[i][j] = dot(left.get_row(i), right.get_column(j));
        }
    }
        
    Matrix::from(result)
}

pub fn vmul<F: Num + Copy, const R: usize, const C: usize>(left: Matrix<F, R, C>, right: Vector<F, C>) -> Vector<F, R> {
    let mut result:[F; R]  = [F::zero(); R];
    for i in 0..R {
        result[i] = dot(left.get_row(i), right);
    }

    Vector::from(result)
}

pub fn inverse<F: Num + Copy, const R: usize, const R2: usize> (matrix: Matrix<F, R, R>) -> Matrix<F, R, R> { // :( Have to specify R2 when calling
    // Matrix<F, R, (R + R)> isn't legal without #![feature(generic_const_exprs)], 
    // so have to use assert for now.
    const { assert!((R + R) == R2) }
    assert!(matrix.rank() == R);

    let augmented: Matrix<F, R, R2> = maugment(matrix, identity());
    let reduced: Matrix<F, R, R2> = augmented.reduce();

    let mut result: [[F; R]; R] = [[F::zero(); R]; R];
    for i in 0..R {
        for j in 0..R {
            result[i][j] = reduced[i][j + R];
        }
    }

    Matrix::from(result)
}

pub fn outer<F: Num + Copy, const R: usize, const C: usize> (left: Vector<F, R>, right: Vector<F, C>) -> Matrix<F, R, C>{
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = left[i] * right[j];
        }
    }

    Matrix::from(result)
}

pub fn identity<F: Num + Copy, const D: usize> () -> Matrix<F, D, D> {
    let mut result: [[F; D]; D] = [[F::zero(); D]; D];
    for i in 0..D {
        result[i][i] = F::one();
    }

    Matrix::from(result)
}

pub fn zero<F: Num + Copy, const D:usize> () -> Matrix<F, D, D> {
    Matrix::from([[F::zero(); D]; D])
}
