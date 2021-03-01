use crate::game::{Field, GameConfig};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub enhanced_graphics: bool,
    pub click: (u16, u16),
    pub last_reveal: usize,

    pub field: Field,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, config: &GameConfig, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            enhanced_graphics,
            click: (0, 0),
            last_reveal: 0,
            field: Field::new(config),
        }
    }

    // TODO: refactor to handle events?

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'r' => {
                self.field = Field::new(&self.field.config);
            }
            _ => {}
        }
    }

    fn to_field_index(&self, x: u16, y: u16) -> Option<usize> {
        // positions start at 1 + remove the border on the left
        let field_x: isize = (x as isize - 2) / 2;
        // positions start at 1 + remove the border on the top + title
        let field_y: isize = y as isize - 3;

        if field_x >= 0
            && field_x < self.field.config.columns as isize
            && field_y >= 0
            && field_y < self.field.config.rows as isize
        {
            Some(field_y as usize * self.field.config.columns + field_x as usize)
        } else {
            None
        }
    }

    pub fn on_click(&mut self, button: termion::event::MouseButton, x: u16, y: u16) {
        self.set_click(x, y);
        if let Some(index) = self.to_field_index(x, y) {
            self.last_reveal = index;
            match button {
                termion::event::MouseButton::Left => {
                    self.field.reveal_cell(self.last_reveal);
                }
                termion::event::MouseButton::Middle
                | termion::event::MouseButton::WheelDown
                | termion::event::MouseButton::WheelUp => {
                    // self.field.toggle_flag(self.last_reveal);
                }
                _ => {}
            }
        }
    }

    pub fn on_press(&mut self) {
        // 🙂
        // TODO: show a 😵
        // self.field.start_press(x, y)
    }

    pub fn set_click(&mut self, x: u16, y: u16) {
        self.click.0 = x;
        self.click.1 = y;
    }
}
