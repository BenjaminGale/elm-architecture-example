use crate::app::context::Dispatcher;
use crate::app::message::Msg;
use gtk::prelude::ButtonExt;
use gtk::Button;

pub trait ButtonDispatcher {
    fn on_clicked<E, F>(&self, dispatcher: &Dispatcher, event: F)
    where
        F: Fn() -> E + 'static,
        E: Into<Msg>;
}

impl ButtonDispatcher for Button {
    fn on_clicked<E, F>(&self, dispatcher: &Dispatcher, event: F)
        where
            F: Fn() -> E + 'static,
            E: Into<Msg>
    {
        let d = dispatcher.clone();
        self.connect_clicked(move |_| {
            d.dispatch(event().into());
        });
    }
}
