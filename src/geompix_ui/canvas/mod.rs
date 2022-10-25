use gtk4 as gtk;
use gtk::{
    gdk, gio, glib, glib::clone, graphene, prelude::*, subclass::prelude::*,
    AccessibleRole, Adjustment, DropTarget, EventControllerKey,
    EventSequenceState, GestureDrag, GestureStylus, Inhibit,
    PropagationPhase, Scrollable, ScrollablePolicy, Widget, GestureClick,
};

use cairo;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::{Application, ApplicationWindow, Button, Orientation};

use graphene::Rect;

use crate::geompix_engine::*;

mod imp {
    use super::*;

    pub struct GeompixCanvas {
        pub mouse_click_gesture: GestureClick,
        pub mouse_drag_gesture: GestureDrag,
        pub engine: Rc<RefCell<GeompixEngine>>,
    }

    impl Default for GeompixCanvas {
        fn default() -> Self {
            let mouse_click_gesture =
                GestureClick::builder().button(0).build();

            let mouse_drag_gesture = GestureDrag::builder()
                .button(0)
                .propagation_phase(PropagationPhase::Bubble)
                .build();

            let engine = Rc::new(RefCell::new(GeompixEngine::default()));

            Self {
                mouse_click_gesture,
                mouse_drag_gesture,
                engine,
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
            // class.set_css_name("button");

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
            // obj.add_controller(&self.mouse_drag_gesture);
        }
    }

    impl WidgetImpl for GeompixCanvas {
        fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
            let cairo_cx =
                snapshot.append_cairo(&Rect::new(0.0, 0.0, 300.0, 500.0));

            self.engine
                .borrow_mut()
                .draw_all_objects_on_context(&cairo_cx);
        }
    }

    impl ScrollableImpl for GeompixCanvas {}

    impl GeompixCanvas {}
}

impl GeompixCanvas {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create `GeompixCanvas`.")
    }

    pub fn draw_object(&self, object: Object) {
        self.imp().engine.borrow_mut().add_object(object);
        self.queue_draw();
    }

    pub fn draw_point(
        &self,
        x: f64,
        y: f64,
        r: f64,
        point_type: PointType,
        style: ObjectStyle,
    ) {
        let free_points = if let point_type = PointType::Free {
            vec![Point { x, y }]
        } else {
            vec![]
        };

        let point = Object {
            name: ObjectName::Point,
            core: ObjectCore::Point(Circle::new((x, y), r), point_type),
            free_points,
            style,
        };

        self.draw_object(point);
    }

    pub fn set_cursor_mode(&self, new_mode: CursorMode) {
        self.imp().engine.borrow_mut().cursor_mode = new_mode;
    }

    pub fn try_to_select_object(&self, x: f64, y: f64, id: i32) -> bool {
        let object =
            self.imp().engine.borrow().objects.get(&id).unwrap().clone();

        match object.name {
            ObjectName::Point => {
                if let ObjectCore::Point(point, point_type) = &object.core {
                    let px = point.center.x;
                    let py = point.center.y;
                    let r = point.radius;

                    // Think about the SELECTION_ACCURACY, how to calculate it right
                    if (x - px).powf(2.0) + (y - py).powf(2.0)
                        <= (r + SELECTION_ACCURACY).powf(2.0)
                    {
                        println!("point is selected at {} {}", px, py);

                        self.imp().engine.borrow_mut().select_object(id);
                        self.queue_draw();

                        // ...

                        return true;
                    }
                }
            }

            _ => {}
        };

        return false;
    }

    pub fn deselect(&self) {
        self.imp().engine.borrow_mut().deselect();
        self.queue_draw();
    }

    pub fn try_to_select(&self, x: f64, y: f64) {
        let map = self.imp().engine.borrow().objects.clone();

        for id in map.keys() {
            if self.try_to_select_object(x, y, *id) {
                return;
            }
        }

        self.deselect();

        // If there is an intersection of some curves then we draw a "fixed"
        // point
        // self.draw_point(x, y, FIXED_POINT_RADIUS, PointType::Fixed); // Think through about default
    }

    pub fn parse_click_gesture(&self, n: i32, x: f64, y: f64) {
        let cursor_mode = self.imp().engine.borrow().cursor_mode.clone();

        match cursor_mode {
            CursorMode::Move => {
                self.try_to_select(x, y);
            }

            CursorMode::Draw(object_type) => {
                match object_type {
                    ObjectName::Point => {
                        self.draw_point(
                            x,
                            y,
                            FREE_POINT_RADIUS,
                            PointType::Free,
                            ObjectStyle {},
                        ); // Think this through

                        println!("Just drew a point at ({}, {})", x, y);
                    }

                    _ => {}
                };
            }
        }
    }

    pub fn setup_input(&self) {
        self.imp().mouse_click_gesture.connect_pressed(
            clone!(@weak self as canvas =>
            move |mouse_click_gesture, n, x, y| {
                mouse_click_gesture
                    .set_state(gtk::EventSequenceState::Claimed);

                canvas.parse_click_gesture(n, x, y);
            }),
        );

        self.imp().mouse_drag_gesture.connect_drag_begin(
            clone!(@weak self as canvas =>
            move |mouse_drag_gesture, x, y| {
                mouse_drag_gesture.set_state(gtk::EventSequenceState::Claimed);
                // canvas.get
                println!("drag started");

                // canvas.parse_(n, x, y);
            }),
        );

        self.imp().mouse_drag_gesture.connect_drag_update(
            clone!(@weak self as canvas =>
            move |mouse_drag_gesture, x, y| {
                // mouse_drag_gesture.set_state(gtk::EventSequenceState::Claimed);
                println!("drag update");


                // canvas.parse_(n, x, y);
            }),
        );

        self.imp().mouse_drag_gesture.connect_drag_end(
            clone!(@weak self as canvas =>
            move |mouse_drag_gesture, x, y| {
                println!("drag end");
                // mouse_drag_gesture.set_state(gtk::EventSequenceState::Claimed);

                // canvas.parse_(n, x, y);
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
