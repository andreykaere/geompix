

// equation Ax + By + C = 0
struct LineEquation {
    A: f64,
    B: f64,
    C: f64,
}

pub struct Line {
    equation: LineEquation
}


impl Line {
    fn new(A: f64, B: f64, C: f64) {
        Line {
            equation: LineEquation { A, B, C }
        }
    }

    fn new_two_points(A: Point, B: Point) {
        
    }

    // fn new(Vector 
    //

    // If lines coincide, we still return None
    fn intersection_point(&self, other: Line) -> Option<Point> {
        let A = self.A;
        let B = self.B;
        let A1 = other.A;
        let B1 = other.B;

        if A / A1 == B / B1 {
            return None;
        }

        // solve the system


    }

}
