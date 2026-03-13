
#[derive(Clone)]
pub enum Msg {
    App(AppMsg),
    Counter(CounterMsg),
}

impl From<AppMsg> for Msg {
    fn from(value: AppMsg) -> Self {
        Msg::App(value)
    }
}

#[derive(Clone)]
pub enum AppMsg {
    Init,
}

#[derive(Clone)]
pub enum CounterMsg {
    Increment,
    Decrement,
}

impl From<CounterMsg> for Msg {
    fn from(value: CounterMsg) -> Self {
        Msg::Counter(value)
    }
}
