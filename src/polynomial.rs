use num::{Num, NumCast};
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<F: Num + Copy> (Vec<F>);
impl<F: Num + Copy> ops::Deref for Polynomial<F> {
    type Target = Vec<F>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<F: Num + Copy> ops::DerefMut for Polynomial<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<F: Num + Copy> ops::Add<Polynomial<F>> for Polynomial<F> {
    type Output = Polynomial<F>;

    fn add(self, right: Polynomial<F>) -> Polynomial<F> {
        padd(self, right)
    }
}
impl<F: Num + Copy> ops::Sub<Polynomial<F>> for Polynomial<F> {
    type Output = Polynomial<F>;

    fn sub(self, right: Polynomial<F>) -> Polynomial<F> {
        psub(self, right)
    }
}
impl<F: Num + Copy> ops::Mul<F> for Polynomial<F> {
    type Output = Polynomial<F>;

    fn mul(self, left: F) -> Polynomial<F> {
        pscale(left, self)
    }
}
impl<F: Num + Copy> ops::Div<F> for Polynomial<F> {
    type Output = Polynomial<F>;

    fn div(self, left: F) -> Polynomial<F> {
        pdiv(left, self)
    }
}
impl<F: Num + Copy> Polynomial<F> {
    pub fn from(vec: Vec<F>) -> Polynomial<F> {
        Polynomial(vec)
    }

    pub fn to_vec(&self) -> Vec<F> {
        self.0.clone()
    }

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn degree(&self) -> usize {
		self.0.len() - 1
	}

	pub fn at(&self, x: F) -> F {
		let mut result: F = F::zero();
		for i in 0..self.len() {
			let mut summand: F = self[i];
			for _ in 0..i {
				summand = summand * x;
			}
			result = result + summand;
		}

		result
	}
}
impl<F: Num + Copy + NumCast> Polynomial<F> {
	pub fn derivative(&self) -> Polynomial<F> {
		let mut result: Vec<F> = Vec::new();

		for i in 1..self.len() {
			result.push(self[i] * NumCast::from(i).unwrap());
		}

		Polynomial::from(result)
	}

	pub fn integral(&self) -> Polynomial<F> {
		// takes \int_{0}^{x} P(t) dt
		let mut result: Vec<F> = Vec::new();

		result.push(F::zero());
		for i in 0..self.len() {
			result.push(self[i] / NumCast::from(i + 1).unwrap());
		}

		Polynomial::from(result)
	}
}

pub fn pscale<F: Num + Copy>(scalar: F, polynomial: Polynomial<F>) -> Polynomial<F> { 
    let mut result: Vec<F> = Vec::new();
    for i in 0..polynomial.len() {
    	result.push(scalar * polynomial[i]);
    }

    Polynomial::from(result)
}

pub fn pdiv<F: Num + Copy>(scalar: F, polynomial: Polynomial<F>) -> Polynomial<F> { 
    let mut result: Vec<F> = Vec::new();
    for i in 0..polynomial.len() {
    	result.push(polynomial[i] / scalar);
    }

    Polynomial::from(result)
}

pub fn padd<F: Num + Copy>(left: Polynomial<F>, right: Polynomial<F>) -> Polynomial<F> { 
    let mut result: Vec<F> = Vec::new();
	let (short, long) = match left.len() < right.len() {
		true => (left.to_vec(), right.to_vec()),
		false => (right.to_vec(), left.to_vec())
	};

    for i in 0..short.len() {
        result.push(short[i] + long[i]);
    }

	for i in short.len()..long.len() {
		result.push(long[i]);
	}

    Polynomial::from(result)
}

pub fn psub<F: Num + Copy>(left: Polynomial<F>, right: Polynomial<F>) -> Polynomial<F> { 
    let mut result: Vec<F> = Vec::new();

	if left.len() < right.len() {
		for i in 0..left.len() {
			result.push(left[i] - right[i]);
		}

		for i in left.len()..right.len() {
			result.push(F::zero() - right[i]);
		}
	}
	else {
		for i in 0..right.len() {
			result.push(left[i] - right[i]);
		}
		
		for i in right.len()..left.len() {
			result.push(left[i]);
		}
	}

    Polynomial::from(result)
}