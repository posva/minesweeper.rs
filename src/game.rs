use std::fmt;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use std::collections::HashSet;

#[derive(Clone)]
pub struct GameConfig {
  pub rows: usize,
  pub columns: usize,
  pub mines: usize,
}

impl fmt::Display for GameConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "--- {} x {} - {} 💣  ---",
      self.columns, self.rows, self.mines
    )
  }
}

impl GameConfig {}

pub const CONFIG_BEGINNER: GameConfig = GameConfig {
  rows: 9,
  columns: 9,
  mines: 10,
};

pub const CONFIG_INTERMEDIATE: GameConfig = GameConfig {
  rows: 16,
  columns: 16,
  mines: 40,
};

pub enum FieldCellType {
  Mine,
  Empty(u32),
}

pub struct FieldCell {
  revealed: bool,
  cell_type: FieldCellType,
}

pub struct Field {
  pub config: GameConfig,
  cells: Vec<FieldCell>,
}

impl Field {
  pub fn new(config: &GameConfig) -> Field {
    let mut rng = thread_rng();
    let size = (config.rows * config.columns) as usize;
    let mut field = Field {
      config: config.clone(),
      cells: Vec::with_capacity(size),
    };

    let mut bombs: HashSet<usize> = HashSet::new();

    while bombs.len() < config.mines as usize {
      bombs.insert(rng.gen_range(0, size));
    }

    for i in 0..size {
      field.cells.push(FieldCell {
        revealed: false,
        cell_type: if bombs.contains(&i) {
          FieldCellType::Mine
        } else {
          FieldCellType::Empty(0)
        },
      });
    }

    fn increment_count(field: &mut Field, i: usize) {
      if let Some(cell) = field.cells.get_mut(i) {
        if let FieldCellType::Empty(n) = &mut cell.cell_type {
          *n += 1;
        }
      }
    }

    // increment counters
    for i in 0..size {
      if bombs.contains(&i) {
        // up
        if i > config.columns {
          increment_count(&mut field, i - config.columns);

          // up left
          if i % config.columns > 0 {
            increment_count(&mut field, i - config.columns - 1);
          }

          // up right
          if i / config.rows < config.columns {
            increment_count(&mut field, i - config.columns + 1);
          }
        }

        // down
        if i / config.columns < config.rows - 1 {
          increment_count(&mut field, i + config.columns);

          // up left
          if i % config.columns > 0 {
            increment_count(&mut field, i + config.columns - 1);
          }

          // up right
          if i / config.rows < config.columns {
            increment_count(&mut field, i + config.columns + 1);
          }
        }

        // left
        if i % config.columns > 0 {
          increment_count(&mut field, i - 1);
        }

        // right
        if i / config.rows < config.columns {
          increment_count(&mut field, i + 1);
        }
      }
    }

    field
  }

  pub fn print(&self) {}

  pub fn format(&self, show_all: bool) -> String {
    let mut text = String::from("\n");
    let mut i = 0usize;
    let len = self.cells.len();
    while i < len {
      if i > 0 && i % self.config.rows as usize == 0 {
        // left padding
        text.push_str("\n");
      }
      if i % self.config.columns as usize == 0 {
        // left padding
        text.push_str("  ");
      }

      let cell = self.cells.get(i).unwrap();

      if cell.revealed || show_all {
        match cell.cell_type {
          FieldCellType::Mine => {
            text.push_str("💣");
          }
          FieldCellType::Empty(n) => {
            if n > 0 {
              text.push_str(format!("{} ", n).as_str());
            } else {
              text.push_str("  ");
            };
          }
        }
      } else {
        text.push_str("🔲");
      }

      i += 1;
    }
    text.push('\n');
    text
  }
}

impl fmt::Display for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}\n{}\n--- ---", self.config, self.format(true))
  }
}
