use crate::app::event::Event;
use crate::app::model::{update_model, AppModel};
use crate::gui::gui::AppGui;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppContext {
    gui: Rc<RefCell<AppGui>>,
    model: Rc<RefCell<AppModel>>
}

impl AppContext {
    pub fn new(model: AppModel, gui: AppGui) -> AppContext {
        AppContext {
            gui: Rc::new(RefCell::new(gui)),
            model: Rc::new(RefCell::new(model))
        }
    }

    pub fn dispatch(self: &Self, event: Event) {
        update_model(&mut self.model.borrow_mut(), &event);
        self.gui.borrow_mut().render(&self.model.borrow(), self.clone());
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            gui: self.gui.clone(),
            model: self.model.clone(),
        }
    }
}
