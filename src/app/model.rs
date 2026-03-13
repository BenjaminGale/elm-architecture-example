use crate::app::message::{AppMsg, CounterMsg, Msg};

#[derive(Debug)]
pub struct AppModel {
    pub count: isize
}

impl AppModel {
    pub fn new() -> AppModel {
        AppModel {
            count: 0
        }
    }

    pub fn update(self: &mut Self, event: &Msg) {
        match event {
            Msg::App(AppMsg::Init) => return,
            Msg::Counter(ev) => self.update_count(ev),
        }
    }

    fn update_count(self: &mut Self, event: &CounterMsg) {
        match event {
            CounterMsg::Increment => self.count += 1,
            CounterMsg::Decrement => self.count -= 1,
        }
    }

    pub fn format_count(self: &Self) -> String {
        format!("Count: {}", self.count)
    }
}
