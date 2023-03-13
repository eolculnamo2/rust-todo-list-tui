use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Clone, Eq, PartialEq)]
pub enum Mode {
    Normal,
    Edit,
}

#[derive(Clone, Eq, PartialEq)]
pub enum TerminalEventOutcome {
    None,
    EndProgram,
}

#[derive(Clone, Eq, PartialEq)]
pub enum AppElement {
    TodoTextInput,
    ListItem(i32),
}

#[derive(Clone)]
pub struct TodoItem {
    pub name: String,
    pub is_done: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub new_todo: TodoItem,
    pub todos: Vec<TodoItem>,
    pub mode: Mode,
    pub focused_element: AppElement,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            new_todo: TodoItem {
                name: String::new(),
                is_done: false,
            },
            todos: vec![],
            mode: Mode::Normal,
            focused_element: AppElement::TodoTextInput,
        }
        .to_owned()
    }

    pub fn terminal_event_handler(&mut self, event: Event) -> TerminalEventOutcome {
        match event {
            Event::Key(key_event) => match key_event {
                // Quit
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => {
                    if self.mode == Mode::Normal {
                        TerminalEventOutcome::EndProgram
                    } else {
                        TerminalEventOutcome::None
                    }
                }

                // Tab Elements
                KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: KeyModifiers::SHIFT,
                    kind: _,
                    state: _,
                } => {
                    self.mode = Mode::Normal;
                    match self.focused_element {
                        AppElement::TodoTextInput if self.todos.len() > 0 => {
                            self.focused_element = AppElement::ListItem(0);
                        }
                        AppElement::ListItem(n) => {
                            if n as usize <= 0 {
                                self.focused_element = AppElement::TodoTextInput;
                            } else {
                                self.focused_element = AppElement::ListItem(n - 1)
                            }
                        }
                        _ => self.focused_element = AppElement::TodoTextInput,
                    }

                    TerminalEventOutcome::None
                }
                KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                } => {
                    self.mode = Mode::Normal;
                    match self.focused_element {
                        AppElement::TodoTextInput if self.todos.len() > 0 => {
                            self.focused_element = AppElement::ListItem(0);
                        }
                        AppElement::ListItem(n) => {
                            if n as usize >= self.todos.len() - 1 {
                                self.focused_element = AppElement::TodoTextInput;
                            } else {
                                self.focused_element = AppElement::ListItem(n + 1)
                            }
                        }
                        _ => self.focused_element = AppElement::TodoTextInput,
                    }

                    TerminalEventOutcome::None
                }

                // Exit Edit
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    self.mode = Mode::Normal;
                    TerminalEventOutcome::None
                }

                // Add Todo
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    if self.mode == Mode::Edit && self.new_todo.name.trim().len() > 0 {
                        self.todos.push(self.new_todo.clone());
                        self.new_todo = TodoItem {
                            name: String::new(),
                            is_done: false,
                        };
                    }
                    TerminalEventOutcome::None
                }
                // Toggle Todo Done
                KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: _,
                    kind: _,
                    state: _,
                } if self.mode == Mode::Normal => {
                    if let AppElement::ListItem(n) = self.focused_element {
                        let new_is_done = !self.todos[n as usize].is_done;
                        self.todos[n as usize].is_done = new_is_done;
                    }
                    TerminalEventOutcome::None
                }

                // Delete text
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: _,
                    kind: KeyEventKind::Press | KeyEventKind::Repeat,
                    state: _,
                } => {
                    if self.mode == Mode::Edit {
                        self.new_todo.name.pop();
                    }
                    TerminalEventOutcome::None
                }
                // typing in input
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    // enter edit mode
                    if self.mode == Mode::Normal && c == 'i' {
                        self.mode = Mode::Edit;
                    } else if self.mode == Mode::Edit {
                        self.new_todo.name.push(c);
                    }
                    TerminalEventOutcome::None
                }
                _ => TerminalEventOutcome::None,
            },
            _ => TerminalEventOutcome::None,
        }
    }
}
