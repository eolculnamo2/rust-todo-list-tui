use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::state::AppState;

pub fn render_todo_list(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    app_state: &AppState,
    highlighted_list_index: Option<i32>,
    chunks: &Vec<Rect>,
) {
    let list_items = app_state
        .todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            ListItem::new(todo.as_str()).style(Style::default().fg(match highlighted_list_index {
                Some(n) if n as usize == i => Color::Yellow,
                _ => Color::White,
            }))
        })
        .collect::<Vec<ListItem>>();

    let list = List::new(list_items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_symbol(">>");
    f.render_widget(list, chunks[1]);
}
