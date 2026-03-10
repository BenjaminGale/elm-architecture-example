use crate::gui::{build_button, build_label, build_layout, ButtonExtensions};
use crate::GuiState;
use gtk::prelude::{BoxExt, GtkWindowExt};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::ApplicationWindow;

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
        update_gui_state(self.clone());
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

fn update_gui_state(context: AppContext) {
    let mut gui = context.gui.borrow_mut();
    let state = context.app.borrow();

    match &mut *gui {
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
