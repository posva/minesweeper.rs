use std::fmt;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use std::collections::HashSet;
use std::iter::FromIterator;

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
    mines: HashSet<usize>,
}

impl Field {
    pub fn new(config: &GameConfig) -> Field {
        let mut rng = thread_rng();
        let size = (config.rows * config.columns) as usize;
        let mut field = Field {
            config: config.clone(),
            cells: Vec::with_capacity(size),
            mines: HashSet::new(),
        };

        while field.mines.len() < config.mines {
            field.mines.insert(rng.gen_range(0, size));
        }

        for i in 0..size {
            field.cells.push(FieldCell {
                revealed: false,
                cell_type: if field.mines.contains(&i) {
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
        // println!("{}", self.as_text_ascii(true));
        // println!("===");
    }

    /**
     * Compute the field (number of mines) based on the current config. Should only be called once
     */
    fn compute_field(&mut self) {
        // increment counters
        for i in 0..self.cells.len() {
            if self.mines.contains(&i) {
                let has_left = i % self.config.columns > 0;
                let has_right = i % self.config.columns < self.config.columns - 1;
                // up
                if i > self.config.columns {
                    let up_pos = i - self.config.columns;
                    self.increment_cell_count(up_pos);

                    // up left
                    if has_left {
                        self.increment_cell_count(up_pos - 1);
                    }

                    // up right
                    if has_right {
                        self.increment_cell_count(up_pos + 1);
                    }
                }

                // down
                if i / self.config.columns < self.config.rows - 1 {
                    let down_pos = i + self.config.columns;
                    self.increment_cell_count(down_pos);

                    // down left
                    if has_left {
                        self.increment_cell_count(down_pos - 1);
                    }

                    // down right
                    if has_right {
                        self.increment_cell_count(down_pos + 1);
                    }
                }

                // left
                if has_left {
                    self.increment_cell_count(i - 1);
                }

                // right
                if has_right {
                    self.increment_cell_count(i + 1);
                }
            }
        }
    }

    pub fn reveal_cell(&mut self, pos: usize) -> bool {
        if let Some(cell) = self.cells.get_mut(pos) {
            if cell.revealed {
                return false;
            }
            cell.revealed = true;
            match cell.cell_type {
                FieldCellType::Mine => return true,
                FieldCellType::Empty(n) => {
                    if n == 0 {
                        // reveal others
                        let has_left = pos % self.config.columns > 0;
                        let has_right = pos % self.config.columns < self.config.columns - 1;
                        if pos > self.config.columns {
                            let up_pos = pos - self.config.columns;
                            self.reveal_cell(up_pos);

                            // up left
                            if has_left {
                                self.reveal_cell(up_pos - 1);
                            }

                            // up right
                            if has_right {
                                self.reveal_cell(up_pos + 1);
                            }
                        }

                        // down
                        if pos / self.config.columns < self.config.rows - 1 {
                            let down_pos = pos + self.config.columns;
                            self.reveal_cell(down_pos);

                            // down left
                            if has_left {
                                self.reveal_cell(down_pos - 1);
                            }

                            // down right
                            if has_right {
                                self.reveal_cell(down_pos + 1);
                            }
                        }

                        // left
                        if has_left {
                            self.reveal_cell(pos - 1);
                        }

                        // right
                        if has_right {
                            self.reveal_cell(pos + 1);
                        }
                    }
                }
            }
        }

        false
    }

    /**
     * Create a field mine from a vec of strings. Used for tests.
     */
    pub fn from(field_text: Vec<&str>) -> Field {
        let config = GameConfig {
            rows: field_text.len(),
            columns: field_text.get(0).unwrap().len(),
            mines: field_text.iter().map(|c| c.matches("x").count()).sum(),
        };
        let size = config.rows * config.columns;

        let mut field = Field {
            config,
            cells: Vec::with_capacity(size),
            mines: HashSet::new(),
        };

        for (y, line) in field_text.iter().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                if cell == 'x' {
                    field.mines.insert(x + y * field.config.columns);
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

    pub fn format(&self, show_all: bool) -> String {
        let mut text = String::from("\n");
        let mut i = 0usize;
        let len = self.cells.len();
        while i < len {
            if i > 0 && i % self.config.columns == 0 {
                // left padding
                text.push_str("\n");
            }
            if i % self.config.columns == 0 {
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
            if i > 0 && i % self.config.columns == 0 {
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
                            text.push('-');
                        };
                    }
                }
            } else {
                text.push('?');
            }

            i += 1;
        }

        text
    }

    pub fn as_lines(&self, show_all: bool) -> Vec<String> {
        let mut i: usize = 0;
        let len = self.cells.len();
        let mut lines = Vec::new();
        // line we are building
        let mut line_buffer = String::new();
        while i < len {
            if i > 0 && i % self.config.columns == 0 {
                // new line
                lines.push(line_buffer);
                line_buffer = String::new();
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

        // last line
        lines.push(line_buffer);

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
        let vec: Vec<usize> = vec![7, 12];
        assert_eq!(
            field.mines,
            HashSet::from_iter(vec.into_iter().collect::<Vec<_>>())
        );
    }

    #[test]
    fn correct_numbers_simple() {
        let field = Field::from(vec!["ooooo", "ooxoo", "ooxoo", "ooooo"]);

        assert_eq!(
            field.as_text_ascii(true),
            "\
-111-
-2x2-
-2x2-
-111-\
"
        );
    }

    #[test]
    fn field_edge_cases() {
        let field = Field::from(vec!["xoxox", "ooooo", "xooox", "xoxox"]);

        assert_eq!(
            field.as_text_ascii(true),
            "\
x2x2x
23132
x313x
x3x3x\
"
        );
    }

    #[test]
    fn print_emojis() {
        let field = Field::from(vec!["oxxxo", "oxoxo", "oxxxo", "xoooo"]);

        assert_eq!(
            field.as_lines(true),
            vec!["2 💣💣💣2 ", "3 💣8 💣3 ", "3 💣💣💣2 ", "💣3 3 2 1 "]
        );
    }

    #[test]
    fn reveal_cells() {
        let mut field = Field::from(vec!["oxxxo", "oxoxo", "ooooo", "xoooo"]);

        println!("{}", field.as_text_ascii(true));

        assert_eq!(
            field.as_text_ascii(false),
            "\
?????
?????
?????
?????\
"
        );

        assert_eq!(field.reveal_cell(0), false);

        assert_eq!(
            field.as_text_ascii(false),
            "\
2????
?????
?????
?????\
"
        );

        assert_eq!(
            field.reveal_cell(field.config.columns * field.config.rows - 1),
            false
        );

        assert_eq!(
            field.as_text_ascii(false),
            "\
2????
?????
?2211
?1---\
"
        );

        assert_eq!(field.reveal_cell(1), true);

        assert_eq!(
            field.as_text_ascii(false),
            "\
2x???
?????
?2211
?1---\
"
        );
    }
}
