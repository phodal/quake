use crate::app::{App, Mode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());
    let command_bar = Paragraph::new(app.command.as_ref())
        .style(match app.mode {
            Mode::Normal => Style::default(),
            Mode::Command => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Action"));
    f.render_widget(command_bar, chunks[0]);
    match app.mode {
        Mode::Normal => {}
        Mode::Command => f.set_cursor(
            chunks[0].x + app.command.width() as u16 + 1,
            chunks[0].y + 1,
        ),
    }

    f.render_widget(app.main_widget.clone(), chunks[1]);
}
