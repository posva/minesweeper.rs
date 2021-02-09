use crate::game::{Field, GameConfig};

pub struct App<'a> {
  pub title: &'a str,
  pub should_quit: bool,
  pub enhanced_graphics: bool,
  pub click: (u16, u16),

  pub field: Field,
}

impl<'a> App<'a> {
  pub fn new(title: &'a str, config: &GameConfig, enhanced_graphics: bool) -> App<'a> {
    App {
      title,
      should_quit: false,
      enhanced_graphics,
      click: (0, 0),
      field: Field::new(config),
    }
  }

  // TODO: refactor to handle events?

  pub fn on_key(&mut self, c: char) {
    match c {
      'q' => {
        self.should_quit = true;
      }
      _ => {}
    }
  }

  pub fn set_click(&mut self, x: u16, y: u16) {
    self.click.0 = x;
    self.click.1 = y;
  }
}
