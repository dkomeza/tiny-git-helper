use crossterm::{
    cursor::{MoveToColumn, MoveToNextLine},
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::stdout;

pub mod input;
pub mod spinner;

pub fn init() {
    enable_raw_mode().unwrap();
}

pub fn clean_up() {
    disable_raw_mode().unwrap();
}

enum VisualEffect {
    SetAttribute(Attribute),
    SetForegroundColor(Color),
    SetBackgroundColor(Color),
}

fn match_effect(effect: &str) -> Vec<VisualEffect> {
    let mut effects: Vec<VisualEffect> = Vec::new();

    let mut i = 1;
    while i < effect.len() {
        let c = effect.chars().nth(i).unwrap();

        let mut special = String::new();
        special.push(c);

        i += 1;

        while i < effect.len() && effect.chars().nth(i).unwrap() != '$' {
            special.push(effect.chars().nth(i).unwrap());
            i += 1;
        }

        match special.as_str() {
            "b" => effects.push(VisualEffect::SetAttribute(Attribute::Bold)),
            "i" => effects.push(VisualEffect::SetAttribute(Attribute::Italic)),
            "u" => effects.push(VisualEffect::SetAttribute(Attribute::Underlined)),
            "s" => effects.push(VisualEffect::SetAttribute(Attribute::Dim)),

            "cr" => effects.push(VisualEffect::SetForegroundColor(Color::Red)),
            "cg" => effects.push(VisualEffect::SetForegroundColor(Color::Green)),
            "cb" => effects.push(VisualEffect::SetForegroundColor(Color::Blue)),
            "cy" => effects.push(VisualEffect::SetForegroundColor(Color::Yellow)),
            "cm" => effects.push(VisualEffect::SetForegroundColor(Color::Magenta)),
            "cc" => effects.push(VisualEffect::SetForegroundColor(Color::Cyan)),
            "cw" => effects.push(VisualEffect::SetForegroundColor(Color::White)),

            "br" => effects.push(VisualEffect::SetBackgroundColor(Color::Red)),
            "bg" => effects.push(VisualEffect::SetBackgroundColor(Color::Green)),
            "bb" => effects.push(VisualEffect::SetBackgroundColor(Color::Blue)),
            "by" => effects.push(VisualEffect::SetBackgroundColor(Color::Yellow)),
            "bm" => effects.push(VisualEffect::SetBackgroundColor(Color::Magenta)),
            "bc" => effects.push(VisualEffect::SetBackgroundColor(Color::Cyan)),
            "bw" => effects.push(VisualEffect::SetBackgroundColor(Color::White)),
            _ => {}
        }

        i += 1;
    }

    effects
}

fn set_new_effects(stdout: &mut std::io::Stdout, effects: &Vec<Vec<VisualEffect>>) {
    execute!(stdout, SetAttribute(Attribute::Reset)).unwrap();
    for effect in effects {
        for e in effect {
            match e {
                VisualEffect::SetAttribute(a) => {
                    execute!(stdout, SetAttribute(*a)).unwrap();
                }
                VisualEffect::SetForegroundColor(c) => {
                    execute!(stdout, SetForegroundColor(*c)).unwrap();
                }
                VisualEffect::SetBackgroundColor(c) => {
                    execute!(stdout, SetBackgroundColor(*c)).unwrap();
                }
            }
        }
    }
}

pub struct PrintSize {
    pub cols: usize,
    pub rows: usize,
}

/**
This function is used to print a string with special effects.

The special effects are defined by the following syntax: $effect `content`
- $b: bold
- $i: italic
- $u: underline

- $cr: red color
- $cg: green color
- $cb: blue color
- $cy: yellow color
- $cm: magenta color
- $cc: cyan color
- $cw: white color

- $br: background red color
- $bg: background green color
- $bb: background blue color
- $by: background yellow color
- $bm: background magenta color
- $bc: background cyan color
- $bw: background white color

- &>: tab (4 spaces)

Multiple effects can be combined, ex. $b$u - bold underline, or $b `Bold and $u `underline``
 */
pub fn printer(content: impl AsRef<str>) -> PrintSize {
    enable_raw_mode().unwrap();
    let size = print(content);
    disable_raw_mode().unwrap();

    size
}

/** Print the content with correct formatting, but without going into raw mode
This function is used to print a string with special effects.

The special effects are defined by the following syntax: $effect `content`
- $b: bold
- $i: italic
- $u: underline

- $cr: red color
- $cg: green color
- $cb: blue color
- $cy: yellow color
- $cm: magenta color
- $cc: cyan color
- $cw: white color

- $br: background red color
- $bg: background green color
- $bb: background blue color
- $by: background yellow color
- $bm: background magenta color
- $bc: background cyan color
- $bw: background white color

- &>: tab (4 spaces)

Multiple effects can be combined, ex. $b$u - bold underline, or $b `Bold and $u `underline``
*/
pub fn print(content: impl AsRef<str>) -> PrintSize {
    let mut size = PrintSize { cols: 0, rows: 0 };
    let mut current_width: usize = 0;

    let content = content.as_ref();
    let chars = content.chars();
    let n = chars.count();
    let mut stdout = stdout();

    let mut effects: Vec<Vec<VisualEffect>> = Vec::new();

    let mut i = 0;
    while i < n {
        let c = content.chars().nth(i).unwrap();

        // Add a special effect
        if c == '$' {
            let mut special = String::new();

            while i < n && content.chars().nth(i).unwrap() != ' ' {
                special.push(content.chars().nth(i).unwrap());
                i += 1;
            }

            let effect = match_effect(&special);
            effects.push(effect);
            set_new_effects(&mut stdout, &effects);

            i += 2;

            continue;
        }

        // Clear the last effect
        if c == '`' {
            effects.pop();

            set_new_effects(&mut stdout, &effects);

            i += 1;
            continue;
        }

        // Print a new line and clear the spaces
        if c == '\n' {
            if current_width > size.cols {
                size.cols = current_width;
                size.rows += 1;
            }

            execute!(stdout, Print('\n'), MoveToNextLine(1)).unwrap();
            i += 1;

            while i < n && content.chars().nth(i).unwrap() == ' ' {
                i += 1;
            }

            continue;
        }

        if c == '&' {
            i += 1;

            if i >= n {
                break;
            }

            let c = content.chars().nth(i).unwrap();

            // Print a 4 wide space (tab)
            if c == '>' {
                execute!(stdout, Print("    ")).unwrap();
            }

            i += 1;
            continue;
        }

        current_width += 1;
        execute!(stdout, Print(c)).unwrap();

        i += 1;
    }

    if current_width > size.cols {
        size.cols = current_width;
        size.rows += 1;
    }

    size
}

pub fn clear_line() {
    execute!(
        stdout(),
        MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        MoveToColumn(0)
    )
    .unwrap();
}

pub fn no_subcommand_error() {
    let eror_message = r#"
        $b$cr `error`: no subcommand provided

        $b$u `Usage`: $b `tgh` [COMMAND]

        For more information try $b `'tgh --help'`
    "#;

    printer(eror_message);
}
