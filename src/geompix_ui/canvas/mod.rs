use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4 as gtk;
use gtk::{
    gdk, gio, glib, glib::clone, graphene, prelude::*, subclass::prelude::*, 
    AccessibleRole, Adjustment, DropTarget, EventControllerKey, 
    EventSequenceState, GestureDrag, GestureStylus, Inhibit, PropagationPhase, 
    Scrollable, ScrollablePolicy, Widget, GestureClick,
};

use gtk::{Application, ApplicationWindow, Button, Orientation};




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
    }

    impl Default for GeompixCanvas {
        fn default() -> Self {
            let mouse_click_gesture = GestureClick::builder()
                .button(0)
                .build();

            Self {
                // change later default to `Move`
                cursor_mode: Cell::new(CursorMode::Point),
                mouse_click_gesture,
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
            let gesture = gtk::GestureClick::new();
            gesture.connect_released(|gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                println!("Button pressed!");
            });
            obj.add_controller(&gesture);

            // obj.add_controller(&self.mouse_click_gesture);
        }
    }

    
    impl WidgetImpl for GeompixCanvas {}

    
    // impl ScrollableImpl for GeompixCanvas {}
}


impl GeompixCanvas {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create `GeompixCanvas`.")
    }

    // pub fn draw_point(&self, x: f64, y: f64) {

    // }
    
    // pub fn setup_input(&self) {
    //     self.imp().mouse_click_gesture.connect_pressed( move | mouse_click_gesture, n, x, y | {
    //         println!("foo");
    //     });
    // }

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


