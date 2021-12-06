use crate::app::{App, MainWidget, Mode};
use std::path::{Path, PathBuf};
use std::{fs, io};
use tui::backend::Backend;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let (msg, style) = match app.mode {
        Mode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled(":", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start command mode."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Command => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quit command mode, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);
    let action = Paragraph::new(app.command.as_ref())
        .style(match app.mode {
            Mode::Normal => Style::default(),
            Mode::Command => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Action"));
    f.render_widget(action, chunks[1]);
    match app.mode {
        Mode::Normal => {}
        Mode::Command => f.set_cursor(
            chunks[1].x + app.command.width() as u16 + 1,
            chunks[1].y + 1,
        ),
    }

    draw_main(app, f, chunks[2]);
}

fn draw_main<B>(app: &App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    match app.main_widget {
        MainWidget::Home => frame.render_widget(Block::default(), area),
        MainWidget::Dirs => {
            let entry_dirs: Vec<ListItem> = list_entries_dirs()
                .unwrap_or_default()
                .iter()
                .rev()
                .map(|dir| {
                    let dir_name = Spans::from(vec![Span::styled(
                        dir.display().to_string(),
                        Style::default().fg(Color::Yellow),
                    )]);

                    ListItem::new(vec![Spans::from("-".repeat(area.width as usize)), dir_name])
                })
                .collect();
            let dir_list = List::new(entry_dirs)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .start_corner(Corner::TopLeft);
            frame.render_widget(dir_list, area);
        }
    }
}

fn list_entries_dirs() -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| path.as_deref().map(is_entries_dir).unwrap_or(false))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn is_entries_dir(path: &Path) -> bool {
    path.is_dir() && path.join("entries.csv").exists()
}
