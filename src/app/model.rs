use crate::app::event::{AppEvent, CounterEvent, Event};

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

    pub fn update(self: &mut Self, event: &Event) {
        match event {
            Event::App(AppEvent::Init) => return,
            Event::Counter(ev) => self.update_count(ev),
        }
    }

    fn update_count(self: &mut Self, event: &CounterEvent) {
        match event {
            CounterEvent::Increment => self.count += 1,
            CounterEvent::Decrement => self.count -= 1,
        }
    }

    pub fn format_count(self: &Self) -> String {
        format!("Count: {}", self.count)
    }
}
