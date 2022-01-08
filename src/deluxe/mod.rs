use std::io::stdout;
use std::time::Duration;

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

mod game_object;
mod movement;
mod render;
pub use game_object::GameObject;
pub use movement::{go_down, go_left, go_right, go_up};
pub use render::{render, DEFAULT_BACKGROUND_COLOR, DEFAULT_FORGROUND_COLOR};

pub const FPS_2: Duration = Duration::from_millis(1000 / 2);
pub const FPS_24: Duration = Duration::from_millis(1000 / 24);
pub const FPS_30: Duration = Duration::from_millis(1000 / 30);
pub const FPS_60: Duration = Duration::from_millis(1000 / 60);

pub struct GameManager {
    pub frame_rate: Duration,
    pub game_objects: Vec<GameObject>,
    pub height: i16,
    pub is_running: bool,
    pub title: String,
    pub width: i16,
}
impl GameManager {
    pub fn new(title: String, width: i16, height: i16) -> Self {
        GameManager {
            frame_rate: FPS_24,
            game_objects: vec![],
            height,
            is_running: false,
            title,
            width,
        }
    }

    pub fn run(&mut self) {
        // Clear the screen
        enable_raw_mode().expect("Could not enable raw mode");
        execute!(
            stdout(),
            EnterAlternateScreen,
            SetForegroundColor(DEFAULT_FORGROUND_COLOR),
            SetBackgroundColor(DEFAULT_BACKGROUND_COLOR),
            // SetSize(WIDTH, HEIGHT),
            SetTitle("Hotel Deluxe 2".to_string()),
            Hide
        );
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;

        // Reset screen after game is over
        execute!(
            stdout(),
            Clear(ClearType::All),
            ResetColor,
            Show,
            LeaveAlternateScreen
        );
        disable_raw_mode().expect("Could not disable raw mode");
    }

    pub fn render(&mut self) {
        if poll(self.frame_rate).expect("Could not poll events") {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => self.stop(),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                }) => self.stop(),

                Event::Key(evt) => {
                    if evt.code == KeyCode::Esc {
                        self.stop();
                    }
                    if evt.code == KeyCode::Up {
                        go_up(&mut self.game_objects);
                    }
                    if evt.code == KeyCode::Down {
                        go_down(&mut self.game_objects, self.height);
                    }
                    if evt.code == KeyCode::Left {
                        go_left(&mut self.game_objects);
                    }
                    if evt.code == KeyCode::Right {
                        go_right(&mut self.game_objects, self.width);
                    }
                    if evt.code == KeyCode::Char('w') {
                        // nick.y -= 1;
                        go_up(&mut self.game_objects);
                    }
                    if evt.code == KeyCode::Char('s') {
                        // nick.y += 1;
                        go_down(&mut self.game_objects, self.height);
                    }
                    if evt.code == KeyCode::Char('a') {
                        // nick.x -= 1;
                        go_left(&mut self.game_objects);
                    }
                    if evt.code == KeyCode::Char('d') {
                        // nick.x += 1;
                        go_right(&mut self.game_objects, self.width);
                    }
                    if evt.code == KeyCode::Char('e') {
                        for obj in self.game_objects.iter_mut() {
                            obj.say();
                        }
                    }
                }
                _ => {}
            }
        }
        render(&self.game_objects, self.width, self.height);
    }
}
