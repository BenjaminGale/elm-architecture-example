use gtk::Button;
use gtk::prelude::ButtonExt;
use crate::app::context::AppContext;
use crate::app::event::Event;

pub trait ButtonExtensions {
    fn on_button_clicked(&self, app_context: AppContext, event: Event);
}

impl ButtonExtensions for Button {
    fn on_button_clicked(&self, app_context: AppContext, event: Event) {
        self.connect_clicked(move |_| {
            app_context.dispatch(event);
        });
    }
}
