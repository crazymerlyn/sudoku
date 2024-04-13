use termion::color;
use termion::cursor;
use termion::style;

use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;
use std::ops::Index;

pub mod square;
pub use self::square::Square;

pub mod generator;

const BORDER_COLOR: color::Fg<color::Rgb> = color::Fg(color::Rgb(220, 220, 220));

const BORDER_TOP: &str = "┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓";
const BORDER_BOTTOM: &str = "┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛";
const BORDER_HORIZONTAL_THIN: &str = "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨";
const BORDER_HORIZONTAL_THICK: &str = "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫";
const BORDER_VERTICAL_THICK: &str = "┃";
const BORDER_VERTICAL_THIN: &str = "│";

#[derive(Debug, Clone)]
struct GridState {
    squares: [[Square; 9]; 9],
    current: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct Grid {
    state: GridState,
    past: Vec<GridState>,
    future: Vec<GridState>,
}

impl Grid {
    pub fn new(values: [[u8; 9]; 9]) -> Self {
        Grid {
            state: GridState::new(values),
            past: vec![],
            future: vec![],
        }
    }

    pub fn from_csv(csv: &str) -> Self {
        Grid {
            state: GridState::from_csv(csv),
            past: vec![],
            future: vec![],
        }
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        self.past.push(self.state.clone());
        self.state.move_cursor(dir);
        self.future = vec![];
    }

    pub fn update_current(&mut self, d: usize) {
        self.past.push(self.state.clone());
        self.state.update_current(d);
        self.future = vec![];
    }

    pub fn freeze(&mut self) {
        self.state.freeze();
    }

    pub fn permute(&mut self, permutation: &[u8]) {
        self.state.permute(permutation);
    }

    pub fn flip_horizontally(&mut self) {
        self.state.flip_horizontally();
    }

    pub fn flip_vertically(&mut self) {
        self.state.flip_vertically();
    }

    pub fn row(&self, row: usize) -> Vec<Square> {
        self.state.row(row)
    }

    pub fn col(&self, col: usize) -> Vec<Square> {
        self.state.col(col)
    }

    pub fn block(&self, y: usize, x: usize) -> Vec<Square> {
        self.state.block(y, x)
    }

    pub fn remove_filled(&mut self) {
        self.past.push(self.state.clone());
        self.state.remove_filled();
        self.future = vec![];
    }

    pub fn undo(&mut self) {
        if let Some(state) = self.past.pop() {
            self.future.push(self.state.clone());
            self.state = state;
        }
    }

    pub fn redo(&mut self) {
        if let Some(state) = self.future.pop() {
            self.past.push(self.state.clone());
            self.state = state;
        }
    }

    pub fn is_solved(&self) -> bool {
        self.state.is_solved()
    }

    /// Check the grid for inaccuracies
    /// and return the problem square locations
    pub fn find_invalid_squares(&self) -> HashSet<(usize, usize)> {
        self.state.find_invalid_squares()
    }
}

impl GridState {
    pub fn new(values: [[u8; 9]; 9]) -> Self {
        let mut squares = [[Square::Empty; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                if values[i][j] == 0 {
                    squares[i][j] = Square::Empty;
                } else {
                    squares[i][j] = Square::initial(values[i][j]);
                }
            }
        }

        GridState {
            squares,
            current: (0, 0),
        }
    }

    pub fn from_csv(csv: &str) -> Self {
        let mut values = [[0; 9]; 9];
        let mut i = 0;
        let mut j = 0;
        for c in csv.chars() {
            if c == ',' {
                j += 1;
            } else if c == '\n' {
                i += 1;
                j = 0;
            } else if c.is_ascii_digit() {
                values[i][j] = c as u8 - b'0';
            } else {
                panic!("Unknown character in csv: {}", c);
            }
        }

        GridState::new(values)
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        let (i, j) = dir.coords();
        let (ci, cj) = self.current;

        self.current = ((ci + i) % 9, (cj + j) % 9);
    }

    pub fn update_current(&mut self, d: usize) {
        let (i, j) = self.current;
        let sq = &mut self.squares[i][j];

        if !sq.is_initial() {
            *sq = Square::from_value(d as u8);
        }
    }

