use num::Num;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<F: Num + Copy, const R: usize> ([F; R]);
impl<F: Num + Copy, const R: usize> ops::Deref for Vector<F, R> {
    type Target = [F; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<F: Num + Copy, const R: usize> ops::DerefMut for Vector<F, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<F: Num + Copy, const R: usize> ops::Add<Vector<F, R>> for Vector<F, R> {
    type Output = Vector<F, R>;

    fn add(self, right: Vector<F, R>) -> Vector<F, R> {
        vadd(self, right)
    }
}
impl<F: Num + Copy, const R: usize> ops::Sub<Vector<F, R>> for Vector<F, R> {
    type Output = Vector<F, R>;

    fn sub(self, right: Vector<F, R>) -> Vector<F, R> {
        vsub(self, right)
    }
}
impl<F: Num + Copy, const R: usize> ops::Mul<F> for Vector<F, R> {
    type Output = Vector<F, R>;

    fn mul(self, left: F) -> Vector<F, R> {
        vscale(left, self)
    }
}
impl<F: Num + Copy, const R: usize> ops::Div<F> for Vector<F, R> {
    type Output = Vector<F, R>;

    fn div(self, left: F) -> Vector<F, R> {
        vdiv(left, self)
    }
}
impl<F: Num + Copy, const R: usize> ops::Mul<Vector<F, R>> for Vector<F, R> {
    type Output = F;

    fn mul(self, right: Vector<F, R>) -> F {
        dot(self, right)
    }
}
impl<F: Num + Copy, const R: usize> Vector<F, R> {
    pub fn from(array: [F; R]) -> Vector<F, R> {
        Vector(array)
    }

    pub fn to_array(&self) -> [F; R] {
        self.0
    }
}

pub fn vscale<F: Num + Copy, const R: usize>(scalar: F, vector: Vector<F, R>) -> Vector<F, R> { 
    let mut result: [F; R] = [F::zero(); R];
    for i in 0..R {
        result[i] = scalar * vector[i];
    }

    Vector::from(result)
}

pub fn vdiv<F: Num + Copy, const R: usize>(scalar: F, vector: Vector<F, R>) -> Vector<F, R> { 
    let mut result: [F; R] = [F::zero(); R];
    for i in 0..R {
        result[i] = vector[i] / scalar;
    }

    Vector::from(result)
}

pub fn vadd<F: Num + Copy, const R: usize>(left: Vector<F, R>, right: Vector<F, R>) -> Vector<F, R> { 
    let mut result: [F; R] = [F::zero(); R];
    for i in 0..R {
        result[i] = left[i] + right[i];
    }

    Vector::from(result)
}

pub fn vsub<F: Num + Copy, const R: usize>(left: Vector<F, R>, right: Vector<F, R>) -> Vector<F, R> { 
    let mut result: [F; R] = [F::zero(); R];
    for i in 0..R {
        result[i] = left[i] - right[i];
    }

    Vector::from(result)
}

pub fn dot<F: Num + Copy, const R: usize>(left: Vector<F, R>, right: Vector<F, R>) -> F {
    let mut product: F = F::zero();
    for i in 0..R {
        product = product + (left[i] * right[i]);
    }

    product
}
