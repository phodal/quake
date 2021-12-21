use std::error::Error;
use std::fs;
use std::path::Path;

use quake_core::entry::entry_paths::EntryPaths;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Corner, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Widget};

use quake_core::entry::EntryDefines;
use quake_core::QuakeConfig;

#[derive(Clone, Debug, PartialEq)]
pub enum MainWidget {
    Home,
    EntryTypes,
    Editor {
        entry_type: String,
        id: usize,
        content: String,
    },
}

impl Widget for MainWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
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
                let p = Paragraph::new(help_messages)
                    .block(Block::default().title("Main").borders(Borders::ALL))
                    .alignment(Alignment::Center);
                p.render(area, buf);
            }
            MainWidget::EntryTypes => {
                let entry_types: Vec<ListItem> = list_entry_types().unwrap_or_default();
                let entry_types_list = List::new(entry_types)
                    .block(Block::default().borders(Borders::ALL).title("List"))
                    .start_corner(Corner::TopLeft);

                entry_types_list.render(area, buf);
            }
            MainWidget::Editor { content, .. } => {
                let editor = Paragraph::new(content.as_ref())
                    .block(Block::default().borders(Borders::ALL).title("Editro"));
                editor.render(area, buf);
            }
        }
    }
}

impl MainWidget {
    pub fn get_input(&self) -> &str {
        match self {
            Self::Editor { content, .. } => content,
            _ => "",
        }
    }

    pub fn input_push(&mut self, c: char) {
        if let Self::Editor {
            ref mut content, ..
        } = self
        {
            content.push(c);
        }
    }

    pub fn input_pop(&mut self) {
        if let Self::Editor {
            ref mut content, ..
        } = self
        {
            content.pop();
        }
    }
}

#[derive(Default, Clone)]
pub struct CmdLine {
    pub(crate) message: String,
}

impl Widget for CmdLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let message = Paragraph::new(self.message.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Command"));
        message.render(area, buf);
    }
}

fn list_entry_types() -> Result<Vec<ListItem<'static>>, Box<dyn Error>> {
    let config: QuakeConfig = serde_yaml::from_str(fs::read_to_string(".quake.yaml")?.as_str())?;
    let entry_defines_path = Path::new(&config.workspace).join(EntryPaths::entries_define());
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