    pub fn freeze(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                self.squares[i][j] = Square::initial(self.squares[i][j].value());
            }
        }
    }

    pub fn permute(&mut self, permutation: &[u8]) {
        assert_eq!(permutation.len(), 9);
        assert!((1..10).all(|n| permutation.contains(&n)));

        for i in 0..9 {
            for j in 0..9 {
                if !self.squares[i][j].is_empty() {
                    let value = self.squares[i][j].value();
                    self.squares[i][j] = Square::initial(permutation[(value - 1) as usize]);
                }
            }
        }
    }

    pub fn flip_horizontally(&mut self) {
        for i in 0..9 {
            self.squares[i].reverse();
        }
    }

    pub fn flip_vertically(&mut self) {
        self.squares.reverse();
    }

    pub fn row(&self, row: usize) -> Vec<Square> {
        self.squares[row].to_vec()
    }

    pub fn col(&self, col: usize) -> Vec<Square> {
        let mut column = Vec::with_capacity(9);
        for i in 0..9 {
            column.push(self.squares[i][col]);
        }
        column
    }

    pub fn block(&self, y: usize, x: usize) -> Vec<Square> {
        let mut block = Vec::with_capacity(9);

        for i in y * 3..(y + 1) * 3 {
            for j in x * 3..(x + 1) * 3 {
                block.push(self.squares[i][j]);
            }
        }

        block
    }

    pub fn remove_filled(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if !self.squares[i][j].is_initial() {
                    self.squares[i][j] = Square::Empty;
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.squares[i][j].is_empty() {
                    return false;
                }
            }
        }

        self.find_invalid_squares().is_empty()
    }

    /// Check the grid for inaccuracies
    /// and return the problem square locations
    pub fn find_invalid_squares(&self) -> HashSet<(usize, usize)> {
        let mut set = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                let value = self.squares[i][j].value();
                if value != 0 {
                    // Check the column
                    for i2 in i + 1..9 {
                        if self.squares[i2][j].value() == value {
                            set.insert((i, j));
                            set.insert((i2, j));
                        }
                    }

                    // Check the row
                    for j2 in j + 1..9 {
                        if self.squares[i][j2].value() == value {
                            set.insert((i, j));
                            set.insert((i, j2));
                        }
                    }

                    // Check the square
                    for i2 in prev_multiple(3, i)..next_multiple(3, i) {
                        for j2 in prev_multiple(3, j)..next_multiple(3, j) {
                            if i2 == i && j2 == j {
                                continue;
                            };
                            if self.squares[i2][j2].value() == value {
                                set.insert((i, j));
                                set.insert((i2, j2));
                            }
                        }
                    }
                }
            }
        }

        set
    }
}

impl Index<usize> for Grid {
    type Output = [Square; 9];

    fn index(&self, idx: usize) -> &[Square; 9] {
        self.state.index(idx)
    }
}

impl Index<usize> for GridState {
    type Output = [Square; 9];

    fn index(&self, idx: usize) -> &[Square; 9] {
        &self.squares[idx]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        self.state.fmt(ff)
    }
}

impl fmt::Display for GridState {
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        let mistakes = self.find_invalid_squares();
        let mut f = String::new();

        write!(f, "{BORDER_COLOR}{BORDER_TOP}")?;
        write!(f, "{}{}", cursor::Down(1), cursor::Left(37))?;
        for i in 0..9 {
            write!(f, "{BORDER_COLOR}{BORDER_VERTICAL_THICK}")?;
            for j in 0..9 {
                let st = if (i, j) == self.current {
                    format!("{}", style::Invert)
                } else {
                    String::new()
                };

                let nt = if (i, j) == self.current {
                    format!("{}", style::NoInvert)
                } else {
                    String::new()
                };

                let fg = if mistakes.contains(&(i, j)) {
                    format!("{}", color::Fg(color::Red))
                } else if self.squares[i][j].is_initial() {
                    format!("{}", color::Fg(color::Cyan))
                } else {
                    format!("{}", color::Fg(color::White))
                };

                write!(f, " {}{}{}{} ", st, fg, self.squares[i][j], nt)?;
                write!(f, "{BORDER_COLOR}")?;
                if j % 3 == 2 {
                    write!(f, "{BORDER_VERTICAL_THICK}")?;
                } else {
                    write!(f, "{BORDER_VERTICAL_THIN}")?;
                }
            }
            write!(f, "{}{}", cursor::Down(1), cursor::Left(37))?;
            write!(f, "{BORDER_COLOR}")?;
            if i == 8 {
                write!(f, "{BORDER_BOTTOM}")?;
            } else if i % 3 == 2 {
                write!(f, "{BORDER_HORIZONTAL_THICK}")?;
            } else {
                write!(f, "{BORDER_HORIZONTAL_THIN}")?;
            }
            write!(f, "{}{}", cursor::Down(1), cursor::Left(37))?;
        }
        write!(ff, "{f}")
    }
}

fn prev_multiple(a: usize, b: usize) -> usize {
    b - (b % a)
}

fn next_multiple(a: usize, b: usize) -> usize {
    b + a - (b % a)
}

#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn coords(self) -> (usize, usize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, 8),
            Direction::Up => (8, 0),
            Direction::Down => (1, 0),
        }
    }
}
