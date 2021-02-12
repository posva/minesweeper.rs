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
    bombs: HashSet<usize>,
}

impl Field {
    pub fn new(config: &GameConfig) -> Field {
        let mut rng = thread_rng();
        let size = (config.rows * config.columns) as usize;
        let mut field = Field {
            config: config.clone(),
            cells: Vec::with_capacity(size),
            bombs: HashSet::new(),
        };

        while field.bombs.len() < config.mines {
            field.bombs.insert(rng.gen_range(0, size));
        }

        for i in 0..size {
            field.cells.push(FieldCell {
                revealed: false,
                cell_type: if field.bombs.contains(&i) {
                    FieldCellType::Mine
                } else {
                    FieldCellType::Empty(0)
                },
            });
        }

        field.compute_field();

        field
    }

    fn increment_cell_count(&mut self, i: usize) {
        if let Some(cell) = self.cells.get_mut(i) {
            if let FieldCellType::Empty(n) = &mut cell.cell_type {
                *n += 1;
            }
        }
    }

    fn compute_field(&mut self) {
        // increment counters
        for i in 0..self.cells.len() {
            if self.bombs.contains(&i) {
                // up
                if i > self.config.columns {
                    self.increment_cell_count(i - self.config.columns);

                    // up left
                    if i % self.config.columns > 0 {
                        self.increment_cell_count(i - self.config.columns - 1);
                    }

                    // up right
                    if i / self.config.rows < self.config.columns {
                        self.increment_cell_count(i - self.config.columns + 1);
                    }
                }

                // down
                if i / self.config.columns < self.config.rows - 1 {
                    self.increment_cell_count(i + self.config.columns);

                    // up left
                    if i % self.config.columns > 0 {
                        self.increment_cell_count(i + self.config.columns - 1);
                    }

                    // up right
                    if i / self.config.rows < self.config.columns {
                        self.increment_cell_count(i + self.config.columns + 1);
                    }
                }

                // left
                if i % self.config.columns > 0 {
                    self.increment_cell_count(i - 1);
                }

                // right
                if i / self.config.rows < self.config.columns {
                    self.increment_cell_count(i + 1);
                }
            }
        }
    }

    pub fn from(field_text: Vec<&str>) -> Field {
        let config = GameConfig {
            rows: field_text.len(),
            columns: field_text.get(0).unwrap().len(),
            mines: field_text.iter().map(|c| c.matches("x").count()).sum(),
        };
        let size = (config.rows * config.columns);

        let mut field = Field {
            config,
            cells: Vec::with_capacity(size),
            bombs: HashSet::new(),
        };

        for line in field_text.iter() {
            for (i, cell) in line.chars().enumerate() {
                if cell == 'x' {
                    field.bombs.insert(i);
                }
                field.cells.push(FieldCell {
                    revealed: false,
                    cell_type: if cell == 'x' {
                        FieldCellType::Mine
                    } else {
                        FieldCellType::Empty(0)
                    },
                });
            }
        }

        field.compute_field();

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

    pub fn as_text_ascii(&self, show_all: bool) -> String {
        let mut i = 0usize;
        let len = self.cells.len();
        let mut text = String::new();
        while i < len {
            if i > 0 && i % self.config.rows == 0 {
                // new line
                text.push('\n');
            }

            let cell = self.cells.get(i).unwrap();

            if cell.revealed || show_all {
                match cell.cell_type {
                    FieldCellType::Mine => {
                        text.push('x');
                    }
                    FieldCellType::Empty(n) => {
                        if n > 0 {
                            text.push_str(&n.to_string());
                        } else {
                            text.push(' ');
                        };
                    }
                }
            } else {
                text.push(' ');
            }

            i += 1;
        }

        text
    }

    pub fn as_lines(&self, show_all: bool) -> Vec<String> {
        let mut text = String::from("\n");
        let mut i = 0usize;
        let len = self.cells.len();
        let mut lines = Vec::new();
        // line we are building
        let mut line_buffer = String::new();
        while i < len {
            if i > 0 && i % self.config.rows as usize == 0 {
                // new line
                lines.push(line_buffer);
                line_buffer = String::new();
                // text.push_str("\n");
            }
            if i % self.config.columns as usize == 0 {
                // left padding
                text.push_str("  ");
            }

            let cell = self.cells.get(i).unwrap();

            if cell.revealed || show_all {
                match cell.cell_type {
                    FieldCellType::Mine => {
                        line_buffer.push_str("💣");
                    }
                    FieldCellType::Empty(n) => {
                        if n > 0 {
                            line_buffer.push_str(format!("{} ", n).as_str());
                        } else {
                            line_buffer.push_str("  ");
                        };
                    }
                }
            } else {
                line_buffer.push_str("🔲");
            }

            i += 1;
        }

        lines
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n--- ---", self.config, self.format(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_field() {
        let field = Field::new(&CONFIG_BEGINNER);

        assert_eq!(
            field.cells.len(),
            CONFIG_BEGINNER.columns * CONFIG_BEGINNER.rows
        );
    }

    #[test]
    fn from_parsing() {
        let field = Field::from(vec!["ooooo", "ooxoo", "ooxoo", "ooooo"]);

        assert_eq!(field.config.mines, 2);
        assert_eq!(field.config.columns, 5);
        assert_eq!(field.config.rows, 4);
    }

    #[test]
    fn correct_numbers_simple() {
        let field = Field::from(vec!["ooooo", "ooxoo", "ooxoo", "ooooo"]);

        assert_eq!(
            field.as_text_ascii(true),
            "\
 111
 2x2
 2x2
 111\
"
        );
    }
}
