pub use crossterm::style::Color;

pub struct GameObject {
    pub background_color: Color,
    pub color: Color,
    pub driveable: bool,
    pub id: i32,
    pub name: String,
    pub sigil_index: usize,
    pub sigils: Vec<String>,
    pub solid: bool,
    pub visible: bool,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl GameObject {
    pub fn say(&self) {
        println!("hi! im {}! i look like: {} \r", self.name, self.sigil());
    }

    pub fn sigil(&self) -> &str {
        &self.sigils[self.sigil_index]
    }

    pub fn set_sigil(&mut self, sigil_index: usize) {
        self.sigil_index = sigil_index;
    }

    pub fn next_sigil(&mut self, sigil_index: usize) {
        if self.sigil_index == self.sigils.len() - 1 {
            self.sigil_index = 0;
        } else {
            self.sigil_index += 1;
        }
    }

    pub fn prev_sigil(&mut self, sigil_index: usize) {
        if self.sigil_index == 0 {
            self.sigil_index = self.sigils.len() - 1;
        } else {
            self.sigil_index -= 1;
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn go_to(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }

    pub fn glide_to(&mut self, x: i16, y: i16) {}

    pub fn change_x_by(&mut self, x: i16) {
        self.x += x;
    }

    pub fn change_y_by(&mut self, y: i16) {
        self.y += y;
    }
}
