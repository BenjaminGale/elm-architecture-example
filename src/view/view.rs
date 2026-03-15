use crate::app::context::Dispatcher;
use crate::app::model::AppModel;

pub trait LazyView<V> {
    fn render(&mut self, model: &AppModel, dispatcher: &Dispatcher);
}
