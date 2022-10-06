use gtk4 as gtk;
use gtk::{
    gdk, gio, glib, glib::clone, graphene, prelude::*, subclass::prelude::*,
    AccessibleRole, Adjustment, DropTarget, EventControllerKey,
    EventSequenceState, GestureDrag, GestureStylus, Inhibit,
    PropagationPhase, Scrollable, ScrollablePolicy, Widget, GestureClick,
};

use cairo;

use gtk::{Application, ApplicationWindow, Button, Orientation};

use graphene::Rect;

use crate::geompix_engine::*;

mod imp {
    use super::*;

    pub struct GeompixCanvas {
        pub mouse_click_gesture: GestureClick,
        pub engine: GeompixEngine,
    }

    impl Default for GeompixCanvas {
        fn default() -> Self {
            let mouse_click_gesture =
                GestureClick::builder().button(0).build();

            let engine = GeompixEngine::default();

            Self {
                mouse_click_gesture,
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
        }
    }

    impl WidgetImpl for GeompixCanvas {
        fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
            let cairo_cx =
                snapshot.append_cairo(&Rect::new(0.0, 0.0, 300.0, 500.0));

            self.engine.draw_all_objects_on_context(&cairo_cx);
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
        self.imp().engine.add_object(object);
        self.queue_draw();
    }

    pub fn draw_point(&self, x: f64, y: f64, r: f64) {
        self.draw_object(Object::Point(Circle::new((x, y), r)))
    }

    pub fn set_cursor_mode(&self, new_mode: CursorMode) {
        self.imp().engine.cursor_mode.replace(new_mode);
    }

    pub fn try_to_focus(&self) {}

    pub fn parse_click_gesture(&self, n: i32, x: f64, y: f64) {
        match *self.imp().engine.cursor_mode.borrow() {
            CursorMode::Move => {
                self.try_to_focus();
            }

            CursorMode::Draw(object_type) => {
                match object_type {
                    ObjectName::Point => {
                        self.draw_point(x, y, 5.0);
                        println!("Just drew a point at ({}, {})", x, y);
                    }

                    _ => {}
                };
                // if let Some(buffer) = &self.imp().engine.buffer {
                //     match buffer {
                //         Object::Point(point) => {
                //             println!("Just drew a point at ({}, {})", x, y);
                //         }

                //         _ => {}
                //     };
                // };
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
