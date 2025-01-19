use num::Num;

pub struct Polynomial<F: Num + Copy> (Vec<F>);