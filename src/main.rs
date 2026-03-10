mod app;

use app::{AppState, Event, dispatch};
use std::cell::{RefCell};
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib, Button, Label, Align};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_button<T: Into<glib::GString>>(label: T) -> Button {
    Button::builder()
        .label(label)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn build_ui(app: &Application) {
    let app_state = Rc::new(RefCell::new(AppState::new()));

    let label = Label::builder()
        .label("")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_inc = build_button("+");
    let button_dec = build_button("-");

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build();

    container.append(&label);
    container.append(&button_inc);
    container.append(&button_dec);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Elm Architecture Example")
        .default_width(350)
        .default_height(150)
        .resizable(false)
        .child(&container)
        .build();

    let label_ref = Rc::new(RefCell::new(label));

    button_inc.connect_clicked({
        let inc_state = app_state.clone();
        let lbl = label_ref.clone();
        move |_| {
            dispatch(inc_state.clone(), Event::Increment, lbl.clone());
        }
    });

    button_dec.connect_clicked({
        let dec_state = app_state.clone();
        let lbl = label_ref.clone();
        move |_| {
            dispatch(dec_state.clone(), Event::Decrement, lbl.clone());
        }
    });

    dispatch(app_state.clone(), Event::Init, label_ref.clone());

    window.present();
}
