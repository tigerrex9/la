use la::vector::*;

#[test]
fn addition() {
	let a: Vector<u8, 2> = Vector::from([1,2]);
	let b: Vector<u8, 2> = Vector::from([2,3]);
	let c: Vector<u8, 2> = Vector::from([3,5]);
    assert_eq!((a + b), c);
}

#[test]
fn subtraction() {
	let a: Vector<u8, 2> = Vector::from([2,3]);
	let b: Vector<u8, 2> = Vector::from([1,2]);
	let c: Vector<u8, 2> = Vector::from([1,1]);
    assert_eq!((a - b), c);
}

#[test]
fn scalar_multiplication() {
	let a: Vector<u8, 2> = Vector::from([1,2]);
	let b: Vector<u8, 2> = Vector::from([2,4]);
    assert_eq!((a * 2), b);
}

#[test]
fn scalar_division() {
    let a: Vector<u8, 2> = Vector::from([4,2]);
    let b: Vector<u8, 2> = Vector::from([2,1]);
    assert_eq!((a / 2), b);
}

#[test]
fn dot_product() {
	let a: Vector<u8, 2> = Vector::from([4,2]);
    let b: Vector<u8, 2> = Vector::from([2,1]);
	assert_eq!((a * b), 10);
}
