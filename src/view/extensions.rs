use gtk::Button;
use gtk::prelude::ButtonExt;
use crate::app::context::AppContext;
use crate::app::event::Event;

pub trait ButtonExtensions {
    fn on_clicked<E, F>(&self, app_context: AppContext, event: F)
        where
            F: Fn() -> E + 'static,
            E: Into<Event>;
}

impl ButtonExtensions for Button {
    fn on_clicked<E, F>(&self, app_context: AppContext, event: F)
        where
            F: Fn() -> E + 'static,
            E: Into<Event>
    {
        self.connect_clicked(move |_| {
            app_context.dispatch(event().into());
        });
    }
}
