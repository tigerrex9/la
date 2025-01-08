use num::Num;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait Field: Num + Copy {}
impl<T: Num + Copy> Field for T {}

#[derive(Debug, Copy, Clone)]
pub struct Matrix<F: Field, const R: usize, const C: usize> ([[F; C]; R]); 
impl<F: Field, const R: usize, const C: usize> Deref for Matrix<F, R, C> {
    type Target = [[F; C]; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<F: Field, const R: usize, const C: usize> DerefMut for Matrix<F, R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<F: Field, const R: usize, const C: usize> Matrix<F, R, C> {
    pub fn rows(&self) -> usize {
        R
    }

    pub fn columns(&self) -> usize {
        C
    }

    pub fn get_row(&self, row_index: usize) -> Vector<F, C> {
        Vector(self[row_index])
    }

    pub fn get_column(&self, column_index: usize) -> Vector<F, R> {
        let mut column: [F; R] = [F::zero(); R];

        for i in 0..R {
            column[i] = self[i][column_index];
        }

        Vector(column)
    }

    pub fn transpose(&self) -> Matrix<F, C, R> {
        let mut result: [[F; R]; C] = [[F::zero(); R]; C];

        for i in 0..R {
            for j in 0..C {
                result[j][i] = self[i][j];
            }
        }

        Matrix(result)
    }

    /*
    pub fn reduce(&self) -> Matrix<F, R, C> { // row reduction
        let mut result: [[F; C]; R] = self.0.clone();
        // Upper Triangularize
        // For all rows below it 
        for above_index in 0..R {
            most_significant = 0;
            while result[above_index][most_significant] != F::zero() {

            }
            for below_index in above_index..R {
                scalar = 
            }
        }
    }

    pub fn rank(&self) -> usize {

    }
    */
}

#[derive(Debug, Copy, Clone)]
pub struct Vector<F: Field, const R: usize> ([F; R]);
impl<F: Field, const R: usize> Deref for Vector<F, R> {
    type Target = [F; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<F: Field, const R: usize> DerefMut for Vector<F, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<F: Field, const R: usize> Vector<F, R> {
    pub fn to_matrix(&self) -> Matrix<F, R, 1> {
        let mut column: [[F; 1]; R] = [[F::zero(); 1]; R];
        for i in 0..R {
            column[i][0] = self[i];
        }

        Matrix(column)
    }
}

// Functions 
pub fn vscale<F: Field, const R: usize>(scalar: F, vector: Vector<F, R>) -> Vector<F, R> { 
    let mut result: [F; R] = [F::zero(); R];
    for i in 0..R {
        result[i] = scalar * vector[i];
    }

    Vector(result)
}

pub fn mscale<F: Field, const R: usize, const C: usize>(scalar: F, matrix: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = scalar * matrix[i][j];
        }
    }

    Matrix(result)
}

pub fn vadd<F: Field, const R: usize>(left: Vector<F, R>, right: Vector<F, R>) -> Vector<F, R> { 
    let mut result: [F; R] = [F::zero(); R];
    for i in 0..R {
        result[i] = left[i] + right[i];
    }

    Vector(result)
}

pub fn madd<F: Field, const R: usize, const C: usize>(left: Matrix<F, R, C>, right: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = left[i][j] + right[i][j];
        }
    }

    Matrix(result)
}

pub fn dot<F: Field, const R: usize>(left: Vector<F, R>, right: Vector<F, R>) -> F {
    let mut product: F = F::zero();
    for i in 0..R {
        product = product + (left[i] * right[i]);
    }

    product
}

pub fn mmul<F: Field, const L: usize, const M: usize, const N: usize>(left: Matrix<F, L, M>, right: Matrix<F, M, N>) -> Matrix<F, L, N> { // try strassen for large?
    let mut result:[[F; N]; L] = [[F::zero(); N]; L];
    for i in 0..N {
        for j in 0..L {
            result[i][j] = dot(left.get_row(i), right.get_column(j));
        }
    }
        
    Matrix(result)
}

pub fn vmul<F: Field, const R: usize, const C: usize>(left: Matrix<F, R, C>, right: Vector<F, C>) -> Vector<F, R> {
    let mut result:[F; R]  = [F::zero(); R];
    for i in 0..R {
        result[i] = dot(left.get_row(i), right);
    }

    Vector(result)
}

pub fn identity<F: Field, const D: usize> () -> Matrix<F, D, D> {
    let mut result: [[F; D]; D] = [[F::zero(); D]; D];
    for i in 0..D {
        result[i][i] = F::one();
    }

    Matrix(result)
}

pub fn zero<F: Field, const D:usize> () -> Matrix<F, D, D> {
    Matrix([[F::zero(); D]; D])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let a: Matrix<f32, 3, 3> = Matrix([[50.0, 9.1, 33.6781],[17.3, 19.76849, 666.666],[2.0, 4.2, 1.1]]);
        let b: Matrix<f32, 3, 3> = Matrix([[1.0, 0.0, 9.3],[0.2, 9.0, 3.0],[2.1, 0.0, 0.0]]);

        let c = mmul(a, b);
        print!("{:?}\n{:?}\n{:?}\n", a, b, c);
        let v: Vector<f32, 3> = Vector([1.0, 0.0, 0.0]);
        let w = vmul(c, v);
        print!("{:?}\n", w);
    }
}
