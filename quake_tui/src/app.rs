pub struct App {
    pub mode: Mode,
    pub command: String,
    pub main_widget: MainWidget,
    pub state: AppState,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn running(&self) -> bool {
        self.state.running
    }

    pub fn shutdown(&mut self) {
        self.state.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            mode: Mode::Normal,
            command: "".to_string(),
            main_widget: MainWidget::Home,
            state: AppState::default(),
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

pub struct AppState {
    running: bool,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState { running: true }
    }
}
