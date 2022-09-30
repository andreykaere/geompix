use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::f64::consts::PI;

use gtk4 as gtk;
use gtk::{
    gdk, gio, glib, glib::clone, graphene, prelude::*, subclass::prelude::*,
    AccessibleRole, Adjustment, DropTarget, EventControllerKey,
    EventSequenceState, GestureDrag, GestureStylus, Inhibit,
    PropagationPhase, Scrollable, ScrollablePolicy, Widget, GestureClick,
};

use gtk::{Application, ApplicationWindow, Button, Orientation};

use graphene::Rect;

use kurbo::Circle;

pub enum CursorMode {
    Move,
    Point,
    Line,
}

mod imp {
    use super::*;

    pub struct GeompixCanvas {
        pub cursor_mode: Cell<CursorMode>,
        pub mouse_click_gesture: GestureClick,
        pub points: RefCell<Vec<Circle>>,
    }

    impl Default for GeompixCanvas {
        fn default() -> Self {
            let mouse_click_gesture =
                GestureClick::builder().button(0).build();

            Self {
                // change later default to `Move`
                cursor_mode: Cell::new(CursorMode::Point),
                mouse_click_gesture,
                points: RefCell::new(vec![]),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GeompixCanvas {
        const NAME: &'static str = "GeompixCanvas";
        type Type = super::GeompixCanvas;
        type ParentType = gtk::Widget;
        // type Interfaces = (Scrollable, );

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk::BinLayout>();

            // Make it look like a GTK button.
            // klass.set_css_name("button");

            // Make it appear as a button to accessibility tools.
            klass.set_accessible_role(gtk::AccessibleRole::Widget);
        }

        fn new() -> Self {
            Self::default()
        }
    }

    impl ObjectImpl for GeompixCanvas {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Connect a gesture to handle clicks.
            // let gesture = gtk::GestureClick::new();
            // gesture.connect_released(|gesture, _, _, _| {
            //     gesture.set_state(gtk::EventSequenceState::Claimed);
            //     println!("Button pressed!");
            // });
            // obj.add_controller(&gesture);

            obj.add_controller(&self.mouse_click_gesture);
        }
    }

    impl WidgetImpl for GeompixCanvas {
        fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
            let cairo_cx =
                snapshot.append_cairo(&Rect::new(0.0, 0.0, 500.0, 300.0));

            for point in self.points.borrow().iter() {
                let xc = point.center.x;
                let yc = point.center.y;

                cairo_cx.arc(xc, yc, point.radius, 0.0, PI * 2.0);
                cairo_cx.fill().expect("Invalid cairo surface state");
            }
        }
    }

    // impl ScrollableImpl for GeompixCanvas {}
}

impl GeompixCanvas {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create `GeompixCanvas`.")
    }

    pub fn draw_point(&self, x: f64, y: f64, radius: f64) {
        // Something weird is happening here with the arguments of the clouser
        self.imp().points.replace_with(|old| {
            old.push(Circle::new((x, y), radius));
            old.to_vec()
        });

        self.queue_draw();
    }

    // pub fn draw_line(&self, p1: Point, p2: Point) {
    //     self.imp().lines

    //     self.queue_draw();
    // }

    pub fn setup_input(&self) {
        self.imp().mouse_click_gesture.connect_pressed(
            clone!(@weak self as canvas =>
            move |mouse_click_gesture, n, x, y| {
                mouse_click_gesture
                    .set_state(gtk::EventSequenceState::Claimed);

                canvas.draw_point(x, y, 5.0);

                println!("Just drew a point at ({}, {})", x, y);
            }),
        );
    }

    // pub fn init(&self, appwindow: &ApplicationWindow) {
    //     self.setup_input(appwindow);
    // }

    // pub fn setup_input(&self, appwindow: &ApplicationWindow) {
    //     self.imp().mouse_click_gesture.connect_pressed( move |mouse_click_gesture, n, x, y| {
    //         println!("foo");
    //     });
    // }
}

glib::wrapper! {
    pub struct GeompixCanvas(ObjectSubclass<imp::GeompixCanvas>)
        @extends gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
        // @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Scrollable;
}
