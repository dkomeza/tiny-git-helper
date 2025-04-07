use crossterm::{
    cursor::{MoveDown, MoveToColumn, MoveUp},
    event::{self, KeyCode, KeyModifiers},
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

pub fn text(prompt: &str) -> Result<String, ReturnType> {
    get_user_text_input(prompt, TextInputType::Text)
}

pub fn password(prompt: &str) -> Result<String, ReturnType> {
    get_user_text_input(prompt, TextInputType::Password)
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

fn get_user_text_input(prompt: &str, input_type: TextInputType) -> Result<String, ReturnType> {
    enable_raw_mode().unwrap();

    let mut input = String::new();
    let mut cursor_position = 0;

    let PrintSize {
        cols: prompt_length,
        rows: _,
    } = print(format!("{}", prompt));
    io::stdout().flush().unwrap();

    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(event) => match event.modifiers {
                    KeyModifiers::ALT => match event.code {
                        KeyCode::Char('b') => {
                            // Move to the beginning of the word
                            // If input type is password, go to the beginning of the line
                            match input_type {
                                TextInputType::Text => {
                                    // Check if the previous character is a whitespace
                                    if cursor_position > 0
                                        && input[cursor_position - 1..cursor_position]
                                            .chars()
                                            .next()
                                            .unwrap()
                                            .is_whitespace()
                                    {
                                        cursor_position -= 1;
                                    }

                                    if let Some(pos) =
                                        input[..cursor_position].rfind(char::is_whitespace)
                                    {
                                        cursor_position = pos + 1;
                                    } else {
                                        cursor_position = 0;
                                    }
                                }
                                TextInputType::Password => {
                                    cursor_position = 0;
                                }
                            }

                            let column = prompt_length + cursor_position;
                            execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                        }
                        KeyCode::Char('f') => {
                            match input_type {
                                TextInputType::Text => {
                                    // Move to the end of the word
                                    // Check if the cursor is on a whitespace
                                    if cursor_position < input.len()
                                        && input[cursor_position..]
                                            .chars()
                                            .next()
                                            .unwrap()
                                            .is_whitespace()
                                    {
                                        cursor_position += 1;
                                    }

                                    if let Some(pos) =
                                        input[cursor_position..].find(char::is_whitespace)
                                    {
                                        cursor_position += pos;
                                    } else {
                                        cursor_position = input.len();
                                    }
                                }
                                TextInputType::Password => {
                                    cursor_position = input.len();
                                }
                            }

                            let column = prompt_length + cursor_position;
                            execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                        }
                        _ => {}
                    },
                    KeyModifiers::CONTROL => match event.code {
                        KeyCode::Char('c') => {
                            write!(io::stdout(), "\n\r").unwrap();
                            disable_raw_mode().unwrap();
                            return Err(ReturnType::Exit);
                        }
                        KeyCode::Char('a') => {
                            // Move to the beginning of the line
                            cursor_position = 0;
                            let column = prompt_length + cursor_position;
                            execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                        }
                        KeyCode::Char('e') => {
                            // Move to the end of the line
                            cursor_position = input.len();
                            let column = prompt_length + cursor_position;
                            execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                        }
                        KeyCode::Char('u') => {
                            // Remove all text before the cursor
                            execute!(
                                io::stdout(),
                                MoveToColumn(0),
                                terminal::Clear(ClearType::CurrentLine)
                            )
                            .unwrap();
                            input.drain(..cursor_position);
                            cursor_position = 0;

                            print(format!("{}{}", prompt, input));
                            io::stdout().flush().unwrap();
                            execute!(
                                io::stdout(),
                                MoveToColumn((prompt_length + cursor_position) as u16)
                            )
                            .unwrap();
                        }
                        KeyCode::Char('h') | KeyCode::Char('w') => {
                            match input_type {
                                TextInputType::Text => {
                                    // Remove the word before the cursor
                                    if let Some(pos) =
                                        input[..cursor_position].rfind(char::is_whitespace)
                                    {
                                        input.drain(pos..cursor_position);
                                        cursor_position = pos;
                                    } else {
                                        input.drain(..cursor_position);
                                        cursor_position = 0;
                                    }
                                }
                                TextInputType::Password => {
                                    // Move to the beginning of the line
                                    cursor_position = 0;
                                    input.clear();
                                }
                            }
                            execute!(
                                io::stdout(),
                                MoveToColumn(0),
                                terminal::Clear(ClearType::CurrentLine)
                            )
                            .unwrap();
                            print(format!("{}{}", prompt, input));
                            io::stdout().flush().unwrap();
                            execute!(
                                io::stdout(),
                                MoveToColumn((prompt_length + cursor_position) as u16)
                            )
                            .unwrap();
                        }
                        _ => {}
                    },
                    KeyModifiers::NONE => match event.code {
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
                        KeyCode::Backspace => {
                            if cursor_position > 0 {
                                cursor_position -= 1;
                                input.remove(cursor_position);

                                execute!(io::stdout(), terminal::Clear(ClearType::CurrentLine))
                                    .unwrap();

                                match input_type {
                                    TextInputType::Text => {
                                        print(format!("\r{}{}", prompt, input));
                                    }
                                    TextInputType::Password => {
                                        print(format!("\r{}{}", prompt, "*".repeat(input.len())));
                                    }
                                }
                                io::stdout().flush().unwrap();

                                let column = prompt_length + cursor_position;
                                execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                            }
                        }
                        KeyCode::Left => {
                            if cursor_position > 0 {
                                cursor_position -= 1;
                                let column = prompt_length + cursor_position;
                                execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                            }
                        }
                        KeyCode::Right => {
                            if cursor_position < input.len() {
                                cursor_position += 1;
                                let column = prompt_length + cursor_position;
                                execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                            }
                        }
                        KeyCode::Char(c) => {
                            input.insert(cursor_position, c);
                            cursor_position += 1;
                            match input_type {
                                TextInputType::Text => {
                                    print(format!("\r{}{}", prompt, input));
                                }
                                TextInputType::Password => {
                                    print(format!("\r{}{}", prompt, "*".repeat(input.len())));
                                }
                            }

                            io::stdout().flush().unwrap();

                            let column = prompt_length + cursor_position;
                            execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }

            io::stdout().flush().unwrap();
        }
    }

    write!(io::stdout(), "\n\r").unwrap();

    disable_raw_mode().unwrap();
    Ok(input)
}
