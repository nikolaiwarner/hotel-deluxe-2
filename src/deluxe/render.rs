use std::io::stdout;

use crate::deluxe::game_object::GameObject;
pub use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetSize, SetTitle,
    },
    Result,
};

pub const DEFAULT_FORGROUND_COLOR: Color = Color::Rgb {
    r: 160,
    g: 220,
    b: 158,
};
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::Rgb { r: 0, g: 0, b: 0 };

pub fn render(game_objects: &Vec<GameObject>, width: i16, height: i16) {
    render_background(width, height);
    render_game_objects(game_objects)
}

fn render_background(width: i16, height: i16) {
    let mut string = String::new();
    for _ in 0..height {
        for _ in 0..width {
            string.push_str(" ");
        }
        string = string + "\n\r";
    }
    execute!(
        stdout(),
        MoveTo(0, 0),
        SetForegroundColor(DEFAULT_FORGROUND_COLOR),
        SetBackgroundColor(DEFAULT_BACKGROUND_COLOR),
        Print(string),
    )
    .expect("Could not render background");
}

fn render_game_objects(objects: &Vec<GameObject>) {
    for obj in objects {
        if obj.visible {
            let mut index = 0;
            let lines = obj.sigil().split("\n");
            for line in lines {
                execute!(
                    stdout(),
                    MoveTo(obj.x as u16, (obj.y + index) as u16),
                    SetForegroundColor(obj.color),
                    SetBackgroundColor(obj.background_color),
                    Print(line)
                )
                .expect("Could not render object");
                index += 1;
            }
        }
    }
}
