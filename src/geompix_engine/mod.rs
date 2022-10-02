pub use std::cell::{Cell, RefCell};
pub use std::rc::Rc;
pub use std::f64::consts::PI;

// pub mod circle;
// pub mod draw;
// pub mod line;
// pub mod point;
// pub mod triangle;
// pub mod utils;
// pub mod vector;

// pub use crate::{line::*, circle::*, point::*, vector::*, utils::*};

pub use kurbo::{Circle, Line, Point};

// Think it through how to implement such behaviour.
#[derive(Clone, Copy)]
pub enum Object {
    Line(Line),
    Point(Circle),
    Circle(Circle),
    Segment(Point, Point),
}
pub enum CursorMode {
    Move,
    Draw(Object), // Think this through, not sure what to include here.
}

pub struct GeompixEngine {
    pub cursor_mode: Cell<CursorMode>,
    pub objects: RefCell<Vec<Object>>,
}

impl Default for GeompixEngine {
    fn default() -> Self {
        GeompixEngine {
            cursor_mode: Cell::new(CursorMode::Move),
            objects: RefCell::new(vec![]),
        }
    }
}

impl GeompixEngine {
    pub fn add_object(&self, object: Object) {
        self.objects.borrow_mut().push(object);
    }

    pub fn draw_all_objects_on_context(&self, cairo_cx: &cairo::Context) {
        for object in self.objects.borrow().iter() {
            Self::draw_object_on_context(&cairo_cx, object);
        }
    }

    pub fn draw_object_on_context(
        cairo_cx: &cairo::Context,
        object: &Object,
    ) {
        match *object {
            Object::Point(point) => {
                let xc = point.center.x;
                let yc = point.center.y;

                cairo_cx.arc(xc, yc, point.radius, 0.0, PI * 2.0);
                cairo_cx.fill().expect("Invalid cairo surface state");
            }

            Object::Line(line) => {}

            Object::Segment(p1, p2) => {}

            Object::Circle(circle) => {
                let xc = circle.center.x;
                let yc = circle.center.y;

                cairo_cx.arc(xc, yc, circle.radius, 0.0, PI * 2.0);
                cairo_cx.stroke().expect("Invalid cairo surface state");
            }
        }
    }
}
