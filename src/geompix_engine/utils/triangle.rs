use crate::{vector::*, point::*, line::*, circle::*};
// use crate::{Vector, Point, Line, Circle};


// struct Side(Point, TriangleVertex);

pub struct Triangle {
    pub A: Point,
    pub B: Point,
    pub C: Point,
}



impl Triangle {
    pub fn new<T: ToPoint>(A: T, B: T, C: T) -> Triangle {
         Triangle { A: A.to_point(), B:  B.to_point(), C: C.to_point() }
    }
    
    pub fn a(&self) -> f64 {
        (self.C - self.B).length()
    }
    
    pub fn b(&self) -> f64 {
        (self.C - self.A).length()
    }
    
    pub fn c(&self) -> f64 {
        (self.B - self.A).length()
    }
    
    fn is_vertex(&self, point: Point) -> bool {
        (point == self.A) || (point == self.B) || (point == self.B)
    }
    
    fn is_degenerate(&self) -> bool {
        colinear_points(self.A, self.B, self.C)
    }
    
    fn process_vertex(&self, point: Point) -> Option<()> {
        if !self.is_vertex(point) {
            println!("ERROR! Specified point is not a vertex of this triangle!");
            return None;
        }
        
        Some(())
    }

    fn process_triangle(&self) -> Option<()> {
        if self.is_degenerate() {
            println!("WARNING! This triangle is generate!");
            return None;
        }

        Some(())
    }

    fn process_triangle_and_vertex(&self, point: Point) -> Option<()> {
        self.process_vertex(point)?;
        self.process_triangle_and_vertex(point)
    }

    
    // pub fn circumcenter(&self) -> Option<Point> {
    //     self.process_triangle()?;
        
    //     let bis1 = bisector(self.A, self.B);
    //     let bis2 = bisector(self.C, self.B);

    //     bis1.intersection_point(bis2) 
    // }
    
    //pub fn incenter(&self) -> Option<Point> {
    //    self.process_triangle()?;

    //    let bis1 = self.internal_bisector(self.A);
    //    let bis2 = self.internal_bisector(self.B);

    //    bis1.intersection_point(bis2)
    //}

    
    //pub fn excenter(&self, vertex: Point) -> Option<Point> {
    //    self.process_triangle_and_vertex()?;

    //    let bis1 = self.internal_bisector(vertex);

    //    if vertex == self.A {
    //    let bis2 = self.internal_bisector(self.B);

    //    bis1.intersection_point(bis2)

        
    //}

    //pub fn circumcircle(&self) -> Option<Circle> {
    //    self.process_triangle()?;
        
    //}
    
    //pub fn incircle(&self) -> Option<Circle> {
    //    self.process_triangle()?;
        
    //}
    
    //pub fn excircle(&self, vertex: Point) -> Option<Circle> {
    //    self.process_triangle_and_vertex()?;
        
    //}

    //pub fn orthocenter(&self) -> Option<Point> {
    //    self.process_triangle()?;

    //}

    //pub fn barycenter(&self) -> Option<Point> {
    //    self.process_triangle()?;

    //    let median1 = self.median(self.A);
    //    let median2 = self.median(self.B);
        
    //    median1.intersection_point(median2)
        
    //}

    //pub fn median(&self, vertex: Point) -> Option<Line> {
    //    self.process_triangle_and_vertex(vertex)?;

    //    //
    //}

    
    //pub fn symmedian(&self, vertex: Point) -> Option<Line> {
    //    self.process_triangle_and_vertex(vertex)?;

    //}

    //pub fn internal_bisector(&self, vertex: Point) -> Option<Line> {
    //    self.process_triangle_and_vertex(vertex)?;

    //    //
    //}
    
    //pub fn external_bisector(&self, vertex: Point) -> Option<Line> {
    //    self.process_triangle_and_vertex(vertex)?;

    //    //
    //}
    
    //pub fn altitude(&self, vertex: Point) -> Option<Line> {
    //    self.process_triangle_and_vertex(vertex)?;

    //    //
    //}

    
    pub fn perimeter(&self) -> f64 {
        self.a() + self.b() + self.c()
    }
    
    pub fn semiperimeter(&self) -> f64 {
        self.perimeter() /  2.
    }
    
    pub fn area(&self) -> f64 {
        let p = self.semiperimeter();
        let a = self.a();
        let b = self.b();
        let c = self.c();
        
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }
    
}
