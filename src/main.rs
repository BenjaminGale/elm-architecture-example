mod app;

use app::{AppState, Event, dispatch};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Application, ApplicationWindow, glib, Button, Label, Align};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt};
use crate::GuiState::Uninitialised;

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

fn build_layout() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build()
}

fn build_label(text: &str) -> Label {
    Label::builder()
        .label(text)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn build_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Elm Architecture Example")
        .default_width(350)
        .default_height(150)
        .resizable(false)
        .build()
}

enum GuiState {
    Uninitialised {
        main_window: ApplicationWindow,
    },
    Initialised {
        count_label: Label
    }
}

impl GuiState {
    fn new(main_window: ApplicationWindow) -> GuiState {
        Uninitialised {
            main_window
        }
    }
}

fn build_ui(app: &Application) {
    let main_window = build_main_window(&app);

    let app_state = Rc::new(RefCell::new(AppState::new()));
    let gui_state = Rc::new(RefCell::new(GuiState::new(main_window)));

    dispatch(gui_state.clone(), app_state.clone(), Event::Init);
}
