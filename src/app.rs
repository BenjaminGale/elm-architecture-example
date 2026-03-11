use crate::gui::update_gui_state;
use crate::GuiState;
use gtk::ApplicationWindow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppContext {
    gui: Rc<RefCell<GuiState>>,
    app: Rc<RefCell<AppState>>
}

impl AppContext {
    pub fn new(main_window: ApplicationWindow) -> AppContext {
        AppContext {
            gui: Rc::new(RefCell::new(GuiState::new(main_window))),
            app: Rc::new(RefCell::new(AppState::new()))
        }
    }

    pub fn dispatch(self: &Self, event: Event) {
        update_app_state(&mut self.app.borrow_mut(), &event);
        update_gui_state(&mut self.gui.borrow_mut(), &self.app.borrow(), self.clone());
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            gui: self.gui.clone(),
            app: self.app.clone(),
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    count: isize
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            count: 0
        }
    }

    pub fn create_count_string(self: &Self) -> String {
        format!("Count: {}", self.count)
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    Init,
    Increment,
    Decrement,
}

fn update_app_state(app_state: &mut AppState, event: &Event) {
    match event {
        Event::Init => return,
        Event::Increment => app_state.count += 1,
        Event::Decrement => app_state.count -= 1,
    }
}
