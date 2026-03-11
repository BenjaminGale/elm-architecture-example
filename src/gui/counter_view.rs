use crate::app::context::AppContext;
use crate::app::event::Event;
use crate::app::model::AppModel;
use gtk::prelude::{BoxExt, GtkWindowExt};
use gtk::{glib, Align, ApplicationWindow, Button, Label};
use crate::gui::extensions::ButtonExtensions;

pub struct CounterView {
    label: Label,
}

impl CounterView {
    pub fn new(model: &AppModel, window: &ApplicationWindow, app_context: AppContext) -> CounterView {
        let label = build_label(&model.format_count());
        let increment_button = build_button("+");
        let decrement_button = build_button("-");
        
        increment_button.on_button_clicked(app_context.clone(), Event::Increment);
        decrement_button.on_button_clicked(app_context.clone(), Event::Decrement);
        
        let container = build_layout();
        container.append(&label);
        container.append(&increment_button);
        container.append(&decrement_button);
        
        window.set_child(Some(&container));
        window.present();
        
        CounterView {
            label
        }
    }

    pub fn render(self: &mut Self, model: &AppModel) {
        self.label.set_label(&model.format_count());
    }
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
