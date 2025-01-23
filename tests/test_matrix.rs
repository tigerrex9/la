use la::matrix::*;
use la::vector::*;

#[test]
fn addition() {
    let a: Matrix<u8, 2, 2> = Matrix::from([[1,1],[1,1]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[1,0],[0,1]]);
    let c: Matrix<u8, 2, 2> = Matrix::from([[2,1],[1,2]]);
    assert_eq!((a + b), c);
}

#[test]
fn subtraction() {
    let a: Matrix<u8, 2, 2> = Matrix::from([[1,1],[1,1]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[1,0],[0,1]]);
    let c: Matrix<u8, 2, 2> = Matrix::from([[0,1],[1,0]]);
    assert_eq!((a - b), c);
}

#[test]
fn scalar_multiplication() {
	let a: Matrix<u8, 2, 2> = Matrix::from([[1,2],[3,4]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[2,4],[6,8]]);
    assert_eq!((a * 2), b);
}

#[test]
fn scalar_division() {
    let a: Matrix<u8, 2, 2> = Matrix::from([[4,2],[2,0]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[2,1],[1,0]]);
    assert_eq!((a / 2), b);
}

#[test]
fn matrix_multiplication() {
	let a: Matrix<u8, 2, 2> = Matrix::from([[1,1],[1,1]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[1,2],[3,4]]);
	let c: Matrix<u8, 2, 2> = Matrix::from([[4,6],[4,6]]);
    assert_eq!((a * b), c);
}

#[test]
fn vector_matrix_multiplication() {
	let a: Matrix<u8, 2, 2> = Matrix::from([[1,2],[3,4]]);
    let v: Vector<u8, 2> = Vector::from([1,2]);
	let w: Vector<u8, 2> = Vector::from([5,11]);
    assert_eq!((a * v), w);
}

#[test]
fn matrix_augmentation() {
    let a: Matrix<u8, 2, 2> = Matrix::from([[1,1],[3,3]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[2,2],[4,4]]);
    let c: Matrix<u8, 2, 4> = Matrix::from([[1,1,2,2], [3,3,4,4]]);
    assert_eq!(maugment(a, b), c);
}

#[test]
fn vector_augmentation() {
	let a: Matrix<u8, 2, 2> = Matrix::from([[1,1],[3,3]]);
    let v: Vector<u8, 2> = Vector::from([2,4]);
	let b: Matrix<u8, 2, 3> = Matrix::from([[1,1,2],[3,3,4]]);
    assert_eq!(vaugment(a,v), b);
}

#[test]
fn reduction() {
    let a: Matrix<u8, 2, 2> = Matrix::from([[2,2],[2,2]]);
    let b: Matrix<u8, 2, 2> = Matrix::from([[1,1],[0,0]]);
    assert_eq!(a.reduce(), b);
    assert_eq!(a.rank(), 1);
}

#[test]
fn inversion() {
    let a: Matrix<f64, 2, 2> = Matrix::from([[1.0,2.0],[3.0,4.0]]);
    let b: Matrix<f64, 2, 2> = Matrix::from([[-2.0,1.0],[1.5,-0.5]]);
    //print!{"{:?}\n{:?}\n", a, b}
    assert_eq!(inverse(a), b);
}

#[test]
fn outer_product() {
    let v: Vector<usize, 3> = Vector::from([1,2,3]);
    let a: Matrix<usize, 3, 3>= Matrix::from([[1,2,3],[2,4,6],[3,6,9]]);
    assert_eq!(outer(v, v), a);
}