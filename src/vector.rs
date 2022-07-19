use std::ops::{Sub, Neg, Add, Mul, Div};
use crate::point::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    end: Point,
}

impl Vector {
    pub fn new_two_points(A: Point, B: Point) -> Vector {
        Vector {
            end: B - A,
        }
    }

    pub fn new_coords(x: f64, y: f64) -> Vector {
        Vector { end: Point { x, y } }
    }
    
    pub fn new<T: ToPoint>(A: T) -> Vector {
        Vector { end: A.to_point() }
    }

    
    pub fn length(&self) -> f64 {
        (self.x().powf(2.) + self.y().powf(2.)).sqrt()
    }

    pub fn x(&self) -> f64 {
        self.end.x()
    }
    
    pub fn y(&self) -> f64 {
        self.end.y()
    }

    pub fn colinear(&self, u: Vector) -> bool {
        self.x() / u.x() == self.y() / u.y()
    }
}


impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            end: self.end + other.end,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            end: -self.end, 
        }
    }
}


impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            end: self.end - other.end, 
        }
    }
}



impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new_coords(x: rhs * self.x(), y: rhs * self.y())
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new_coords(x: self.x() / rhs, y: rhs * self.y() / rhs)
    }
}

impl Div for Vector {
    type Output = Option<f64>;

    fn div(self, rhs: Self) -> Self::Output {
        // Vector::new(x: self.x() / rhs, y: rhs * self.y() / rhs)
        if colinear(self, rhs) {
            Some(self.x() / rhs.x())
        }

        None
    }
}

