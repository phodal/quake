use crate::app::{App, MainWidget, Mode};
use quake_core::entry::EntryDefines;
use quake_core::QuakeConfig;
use serde_yaml;
use std::error::Error;
use std::fs;
use std::path::Path;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
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

    draw_main(app, f, chunks[1]);
}

fn draw_main<B>(app: &App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    match app.main_widget {
        MainWidget::Home => {
            let help_messages = vec![
                Spans::from(vec![
                    Span::raw("Press "),
                    Span::styled(":", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" into command mode, "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" back to normal mode."),
                ]),
                Spans::from(vec![
                    Span::raw("Command "),
                    Span::styled("listAll", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" list all workspace."),
                ]),
                Spans::from(vec![
                    Span::raw("Command "),
                    Span::styled("quit", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" quit quake app."),
                ]),
            ];
            frame.render_widget(
                Paragraph::new(help_messages)
                    .block(Block::default().title("Main").borders(Borders::ALL))
                    .alignment(Alignment::Center),
                area,
            )
        }
        MainWidget::EntryTypes => {
            let entry_types: Vec<ListItem> = list_entry_types().unwrap_or_default();
            let entry_types_list = List::new(entry_types)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .start_corner(Corner::TopLeft);
            frame.render_widget(entry_types_list, area);
        }
    }
}

fn list_entry_types() -> Result<Vec<ListItem<'static>>, Box<dyn Error>> {
    let config: QuakeConfig = serde_yaml::from_str(fs::read_to_string(".quake.yaml")?.as_str())?;
    let entry_defines_path = Path::new(&config.workspace).join("entries-define.yaml");
    let entry_defines: EntryDefines =
        serde_yaml::from_str(&fs::read_to_string(entry_defines_path)?)?;

    Ok(entry_defines
        .entries
        .iter()
        .map(|define| {
            let entry_type = Spans::from(vec![Span::styled(
                define.entry_type.clone(),
                Style::default().fg(Color::Yellow),
            )]);

            ListItem::new(vec![entry_type])
        })
        .collect())
}
