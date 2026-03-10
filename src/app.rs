use std::cell::RefCell;
use std::rc::Rc;
use gtk::Label;

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
}

#[derive(Copy, Clone)]
pub enum Event {
    Init,
    Increment,
    Decrement,
}

pub fn dispatch(app_state: Rc<RefCell<AppState>>, event: Event, label: Rc<RefCell<Label>>) {
    update(&mut app_state.borrow_mut(), &event);
    render(&app_state.borrow(), label.clone());
}

fn update(app_state: &mut AppState, event: &Event) {
    match event {
        Event::Init => return,
        Event::Increment => app_state.count += 1,
        Event::Decrement => app_state.count -= 1,
    }
}

fn render(app_state: &AppState, label: Rc<RefCell<Label>>) {
    label.borrow_mut().set_label(format!("Count: {:?}", app_state.count).as_str());
}
