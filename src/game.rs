use std::fmt;
use std::fmt::format;

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

pub const CONFIG_EXPERT: GameConfig = GameConfig {
    rows: 16,
    columns: 30,
    mines: 99,
};

pub enum FieldCellType {
    Mine,
    Empty(u32), // TODO: can we just use u8?
}

pub enum FieldCellState {
    Hidden,
    Revealed,
    Flagged,
}

pub struct FieldCell {
    state: FieldCellState,
    cell_type: FieldCellType,
}

impl FieldCell {
    pub fn as_unicode_str(&self, force_reveal: bool) -> String {
        if force_reveal {
            self.as_revealed_str()
        } else {
            match self.state {
                FieldCellState::Hidden => String::from("🔲"),
                _ => self.as_revealed_str(),
            }
        }
    }

    pub fn as_number(&self, force_reveal: bool) -> u32 {
        if force_reveal {
            self.as_revealed_number()
        } else {
            match self.state {
                FieldCellState::Hidden => 10,
                _ => self.as_revealed_number(),
            }
        }
    }

    fn as_ascii_str(&self, force_reveal: bool) -> String {
        if force_reveal {
            self.as_revealed_ascii_str()
        } else {
            match self.state {
                FieldCellState::Hidden => String::from("?"),
                _ => self.as_revealed_ascii_str(),
            }
        }
    }

    pub fn as_revealed_number(&self) -> u32 {
        match self.state {
            FieldCellState::Flagged => 8,
            _ => match self.cell_type {
                FieldCellType::Mine => 9,
                FieldCellType::Empty(mines) => mines,
            },
        }
    }

    pub fn as_revealed_ascii_str(&self) -> String {
        match self.state {
            FieldCellState::Flagged => String::from("f"),
            _ => match self.cell_type {
                FieldCellType::Mine => String::from("x"),
                FieldCellType::Empty(mines) => {
                    if mines == 0 {
                        String::from("-")
                    } else {
                        format!("{}", mines)
                    }
                }
            },
        }
    }

    pub fn as_revealed_str(&self) -> String {
        match self.state {
            FieldCellState::Flagged => String::from("🚩"),
            _ => match self.cell_type {
                FieldCellType::Mine => String::from("💣"),
                FieldCellType::Empty(mines) => {
                    let codepoint = 0x245f + mines as u16;
                    String::from_utf16(&[codepoint]).unwrap()
                }
            },
        }
    }
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
                state: FieldCellState::Hidden,
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

    pub fn toggle_flag(&mut self, pos: usize) {
        if let Some(cell) = self.cells.get_mut(pos) {
            cell.state = match cell.state {
                FieldCellState::Flagged => FieldCellState::Hidden,
                _ => FieldCellState::Flagged,
            }
        }
    }

    pub fn reveal_cell(&mut self, pos: usize) -> bool {
        if let Some(cell) = self.cells.get_mut(pos) {
            match cell.state {
                // a flagged cell cannot be revealed when clicked on
                // FieldCellState::Flagged => return false,
                FieldCellState::Revealed => return false,
                _ => {
                    cell.state = FieldCellState::Revealed;
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
                    state: FieldCellState::Hidden,
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

            text.push_str(&cell.as_unicode_str(show_all));

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

            text.push_str(&cell.as_ascii_str(show_all));

            i += 1;
        }

        text
    }

    pub fn get_field(&self) -> Vec<Vec<u32>> {
        let mut i: usize = 0;
        let len = self.cells.len();
        let mut lines = Vec::new();

        let mut line_buffer = Vec::new();

        // show all if lost
        let show_all = false;

        while i < len {
            if i > 0 && i % self.config.columns == 0 {
                // new line
                lines.push(line_buffer);
                line_buffer = Vec::new();
            }

            let cell = self.cells.get(i).unwrap();

            line_buffer.push(cell.as_number(show_all));

            i += 1;
        }

        // last line
        lines.push(line_buffer);

        lines
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

            line_buffer.push_str(&cell.as_unicode_str(show_all));

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
            vec!["②💣💣💣②", "③💣⑧💣③", "③💣💣💣②", "💣③③②①"]
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
