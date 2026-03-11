use crate::app::{AppContext, AppState, Event};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use gtk::{glib, Align, Application, ApplicationWindow, Button, Label};

pub enum GuiState {
    Uninitialised {
        main_window: ApplicationWindow,
    },
    Initialised {
        count_label: Label
    }
}

impl GuiState {
    pub fn new(main_window: ApplicationWindow) -> GuiState {
        GuiState::Uninitialised {
            main_window
        }
    }
}

pub fn build_button<T: Into<glib::GString>>(label: T) -> Button {
    Button::builder()
        .label(label)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_layout() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build()
}

pub fn build_label(text: &str) -> Label {
    Label::builder()
        .label(text)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Elm Architecture Example")
        .default_width(350)
        .default_height(150)
        .resizable(false)
        .build()
}

pub trait ButtonExtensions {
    fn on_button_clicked(&self, context: AppContext, event: Event);
}

impl ButtonExtensions for Button {
    fn on_button_clicked(&self, context: AppContext, event: Event) {
        self.connect_clicked(move |_| {
            context.dispatch(event);
        });
    }
}

pub fn update_gui_state(gui: &mut GuiState, state: &AppState, context: AppContext,) {
    match gui {
        GuiState::Uninitialised { main_window } => {
            let label = build_label(&state.create_count_string());
            let button_inc = build_button("+");
            let button_dec = build_button("-");

            let container = build_layout();
            container.append(&label);
            container.append(&button_inc);
            container.append(&button_dec);

            button_inc.on_button_clicked(context.clone(), Event::Increment);
            button_dec.on_button_clicked(context.clone(), Event::Decrement);

            main_window.set_child(Some(&container));
            main_window.present();

            *gui = GuiState::Initialised {
                count_label: label,
            }
        }
        GuiState::Initialised { count_label, .. } => {
            count_label.set_label(&state.create_count_string());
        }
    }
}
