pub use std::cell::{Cell, RefCell};
pub use std::rc::Rc;
pub use std::f64::consts::PI;

pub use kurbo::{Circle, Line, Point};

pub const FREE_POINT_RADIUS: f64 = 5.0;
pub const FIXED_POINT_RADIUS: f64 = 3.0;

#[derive(Clone, Copy)]
pub enum PointType {
    Free,
    Fixed,
}

// Think it through how to implement such behaviour.
#[derive(Clone)]
pub enum Object {
    Line(Line),
    Point(Circle, PointType),
    Circle(Circle),
    Segment(Point, Point),
}

#[derive(Clone, Copy)]
pub enum ObjectName {
    Line,
    Point,
    Circle,
    Segment,
}

pub enum CursorMode {
    Move,
    Draw(ObjectName), // Think this through, not sure what to include here.
}
// Select,

pub struct GeompixEngine {
    pub cursor_mode: RefCell<CursorMode>,
    pub objects: RefCell<Vec<Object>>,
    pub buffer: Option<Object>,
}

impl Default for GeompixEngine {
    fn default() -> Self {
        GeompixEngine {
            cursor_mode: RefCell::new(CursorMode::Move),
            objects: RefCell::new(vec![]),
            buffer: None,
        }
    }
}

impl GeompixEngine {
    pub fn add_object(&self, object: Object) {
        self.objects.borrow_mut().push(object);
    }

    pub fn draw_all_objects_on_context(&self, cairo_cx: &cairo::Context) {
        for object in self.objects.borrow().iter() {
            Self::draw_object_on_context(cairo_cx, object);
        }
    }

    pub fn draw_object_on_context(
        cairo_cx: &cairo::Context,
        object: &Object,
    ) {
        match object {
            Object::Point(point, _) => {
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
