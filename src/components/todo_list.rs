use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
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
            ListItem::new(todo.name.as_str()).style(
                Style::default()
                    .fg(if todo.is_done {
                        Color::Green
                    } else {
                        match highlighted_list_index {
                            Some(n) if n as usize == i => Color::Yellow,
                            _ => Color::White,
                        }
                    })
                    .add_modifier(match highlighted_list_index {
                        Some(n) if n as usize == i => Modifier::BOLD,
                        _ => Modifier::empty(),
                    }),
            )
        })
        .collect::<Vec<ListItem>>();

    let list = List::new(list_items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_symbol(">>");
    f.render_widget(list, chunks[2]);
}
