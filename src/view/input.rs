use crossterm::{
    cursor::{MoveDown, MoveToColumn, MoveUp},
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use std::{
    fmt::Display,
    io::{self, Write},
};

use super::{print, PrintSize};

const MAX_ROWS: usize = 12;

pub enum ReturnType {
    Cancel,
    Exit,
}

enum TextInputType {
    Text,
    Password,
}

/**
## A struct to handle text input
#### It handles the input, cursor position, and input type (text or password), it only handles input (special actions (like Ctrl+C) are handled in the main function)
*/
struct TextInput {
    input: String,
    position: usize,
    input_type: TextInputType,
    cursor_position: usize,
}

impl TextInput {
    fn new(position: usize, input_type: TextInputType) -> Self {
        Self {
            input: String::new(),
            position,
            input_type,
            cursor_position: 0,
        }
    }

    fn handle_event(&mut self, event: KeyEvent) {
        match event.modifiers {
            KeyModifiers::ALT => match event.code {
                KeyCode::Char('b') => {
                    // Move to the beginning of the word
                    // If input type is password, go to the beginning of the line
                    match self.input_type {
                        TextInputType::Text => {
                            // Check if the previous character is a whitespace
                            if self.cursor_position > 0
                                && self.input[self.cursor_position - 1..self.cursor_position]
                                    .chars()
                                    .next()
                                    .unwrap()
                                    .is_whitespace()
                            {
                                self.cursor_position -= 1;
                            }

                            if let Some(pos) =
                                self.input[..self.cursor_position].rfind(char::is_whitespace)
                            {
                                self.cursor_position = pos + 1;
                            } else {
                                self.cursor_position = 0;
                            }
                        }
                        TextInputType::Password => {
                            self.cursor_position = 0;
                        }
                    }

                    let column = self.position + self.cursor_position;
                    execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                }
                KeyCode::Char('f') => {
                    match self.input_type {
                        TextInputType::Text => {
                            // Move to the end of the word
                            // Check if the cursor is on a whitespace
                            if self.cursor_position < self.input.len()
                                && self.input[self.cursor_position..]
                                    .chars()
                                    .next()
                                    .unwrap()
                                    .is_whitespace()
                            {
                                self.cursor_position += 1;
                            }

                            if let Some(pos) =
                                self.input[self.cursor_position..].find(char::is_whitespace)
                            {
                                self.cursor_position += pos;
                            } else {
                                self.cursor_position = self.input.len();
                            }
                        }
                        TextInputType::Password => {
                            self.cursor_position = self.input.len();
                        }
                    }

                    let column = self.position + self.cursor_position;
                    execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                }
                _ => {}
            },
            KeyModifiers::CONTROL => match event.code {
                KeyCode::Char('a') => {
                    // Move to the beginning of the line
                    self.cursor_position = 0;
                    let column = self.position + self.cursor_position;
                    execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                }
                KeyCode::Char('e') => {
                    // Move to the end of the line
                    self.cursor_position = self.input.len();
                    let column = self.position + self.cursor_position;
                    execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                }
                KeyCode::Char('u') => {
                    // Remove all text before the cursor
                    execute!(
                        io::stdout(),
                        MoveToColumn(self.position as u16),
                        terminal::Clear(ClearType::UntilNewLine)
                    )
                    .unwrap();
                    self.input.drain(..self.cursor_position);
                    self.cursor_position = 0;

                    print(format!("{}", self.input));
                    io::stdout().flush().unwrap();
                    execute!(
                        io::stdout(),
                        MoveToColumn((self.position + self.cursor_position) as u16)
                    )
                    .unwrap();
                }
                KeyCode::Char('h') | KeyCode::Char('w') => {
                    match self.input_type {
                        TextInputType::Text => {
                            // Remove the word before the cursor
                            if let Some(pos) =
                                self.input[..self.cursor_position].rfind(char::is_whitespace)
                            {
                                self.input.drain(pos..self.cursor_position);
                                self.cursor_position = pos;
                            } else {
                                self.input.drain(..self.cursor_position);
                                self.cursor_position = 0;
                            }
                        }
                        TextInputType::Password => {
                            // Move to the beginning of the line
                            self.cursor_position = 0;
                            self.input.clear();
                        }
                    }
                    execute!(
                        io::stdout(),
                        MoveToColumn(self.position as u16),
                        terminal::Clear(ClearType::UntilNewLine)
                    )
                    .unwrap();
                    print(format!("{}", self.input));
                    io::stdout().flush().unwrap();
                    execute!(
                        io::stdout(),
                        MoveToColumn((self.position + self.cursor_position) as u16)
                    )
                    .unwrap();
                }
                _ => {}
            },
            KeyModifiers::NONE => match event.code {
                KeyCode::Backspace => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.input.remove(self.cursor_position);

                        execute!(
                            io::stdout(),
                            MoveToColumn(self.position as u16),
                            terminal::Clear(ClearType::UntilNewLine)
                        )
                        .unwrap();

                        match self.input_type {
                            TextInputType::Text => {
                                print(format!("{}", self.input));
                            }
                            TextInputType::Password => {
                                print(format!("{}", "*".repeat(self.input.len())));
                            }
                        }
                        io::stdout().flush().unwrap();

                        let column = self.position + self.cursor_position;
                        execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                    }
                }
                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        let column = self.position + self.cursor_position;
                        execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                    }
                }
                KeyCode::Right => {
                    if self.cursor_position < self.input.len() {
                        self.cursor_position += 1;
                        let column = self.position + self.cursor_position;
                        execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                    }
                }
                KeyCode::Char(c) => {
                    self.input.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                    execute!(
                        io::stdout(),
                        MoveToColumn(self.position as u16),
                        terminal::Clear(ClearType::UntilNewLine)
                    )
                    .unwrap();
                    match self.input_type {
                        TextInputType::Text => {
                            print(format!("{}", self.input));
                        }
                        TextInputType::Password => {
                            print(format!("{}", "*".repeat(self.input.len())));
                        }
                    }

                    io::stdout().flush().unwrap();

                    let column = self.position + self.cursor_position;
                    execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn text(prompt: &str) -> Result<String, ReturnType> {
    let PrintSize {
        cols: prompt_length,
        rows: _,
    } = print(format!("{}", prompt));
    io::stdout().flush().unwrap();

    get_user_text_input(prompt_length, TextInputType::Text)
}

pub fn password(prompt: &str) -> Result<String, ReturnType> {
    let PrintSize {
        cols: prompt_length,
        rows: _,
    } = print(format!("{}", prompt));
    io::stdout().flush().unwrap();

    get_user_text_input(prompt_length, TextInputType::Password)
}

pub fn list<T>(prompt: &str, items: Vec<T>) -> Result<T, ReturnType>
where
    T: Display + Clone,
{
    enable_raw_mode().unwrap();
    let mut selected = 0;
    let mut offset = 0;

    let PrintSize {
        cols: prompt_length,
        rows: _,
    } = print(format!("{}\n", prompt));

    let total_rows = items.len();
    let available_rows = terminal::size().unwrap().1 as usize - 1; // 1 for the prompt

    let mut usable_rows = if total_rows > available_rows {
        available_rows
    } else {
        total_rows
    };
    if usable_rows > MAX_ROWS {
        usable_rows = MAX_ROWS;
    }

    for i in offset..usable_rows + offset {
        if i == selected {
            print(format!("$cc `>` {}\n", items[i]));
        } else {
            if i == usable_rows + offset - 1 && total_rows > usable_rows {
                print(format!("▼ {}\n", items[i]));
            } else {
                print(format!("  {}\n", items[i]));
            }
        }
    }

    execute!(
        io::stdout(),
        MoveUp((usable_rows + 1) as u16),
        MoveToColumn((prompt_length) as u16)
    )
    .unwrap();

    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        execute!(
                            io::stdout(),
                            MoveToColumn(0),
                            terminal::Clear(ClearType::CurrentLine)
                        )
                        .unwrap();
                        print(format!("{}$cr `canceled`\n", prompt));

                        disable_raw_mode().unwrap();
                        return Err(ReturnType::Cancel);
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < total_rows - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Char('c') => {
                        if event.modifiers == KeyModifiers::CONTROL {
                            write!(io::stdout(), "\n\r").unwrap();
                            disable_raw_mode().unwrap();
                            return Err(ReturnType::Exit);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            let diff = (selected + 1) as isize - (usable_rows / 2) as isize;
            if diff <= 0 {
                offset = 0;
            } else if diff > 0 {
                offset = diff as usize;
            }
            if offset > total_rows - usable_rows {
                offset = total_rows - usable_rows;
            }

            // Render the prompt
            execute!(
                io::stdout(),
                MoveToColumn(0),
                terminal::Clear(ClearType::FromCursorDown)
            )
            .unwrap();
            print(format!("{} {}", prompt, diff));
            execute!(io::stdout(), MoveDown(1), MoveToColumn(0)).unwrap();

            // Render the list
            for i in offset..usable_rows + offset {
                if i == selected {
                    print(format!("$cc `>` {}", items[i]));
                } else {
                    if i == usable_rows + offset - 1 && total_rows > usable_rows + offset {
                        print(format!("▼ {}", items[i]));
                    } else if offset > 0 && i == offset {
                        print(format!("▲ {}", items[i]));
                    } else {
                        print(format!("  {}", items[i]));
                    }
                }
                execute!(io::stdout(), MoveDown(1), MoveToColumn(0)).unwrap();
            }

            // Go back to the prompt
            execute!(
                io::stdout(),
                MoveUp((usable_rows + 1) as u16),
                MoveToColumn((prompt_length) as u16)
            )
            .unwrap();

            io::stdout().flush().unwrap();
        }
    }

    disable_raw_mode().unwrap();

    Ok(items[selected].clone())
}

fn get_user_text_input(position: usize, input_type: TextInputType) -> Result<String, ReturnType> {
    enable_raw_mode().unwrap();

    let mut text_input = TextInput::new(position, input_type);

    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(event) => match event.modifiers {
                    KeyModifiers::CONTROL => match event.code {
                        KeyCode::Char('c') => {
                            write!(io::stdout(), "\n\r").unwrap();
                            disable_raw_mode().unwrap();
                            return Err(ReturnType::Exit);
                        }
                        _ => text_input.handle_event(event),
                    },
                    KeyModifiers::NONE => match event.code {
                        KeyCode::Esc => {
                            execute!(
                                io::stdout(),
                                MoveToColumn(position as u16),
                                terminal::Clear(ClearType::UntilNewLine)
                            )
                            .unwrap();
                            print("$cr `canceled`\n");

                            disable_raw_mode().unwrap();
                            return Err(ReturnType::Cancel);
                        }
                        KeyCode::Enter => {
                            break;
                        }
                        _ => text_input.handle_event(event),
                    },
                    _ => text_input.handle_event(event),
                },
                _ => {}
            }

            io::stdout().flush().unwrap();
        }
    }

    write!(io::stdout(), "\n\r").unwrap();

    disable_raw_mode().unwrap();
    Ok(text_input.input)
}
