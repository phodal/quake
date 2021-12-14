use crate::app::{App, Mode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());
    f.render_widget(app.cmd_line.clone(), chunks[0]);
    f.render_widget(app.main_widget.clone(), chunks[1]);

    match app.mode {
        Mode::Normal => {}
        Mode::Command => f.set_cursor(
            chunks[0].x + app.cmd_line.message.width() as u16 + 1,
            chunks[0].y + 1,
        ),
        Mode::Insert => {
            f.set_cursor(
                chunks[1].x + app.main_widget.get_input().width() as u16 + 1,
                chunks[1].y + 1,
            );
        }
    }
}
