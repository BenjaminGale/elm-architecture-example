
#[derive(Clone)]
pub enum Event {
    App(AppEvent),
    Counter(CounterEvent),
}

impl From<AppEvent> for Event {
    fn from(value: AppEvent) -> Self {
        Event::App(value)
    }
}

#[derive(Clone)]
pub enum AppEvent {
    Init,
}

#[derive(Clone)]
pub enum CounterEvent {
    Increment,
    Decrement,
}

impl From<CounterEvent> for Event {
    fn from(value: CounterEvent) -> Self {
        Event::Counter(value)
    }
}
