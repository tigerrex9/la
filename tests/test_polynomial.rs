use la::polynomial::*;

#[test]
fn addition() {
	let a: Polynomial<u8> = Polynomial::from(vec![1,2,3]);
	let b: Polynomial<u8> = Polynomial::from(vec![4,5]);
	let c: Polynomial<u8> = Polynomial::from(vec![5,7,3]);
    assert_eq!((a + b), c);

	let a: Polynomial<u8> = Polynomial::from(vec![1,2,3]);
	let b: Polynomial<u8> = Polynomial::from(vec![4,5]);
	let c: Polynomial<u8> = Polynomial::from(vec![5,7,3]);
	assert_eq!((b + a), c);
}

#[test]
fn subtraction() {
	let a: Polynomial<i8> = Polynomial::from(vec![1,2]);
	let b: Polynomial<i8> = Polynomial::from(vec![1,2,3]);
	let c: Polynomial<i8> = Polynomial::from(vec![0,0,-3]);
    assert_eq!((a - b), c);
}

#[test]
fn scalar_multiplication() {
	let a: Polynomial<u8> = Polynomial::from(vec![1,2]);
	let b: Polynomial<u8> = Polynomial::from(vec![2,4]);
    assert_eq!((a * 2), b);
}

#[test]
fn scalar_division() {
    let a: Polynomial<u8> = Polynomial::from(vec![4,2]);
    let b: Polynomial<u8> = Polynomial::from(vec![2,1]);
    assert_eq!((a / 2), b);
}

#[test]
fn application() {
	let a: Polynomial<u8> = Polynomial::from(vec![1,2,3]);
	assert_eq!(a.at(2), 17);
}

#[test]
fn derivatation() {
	let a: Polynomial<u8> = Polynomial::from(vec![1,2,3]);
    let b: Polynomial<u8> = Polynomial::from(vec![2,6]);
	assert_eq!(a.derivative(), b);
}

#[test]
fn integration() {
	let a: Polynomial<u8> = Polynomial::from(vec![1,2]);
    let b: Polynomial<u8> = Polynomial::from(vec![0,1,1]);
	assert_eq!(a.integral(), b);
}