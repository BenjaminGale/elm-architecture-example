mod app;
mod gui;

use crate::app::AppContext;
use crate::gui::{build_main_window, GuiState};
use app::Event;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let main_window = build_main_window(&app);
    let context = AppContext::new(main_window);

    context.dispatch(Event::Init);
}
