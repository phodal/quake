use crate::app::App;
use tui::backend::Backend;
use tui::layout::Corner;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let entry_dirs: Vec<ListItem> = app
        .entry_dirs
        .iter()
        .rev()
        .map(|dir| {
            let dir_name = Spans::from(vec![Span::styled(
                dir.display().to_string(),
                Style::default().fg(Color::Yellow),
            )]);

            ListItem::new(vec![Spans::from("-".repeat(size.width as usize)), dir_name])
        })
        .collect();
    let dir_list = List::new(entry_dirs)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .start_corner(Corner::TopLeft);
    f.render_widget(dir_list, size);
}
