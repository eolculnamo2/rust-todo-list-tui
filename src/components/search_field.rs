use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{AppState, Mode};

pub fn render_search_field(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    app_state: &AppState,
    highlighted_list_index: &Option<i32>,
    chunks: &Vec<Rect>,
) {
    let input_paragraph = Paragraph::new(app_state.new_todo.clone())
        .style(match app_state.mode {
            Mode::Edit if highlighted_list_index.is_none() => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input_paragraph, chunks[0]);
}
