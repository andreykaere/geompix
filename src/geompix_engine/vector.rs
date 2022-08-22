use std::ops::{Sub, Neg, Add, Mul, Div};
use crate::point::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn new(A: Point, B: Point) -> Vector {
        Vector {
            x: B.x - A.x,
            y: B.y - A.y,
        }
    }
    
    // pub fn new<T: ToPoint>(A: T) -> Vector {
    //     Vector { end: A.to_point() }
    // }

    
    pub fn length(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }


    pub fn colinear(&self, u: Vector) -> bool {
        self.x / u.x == self.y / u.y
    }
}


impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}


impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}



impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector { x: rhs * self.x, y: rhs * self.y }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector { x: self.x / rhs, y: rhs * self.y / rhs }
    }
}

impl Div for Vector {
    type Output = Option<f64>;

    fn div(self, rhs: Self) -> Self::Output {
        // Vector::new(x: self.x() / rhs, y: rhs * self.y() / rhs)
        if self.colinear(rhs) {
            return Some(self.x / rhs.x);
        }

        None
    }
}

