use crate::app::context::AppContext;
use crate::app::event::AppEvent;
use crate::app::model::AppModel;
use crate::gui::gui::AppGui;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{glib, Application};

mod app;
mod gui;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(on_activate);
    app.run()
}

fn on_activate(app: &Application) {
    let model = AppModel::new();
    let gui = AppGui::new(app);
    let app_context = AppContext::new(model, gui);

    app_context.dispatch(AppEvent::Init);
}
