pub struct App {
    pub mode: Mode,
    pub command: String,
    pub main_widget: MainWidget,
}

impl App {
    pub fn new() -> App {
        App::default()
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            mode: Mode::Normal,
            command: "".to_string(),
            main_widget: MainWidget::Home,
        }
    }
}

pub enum Mode {
    Command,
    Normal,
}

pub enum MainWidget {
    Home,
    Dirs,
}
