use crate::{vector::*, point::*};
// use self::point::*;

// equation Ax + By + C = 0
#[derive(Debug, Copy, Clone)]
struct LineEquation {
    A: f64,
    B: f64,
    C: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    equation: LineEquation
}


impl Line {
    pub fn new_eq(A: f64, B: f64, C: f64) -> Line {
        Line {
            equation: LineEquation { A, B, C }
        }
    }

    pub fn new_two_points(A: Point, B: Point) -> Line {
        
    }

    pub fn new_vec(vec: Vector, A: Point) -> Line {
    }
    

    // If lines coincide, we still return None
    pub fn intersection_point(&self, other: Line) -> Option<Point> {
        let A = self.equation.A;
        let B = self.equation.B;
        let A1 = other.equation.A;
        let B1 = other.equation.B;

        if A / A1 == B / B1 {
            return None;
        }

        // solve the system


    }



}
