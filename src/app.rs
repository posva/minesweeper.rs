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

    pub fn on_click(&mut self, x: u16, y: u16) {
        self.set_click(x, y);
        // positions start at 1 + remove the border on the left
        let field_x: usize = (x as usize - 2) / 2;
        // positions start at 1 + remove the border on the top + title
        let field_y: usize = y as usize - 3;

        self.last_reveal = field_y * self.field.config.columns + field_x;
        self.field.reveal_cell(self.last_reveal);
    }

    pub fn set_click(&mut self, x: u16, y: u16) {
        self.click.0 = x;
        self.click.1 = y;
    }
}
