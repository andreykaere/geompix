// pub use std::cell::{Cell, RefCell};
// pub use std::rc::Rc;
pub use std::f64::consts::PI;
pub use std::collections::HashMap;

pub use kurbo::{Circle, Line, Point};

pub const FREE_POINT_RADIUS: f64 = 5.0;
pub const FIXED_POINT_RADIUS: f64 = 3.0;
pub const SELECTION_ACCURACY: f64 = 7.0;

#[derive(Clone)]
pub enum PointType {
    Free,
    Fixed,
    OnObject(i32),
}

// Think it through how to implement such behaviour.
#[derive(Clone)]
pub enum ObjectCore {
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

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

// alpha: f64,
#[derive(Clone, Copy)]
pub struct ObjectStyle {
    // pub color: Color,
}

#[derive(Clone)]
pub struct Object {
    pub name: ObjectName,
    pub core: ObjectCore,
    pub free_points: Vec<Point>,
    pub style: ObjectStyle,
}

impl Object {
    // fn select(&self) {
    //     match self.name {
    //         ObjectName::Point => {}
    //         ObjectName::Line => {}
    //         ObjectName::Segment => {}
    //         ObjectName::Circle => {}
    //     }
    // }

    // fn deselect(&self) {
    //     match self.name {
    //         ObjectName::Point => {}
    //         ObjectName::Line => {}
    //         ObjectName::Segment => {}
    //         ObjectName::Circle => {}
    //     }
    // }
    // fn to_object(
    //     core: ObjectCore,
    //     color: Color,
    //     style: Objectstyle,
    // ) -> Object {
    //     match core {
    //         ObjectCore::Point(circle, point_type) => {
    //             return Object {
    //                 name: ObjectName::Point,
    //                 core,
    //                 color,

    //             }
    //         }
    //     }
    // }
}

// fn circle_by_three_points(
//     p1: Point,
//     p2: Point,
//     p3: Point,
//     style: ObjectStyle,
// ) -> Object {
//     let circle = Circle::new(); // Calculate actual circle

//     let free_points = [p1, p2, p3];
//     let core = Circle(circle);

//     Object {
//         name: Circle,
//         free_points,
//         core,
//         style,
//     }
// }

#[derive(Clone)]
pub enum CursorMode {
    Move,
    Draw(ObjectName), // Think this through, not sure what to include here.
}
// Select,

pub struct GeompixEngine {
    pub cursor_mode: CursorMode,
    pub objects: HashMap<i32, Object>,
    pub buffer: Option<i32>,
    pub selected: Option<i32>,
}

impl Default for GeompixEngine {
    fn default() -> Self {
        GeompixEngine {
            cursor_mode: CursorMode::Move,
            objects: HashMap::new(),
            buffer: None,
            selected: None,
        }
    }
}

impl GeompixEngine {
    pub fn select_object(&mut self, id: i32) {
        self.selected = Some(id);
        // self.selected.replace(Some(object.clone())); // Think about replacing clone with reference
    }

    pub fn deselect(&mut self) {
        self.selected = None;
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.insert((self.objects.len() + 1) as i32, object);
    }

    pub fn draw_all_objects_on_context(&self, cairo_cx: &cairo::Context) {
        let map = &self.objects;
        let mut keys: Vec<&i32> = map.keys().collect();
        keys.sort();

        for id in keys.iter() {
            self.draw_object_on_context(cairo_cx, **id);
        }

        // self.draw_selection_on_context(cairo_cx);
    }

    // pub fn draw_selection_on_context(&self, cairo_cx: &cairo::Context) {
    //     if let Some(object) = self.selected.borrow().as_ref() {
    //         match object.name {
    //             ObjectName::Point => {}
    //             _ => {}
    //         }
    //     }

    //     // match *object.name {

    //     // }
    // }

    pub fn draw_object_on_context(&self, cairo_cx: &cairo::Context, id: i32) {
        let object = self.objects.get(&id).unwrap();

        match object.core {
            ObjectCore::Point(point, _) => {
                let xc = point.center.x;
                let yc = point.center.y;

                cairo_cx.arc(xc, yc, point.radius, 0.0, PI * 2.0);
                cairo_cx.fill().expect("Invalid cairo surface state");
            }

            ObjectCore::Line(line) => {}

            ObjectCore::Segment(p1, p2) => {}

            ObjectCore::Circle(circle) => {
                let xc = circle.center.x;
                let yc = circle.center.y;

                cairo_cx.arc(xc, yc, circle.radius, 0.0, PI * 2.0);
                cairo_cx.stroke().expect("Invalid cairo surface state");
            }
        }
    }
}
