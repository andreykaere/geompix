use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}


impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    end: Point,
}

impl Vector {
    fn new(A: Point, B: Point) -> Vector {
        Vector {
            start: A,
            end: B,
        }
    }
    
    fn length(&self) -> f64 {
        (self.end.x.pow(2) + self.end.y.pow(2)).sqrt()
    }
}


impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            end: self.end + other.end
        }
    }
}

