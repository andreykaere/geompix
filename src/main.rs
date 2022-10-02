use gtk4 as gtk;
use gdk4 as gdk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation};
use piet;
use piet_cairo;
use graphene;

mod geompix_engine;
mod geompix_ui;

use geompix_ui::canvas::GeompixCanvas;

const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Move the point")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    // button.connect_clicked(move |button| {
    //     // Set the label to "Hello World!" after the button has been clicked on

    // });
    //
    //

    // let drawing_area = gtk::DrawingArea::new();

    // let rect = graphene::Rect::new(23.0, 23.0, 34.0, 35.0);
    // let cairo_ctx = gtk::Snapshot::new().append_cairo(&rect);

    // drawing_area.queue_draw();

    // let cairo_ctx = gdk::CairoContext.cairo_create().unwrap();
    // let piet_ctx = piet_cairo::CairoRenderContext::new(&cairo_ctx);

    let canvas = GeompixCanvas::new();

    canvas.set_size_request(300, 500);

    canvas.setup_input();

    // canvas.draw_point(23., 23.);

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&canvas);
    gtk_box.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        // .child(&canvas)
        .build();

    // Present window
    window.present();
}
