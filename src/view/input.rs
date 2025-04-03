use crossterm::{
    cursor::MoveToColumn,
    event::{self, KeyCode, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use std::io::{self, Write};

use crate::view::{print, printer};

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

fn get_user_text_input(prompt: &str, input_type: TextInputType) -> Result<String, ReturnType> {
    enable_raw_mode().unwrap();

    let mut input = String::new();
    let mut cursor_position = 0;

    print!("{}: ", prompt);
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

                            let column = prompt.len() + 2 + cursor_position;
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

                            let column = prompt.len() + 2 + cursor_position;
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
                            let column = prompt.len() + 2 + cursor_position;
                            execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                        }
                        KeyCode::Char('e') => {
                            // Move to the end of the line
                            cursor_position = input.len();
                            let column = prompt.len() + 2 + cursor_position;
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

                            print!("{}: {}", prompt, input);
                            io::stdout().flush().unwrap();
                            execute!(
                                io::stdout(),
                                MoveToColumn((prompt.len() + 2 + cursor_position) as u16)
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
                            print!("{}: {}", prompt, input);
                            io::stdout().flush().unwrap();
                            execute!(
                                io::stdout(),
                                MoveToColumn((prompt.len() + 2 + cursor_position) as u16)
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
                            printer(&format!("{}: $cr `canceled`\n", prompt));

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
                                print!("\r{}: {}", prompt, input);
                                io::stdout().flush().unwrap();

                                let column = prompt.len() + 2 + cursor_position;
                                execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                            }
                        }
                        KeyCode::Left => {
                            if cursor_position > 0 {
                                cursor_position -= 1;
                                let column = prompt.len() + 2 + cursor_position;
                                execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                            }
                        }
                        KeyCode::Right => {
                            if cursor_position < input.len() {
                                cursor_position += 1;
                                let column = prompt.len() + 2 + cursor_position;
                                execute!(io::stdout(), MoveToColumn(column as u16)).unwrap();
                            }
                        }
                        KeyCode::Char(c) => {
                            input.insert(cursor_position, c);
                            cursor_position += 1;
                            match input_type {
                                TextInputType::Text => {
                                    print!("\r{}: {}", prompt, input);
                                }
                                TextInputType::Password => {
                                    print!("\r{}: {}", prompt, "*".repeat(input.len()));
                                }
                            }

                            io::stdout().flush().unwrap();

                            let column = prompt.len() + 2 + cursor_position;
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
