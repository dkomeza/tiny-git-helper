use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SORTING {
    LastUpdated,
    Alphabetical,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PROTOCOL {
    SSH,
    HTTPS,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum COLOR {
    NORMAL,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    GRAY,
}

impl COLOR {
    pub fn to_string(&self) -> String {
        match self {
            COLOR::NORMAL => "Normal",
            COLOR::RED => "Red",
            COLOR::GREEN => "Green",
            COLOR::YELLOW => "Yellow",
            COLOR::BLUE => "Blue",
            COLOR::MAGENTA => "Magenta",
            COLOR::CYAN => "Cyan",
            COLOR::WHITE => "White",
            COLOR::GRAY => "Gray",
        }
        .to_string()
    }

    pub fn as_inquire_color(&self) -> inquire::ui::Color {
        match self {
            COLOR::RED => inquire::ui::Color::LightRed,
            COLOR::GREEN => inquire::ui::Color::LightGreen,
            COLOR::YELLOW => inquire::ui::Color::LightYellow,
            COLOR::BLUE => inquire::ui::Color::LightBlue,
            COLOR::MAGENTA => inquire::ui::Color::LightMagenta,
            COLOR::CYAN => inquire::ui::Color::LightCyan,
            COLOR::WHITE => inquire::ui::Color::White,
            COLOR::GRAY => inquire::ui::Color::Grey,
            _ => inquire::ui::Color::White,
        }
    }
}
impl std::fmt::Display for COLOR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let color = match self {
            COLOR::RED => "\x1B[31m",
            COLOR::GREEN => "\x1B[32m",
            COLOR::YELLOW => "\x1B[33m",
            COLOR::BLUE => "\x1B[34m",
            COLOR::MAGENTA => "\x1B[35m",
            COLOR::CYAN => "\x1B[36m",
            COLOR::WHITE => "\x1B[97m",
            COLOR::GRAY => "\x1B[37m",
            _ => "\x1B[m",
        };

        write!(f, "{}{}\x1B[m", color, self.to_string())
    }
}
