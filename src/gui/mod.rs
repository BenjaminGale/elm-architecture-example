
pub mod counter_view;
pub mod extensions;

pub mod gui {
    use crate::app::context::AppContext;
    use crate::app::model::AppModel;
    use crate::gui::counter_view::CounterView;
    use gtk::{Application, ApplicationWindow};

    pub struct AppGui {
        main_window: ApplicationWindow,
        counter: Option<CounterView>
    }

    impl AppGui {
        pub fn new(app: &Application) -> AppGui {
            AppGui {
                main_window: build_main_window(app),
                counter: None
            }
        }

        pub fn render(self: &mut Self, model: &AppModel, app_context: AppContext) {
            match &mut self.counter {
                None => self.counter = Some(CounterView::new(model, &self.main_window, app_context.clone())),
                Some(counter)  => counter.render(model)
            }
        }
    }

    fn build_main_window(app: &Application) -> ApplicationWindow {
        ApplicationWindow::builder()
            .application(app)
            .title("Elm Architecture Example")
            .default_width(350)
            .default_height(150)
            .resizable(false)
            .build()
    }
}
