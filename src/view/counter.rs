use crate::app::context::Dispatcher;
use crate::app::message::CounterMsg;
use crate::app::model::AppModel;
use crate::view::button_ext::ButtonDispatcher;
use gtk::prelude::BoxExt;
use gtk::{glib, Align, Button, Label};
use crate::view::view::LazyView;

pub struct CounterView {
    pub root: gtk::Box,
    label: Label,
}

impl LazyView<CounterView> for Option<CounterView> {
    fn render(&mut self, model: &AppModel, dispatcher: &Dispatcher) {
        match self {
            None => {
                let label = build_label(&model.format_count());

                let inc_button = build_button("+");
                let dec_button = build_button("-");

                inc_button.on_clicked(dispatcher, || CounterMsg::Increment);
                dec_button.on_clicked(dispatcher, || CounterMsg::Decrement);

                let container = build_layout();
                container.append(&label);
                container.append(&inc_button);
                container.append(&dec_button);

                let counter_view = CounterView {
                    root: container,
                    label
                };

                *self = Some(counter_view)
            },
            Some(view) => {
                view.label.set_label(&model.format_count());
            }
        }
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
