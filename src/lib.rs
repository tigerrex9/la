use num::Num;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait Field: Num + Copy {}
impl Field for i32 {}

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

    pub fn scale(&mut self, scalar: F) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = scalar * self[i][j];
            }
        }
    }
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

// Functions 
pub fn dot<F: Field, const R: usize>(left: Vector<F, R>, right: Vector<F, R>) -> F {
    let mut product: F = F::zero();
    for i in 0..R {
        product = product + (left[i] * right[i]);
    }

    product
}

pub fn smul<F: Field, const R: usize, const C: usize>(scalar: F, right: Matrix<F, R, C>) -> Matrix<F, R, C> { 
    let mut result: [[F; C]; R] = [[F::zero(); C]; R];
    for i in 0..R {
        for j in 0..C {
            result[i][j] = scalar * right[i][j];
        }
    }

    Matrix(result)
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
        let a: Matrix<i32, 2, 3> = Matrix([[1,2,3],[4,5,6]]);
        let b: Matrix<i32, 3, 2> = Matrix([[7,8],[9,10],[11,12]]);
        let c: Matrix<i32, 2, 2> = mmul(a, b);
        print!("{:?}\n", c);

        let d: Matrix<i32, 3, 3> = identity();
        print!("{:?}\n", d);
        let v: Vector<i32, 3> = Vector([1,2,3]);
        let w = vmul(d, v);
        print!("{:?}\n", w);

        let e: Matrix<i32, 3, 3> = zero();
        print!("{:?}\n", e);
        let x = vmul(e, v);
        print!("{:?}\n", x);
    }
}
