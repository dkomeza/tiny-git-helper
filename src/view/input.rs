use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

pub fn read_password_input(prompt: &str) -> String {
    let mut password = String::new();
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(event) => match event.code {
                    KeyCode::Enter => break,
                    KeyCode::Esc => return String::new(),
                    KeyCode::Backspace => {
                        password.pop();
                        execute!(io::stdout(), terminal::Clear(ClearType::CurrentLine)).unwrap();
                        print!("\r{}: {}", prompt, "*".repeat(password.len()));
                        io::stdout().flush().unwrap();
                    }
                    _ => {
                        if let KeyCode::Char(c) = event.code {
                            password.push(c);
                            print!("\r{}: {}", prompt, "*".repeat(password.len()));
                            io::stdout().flush().unwrap();
                        }
                    }
                },
                _ => {}
            }
        }
    }
    write!(io::stdout(), "\n\r").unwrap();
    password
}
