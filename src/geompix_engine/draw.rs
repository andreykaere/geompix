use crate::triangle::Triangle;
use crate::point::Point;

pub trait Draw {
    fn draw(&self);
}

pub trait Move {
    fn move(self, new: Self);
    // fn redraw(&self);
}


impl Draw for Triangle {
    fn draw(&self) {
        println!("I've just drawn a triangle with vertices A: x: {}, y: {}, B: x: {}, y:{}, C: x: {}, y: {}", self.A.x, self.A.y, self.B.x,
        self.B.y, self.C.x, self.C.y);
    }
}


impl Move for Point {
    fn move(self, new: Self) {
        self.x = new.x;
        self.y = new.y;
    }
}
