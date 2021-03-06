use crate::app::{App, Mode};
use crate::widgets::CmdLine;
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
    f.render_stateful_widget(CmdLine, chunks[0], &mut app.state);
    f.render_stateful_widget(app.main_widget.clone(), chunks[1], &mut app.state);

    match app.state.mode {
        Mode::Normal => {}
        Mode::Command => f.set_cursor(
            chunks[0].x + app.state.message.width() as u16 + 1,
            chunks[0].y + 1,
        ),
        Mode::Insert => {
            let input_lines: Vec<&str> = app.main_widget.get_input().split('\n').collect();
            f.set_cursor(
                chunks[1].x + input_lines.last().unwrap_or(&"").width() as u16 + 1,
                chunks[1].y + input_lines.len() as u16,
            );
        }
    }
}
