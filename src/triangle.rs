
struct TriangleVertex {
    x: f64,
    y: f64,
}

// struct Side(TriangleVertex, TriangleVertex);

pub struct Triangle {
    A: TriangleVertex,
    B: TriangleVertex,
    C: TriangleVertex,
}


impl Triangle {
    fn new(A: Point, B: Point, C: Point) -> Triangle {
        Triangle { A.to_vertex(), B.to_vertex(), C.to_vertex() }
    }

    fn new(A: (f64, f64), 
           B: (f64, f64),
           C: (f64, f64),
    ) -> Triangle 
    {
        
        let A = TriangleVertex { x: A.0, y: A.1 };
        let B = TriangleVertex { x: B.0, y: B.1 };
        let C = TriangleVertex { x: C.0, y: C.1 };

        Triangle { A, B, C }
    }

    fn A(&self) -> TriangleVertex {
        self.A
    }
    
    fn B(&self) -> TriangleVertex {
        self.B
    }
    
    fn C(&self) -> TriangleVertex {
        self.C
    }
    
    fn a(&self) -> f64 {
        (self.C - self.B).abs()
    }
    
    fn b(&self) -> f64 {
        (self.C - self.A).abs()
    }
    
    fn c(&self) -> f64 {
        (self.B - self.A).abs()
    }
    
    
    fn circumcenter(&self) -> Point {
        
    }
    
    fn incenter(&self) -> Point {
        
    }
    
    fn excenter(&self, vertex: TriangleVertex) -> Point {
        
    }

    fn circumcircle(&self) -> Circle {
        
    }
    
    fn incircle(&self) -> Circle {
        
    }
    
    fn excircle(&self, vertex: TriangleVertex) -> Circle {
        
    }

    fn orthocenter(&self) -> Point {

    }

    fn barycenter(&self) -> Point {
        let median1 = self.median(self.A);
        let median2 = self.median(self.B);
        
        median1.intersectionpoint(median2)
        
    }

    fn median(&self, vertex: TriangleVertex) -> Line {

    }
    
    fn symmedian(&self, vertex: TriangleVertex) -> Line {

    }

    fn internal_bisector(&self, vertex: TriangleVertex) -> Line {
    }
    
    fn external_bisector(&self, vertex: TriangleVertex) -> Line {
    }
    
    fn altitude(&self, vertex: TriangleVertex) -> Line {
    }

    
    fn perimeter(&self) -> f64 {
    }
    
    fn semiperimeter(&self) -> f64 {
        self.perimeter /  2
    }
    
    fn area(&self) -> f64 {
        let p = self.semiperimeter();
        let a = self.a();
        let b = self.b();
        let c = self.c();
        
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }

}
