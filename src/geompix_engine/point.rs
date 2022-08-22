use std::ops::{Sub, Neg, Add, Mul, Div};


enum PointState {
    free,
    fixed,
}


pub struct Point {
    pub x: f64,
    pub y: f64,
    belongs_to: Vec<Box<dyn Path>>,
    state: PointState,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
            belongs_to: Vec::new(),
            state: PointState::free,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}


pub trait ToPoint {
    fn to_point(&self) -> Point;
}

impl ToPoint for Point {
    fn to_point(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

impl ToPoint for (f64, f64) {
    fn to_point(&self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}



pub fn colinear_points(A: Point, B: Point, C: Point) -> bool {
    true
}
