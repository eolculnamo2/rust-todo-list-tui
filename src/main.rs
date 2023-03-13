use std::io;

use anyhow::Result;
use components::{search_field::render_search_field, todo_list::render_todo_list};
use crossterm::{
    event::{self, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use state::{AppElement, TerminalEventOutcome};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

mod components;
mod state;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = state::AppState::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3), // these seem to control how much space chunks take?
                        Constraint::Min(12),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let main_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(main_block, f.size());

            let title =
                Paragraph::new("Todo List: Push \"i\" to insert text.").style(Style::default());
            f.render_widget(title, chunks[0]);

            let highlighted_list_index = match app_state.focused_element {
                AppElement::ListItem(n) => Some(n),
                _ => None,
            };
            render_search_field(&mut f, &app_state, &highlighted_list_index, &chunks);
            render_todo_list(&mut f, &app_state, highlighted_list_index.clone(), &chunks);
        })?;

        let read_event = event::read()?;
        let outcome = app_state.terminal_event_handler(read_event);
        if outcome == TerminalEventOutcome::EndProgram {
            break;
        }
    }
    Ok(())
}
