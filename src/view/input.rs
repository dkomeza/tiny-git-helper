use crossterm::{
    cursor::{MoveDown, MoveToColumn, MoveUp},
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};

use super::{print, PrintSize};

const MAX_ROWS: usize = 12;

pub enum ReturnType {
    Cancel,
    Exit,
}

#[derive(PartialEq, Clone, Copy)]
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
            KeyModifiers::NONE | KeyModifiers::SHIFT => match event.code {
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

struct ListValue<T: Display + Clone> {
    key: usize,
    value: T,
    matched: bool,
}

impl<T: Display + Clone> Display for ListValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
    use fuzzy_matcher::skim::SkimMatcherV2;

    if items.is_empty() {
        return Err(ReturnType::Cancel);
    }

    let mut kv_items: Vec<ListValue<T>> = items
        .iter()
        .enumerate()
        .map(|(i, x)| ListValue {
            key: i,
            value: x.clone(),
            matched: true,
        })
        .collect();

    super::init();

    let mut selected = 0;
    let matcher = SkimMatcherV2::default();

    let PrintSize {
        cols: prompt_length,
        rows: _,
    } = print(format!("{}\n", prompt));

    let mut text_input = TextInput::new(prompt_length, TextInputType::Text);
    let mut available_rows = terminal::size().unwrap().1 as usize - 1; // 1 for the prompt
    if available_rows > MAX_ROWS {
        available_rows = MAX_ROWS;
    }

    let usable_rows;
    if kv_items.len() < available_rows {
        usable_rows = kv_items.len()
    } else {
        usable_rows = available_rows;
    }

    for _ in 0..usable_rows {
        print(format!("\n\r"));
    }
    execute!(io::stdout(), MoveUp(usable_rows as u16), MoveToColumn(0)).unwrap();

    let rendered = render_list(
        &kv_items,
        0,
        0,
        usable_rows,
        &matcher,
        text_input.input.clone(),
    );

    execute!(
        io::stdout(),
        MoveUp((rendered + 1) as u16),
        MoveToColumn((prompt_length) as u16)
    )
    .unwrap();
    io::stdout().flush().unwrap();

    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(event) => match event.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        execute!(
                            io::stdout(),
                            MoveToColumn(0),
                            terminal::Clear(ClearType::FromCursorDown)
                        )
                        .unwrap();

                        if event.code == KeyCode::Enter {
                            print(format!("{}$cw$b `{}`\n", prompt, items[selected]));
                        } else {
                            print(format!("{}$cr$b `canceled`\n", prompt));
                        }

                        disable_raw_mode().unwrap();
                        return if event.code == KeyCode::Enter {
                            Ok(items[selected].clone())
                        } else {
                            Err(ReturnType::Cancel)
                        };
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }

                        while selected > 0 && !kv_items[selected].matched {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < kv_items.len() - 1 {
                            selected += 1;

                            while selected < kv_items.len() - 1 && !kv_items[selected].matched {
                                selected += 1;
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        if event.modifiers == KeyModifiers::CONTROL && c == 'c' {
                            print("\n");
                            disable_raw_mode().unwrap();
                            return Err(ReturnType::Exit);
                        }

                        text_input.handle_event(event);
                    }
                    _ => text_input.handle_event(event),
                },
                _ => {}
            }

            // Render the prompt
            execute!(
                io::stdout(),
                MoveToColumn(0),
                terminal::Clear(ClearType::FromCursorDown)
            )
            .unwrap();
            print(format!("{}{}", prompt, text_input.input));
            execute!(io::stdout(), MoveDown(1), MoveToColumn(0)).unwrap();

            // Render the list
            kv_items.iter_mut().for_each(|x| {
                x.matched = matcher
                    .fuzzy_match(&x.value.to_string(), &text_input.input)
                    .is_some()
            });
            let visible = kv_items.iter().filter(|x| x.matched).count();

            if !kv_items[selected].matched {
                if let Some(found) = kv_items.iter().find(|&x| x.matched) {
                    selected = found.key;
                } else {
                    selected = 0;
                }
            }

            let diff = (selected + 1) as isize - (usable_rows / 2) as isize;
            let mut offset = 0;
            if diff <= 0 {
                offset = 0;
            } else if diff > 0 {
                offset = diff as usize;
            }
            if visible < usable_rows {
                offset = 0;
            } else if offset > visible - usable_rows {
                offset = visible - usable_rows;
            }

            let rendered = render_list(
                &kv_items,
                selected,
                offset,
                usable_rows,
                &matcher,
                text_input.input.clone(),
            );

            // Go back to the prompt
            execute!(
                io::stdout(),
                MoveUp((rendered + 1) as u16),
                MoveToColumn((prompt_length + text_input.cursor_position) as u16)
            )
            .unwrap();

            io::stdout().flush().unwrap();
        }
    }

    disable_raw_mode().unwrap();

    Ok(items[selected].clone())
}

/// Render the list of items
/// This function assumes that the items are already filtered, and correctly offset, and uses the matcher to color the items
fn render_list<T>(
    items: &Vec<ListValue<T>>,
    selected: usize,
    offset: usize,
    usable_rows: usize,
    matcher: &SkimMatcherV2,
    input: String,
) -> usize
where
    T: Display + Clone,
{
    let mut rendered = 0;
    let mut i = offset;
    while rendered < usable_rows {
        if i >= items.len() {
            break;
        }

        if !items[i].matched {
            i += 1;
            continue;
        }

        if items[i].key == selected {
            print("$cc `>` ");
        } else if rendered == 0 && offset > 0 {
            print("⌃ ");
        } else if rendered == usable_rows - 1 && i < items.len() - 1 {
            print("⌄ ");
        } else {
            print("  ");
        }

        let matched_letters = matcher
            .fuzzy_indices(&items[i].value.to_string(), &input)
            .unwrap_or((0, vec![]))
            .1;

        let word = items[i].value.to_string();

        for (byte_idx, ch) in word.char_indices() {
            if matched_letters.iter().any(|&x| x == byte_idx) {
                print(format!("$cc `{}`", ch));
            } else {
                print(format!("{}", ch));
            }
        }

        rendered += 1;
        i += 1;

        execute!(io::stdout(), MoveDown(1), MoveToColumn(0)).unwrap();
    }

    rendered
}

fn get_user_text_input(position: usize, input_type: TextInputType) -> Result<String, ReturnType> {
    super::init();

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
                            execute!(
                                io::stdout(),
                                MoveToColumn(position as u16),
                                terminal::Clear(ClearType::UntilNewLine)
                            )
                            .unwrap();
                            if input_type == TextInputType::Password {
                                print(format!("$cw$b `{}`", "*".repeat(text_input.input.len())));
                            } else {
                                print(format!("$cw$b `{}`", text_input.input));
                            }
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
