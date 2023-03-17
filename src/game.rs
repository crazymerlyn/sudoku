use grid::generator::Difficulty;
use grid::generator::Generator;
use grid::Direction;
use grid::Grid;

use termion;
use termion::clear;
use termion::cursor;
use termion::event::*;
use termion::input::{Events, TermRead};
use termion::style;

use std::io::{Read, Write};

pub struct Game<R, W: Write> {
    grid: Grid,
    stdout: W,
    events: Events<R>,
}

impl<R, W: Write> Drop for Game<R, W> {
    fn drop(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1),
            cursor::Show
        )
        .unwrap();
    }
}

impl<R: TermRead + Read, W: Write> Game<R, W> {
    pub fn new(stdin: R, stdout: W) -> Game<R, W> {
        Game {
            grid: Generator::generate("very easy"),
            events: stdin.events(),
            stdout,
        }
    }

    pub fn init(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            cursor::Hide,
            clear::All,
            cursor::Goto(1, 1)
        )
        .unwrap();
    }

    /// Returns None if user decided to quit instead
    pub fn get_difficulty(&mut self) -> Option<Difficulty> {
        let difficulties = [
            Difficulty::VeryEasy,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
            Difficulty::Fiendish,
        ];
        let mut current_index = 0;

        loop {
            let (w, h) = termion::terminal_size().unwrap();
            let top = (h - 6) / 2;
            let left = (w - 20) / 2;

            write!(self.stdout, "{}", clear::All).unwrap();
            write!(
                self.stdout,
                "{}Choose a difficulty:",
                cursor::Goto(left, top - 1)
            )
            .unwrap();

            for (i, diff) in difficulties.iter().enumerate() {
                if i == current_index {
                    write!(self.stdout, "{}>>", cursor::Goto(left + 2, top + i as u16)).unwrap();
                }
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto(left + 5, top + i as u16),
                    diff
                )
                .unwrap();
            }
            self.stdout.flush().unwrap();

            let evt = self.events.next().unwrap().unwrap();

            if let Event::Key(key) = evt {
                match key {
                    Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => return None,
                    Key::Down | Key::Char('j') | Key::Char('s') => {
                        current_index += 1;
                    }
                    Key::Up | Key::Char('k') | Key::Char('w') => {
                        current_index = if current_index > 0 {
                            current_index - 1
                        } else {
                            difficulties.len() - 1
                        };
                    }
                    Key::Char('\n') => return Some(difficulties[current_index]),
                    _ => {}
                }
            }
        }
    }

    pub fn run(&mut self) {
        self.init();

        let diff = match self.get_difficulty() {
            Some(val) => val,
            None => {
                return;
            }
        };

        self.grid = Generator::generate(diff);

        let (w, h) = termion::terminal_size().unwrap();
        let top = (h - 19) / 2;
        let left = (w - 37) / 2;
        let mut message = "".to_string();

        writeln!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(left, top),
            self.grid
        )
        .unwrap();
        loop {
            let evt = self.events.next().unwrap().unwrap();

            let (w, h) = termion::terminal_size().unwrap();
            let top = (h - 19) / 2;
            let left = (w - 37) / 2;

            if let Event::Key(key) = evt {
                match key {
                    Key::Right | Key::Char('d') | Key::Char('l') => {
                        self.grid.move_cursor(Direction::Right);
                    }
                    Key::Left | Key::Char('a') | Key::Char('h') => {
                        self.grid.move_cursor(Direction::Left);
                    }
                    Key::Up | Key::Char('w') | Key::Char('k') => {
                        self.grid.move_cursor(Direction::Up);
                    }
                    Key::Down | Key::Char('s') | Key::Char('j') => {
                        self.grid.move_cursor(Direction::Down);
                    }
                    Key::Char(' ') | Key::Backspace => self.grid.update_current(0),
                    Key::Ctrl('r') => self.grid.redo(),
                    Key::Char(ch) => match ch {
                        'q' => return,
                        'u' => self.grid.undo(),
                        'r' => self.grid.remove_filled(),
                        'n' => {
                            self.run();
                            return;
                        }
                        ch if ch.is_digit(10) => {
                            self.grid.update_current(ch as usize - '0' as usize)
                        }
                        _ => {}
                    },
                    Key::Ctrl('c') | Key::Ctrl('d') => break,
                    _ => {}
                };
                if self.grid.is_solved() {
                    self.grid.freeze();
                    message = "You finished the puzzle! Press n to start a new one or q to quit"
                        .to_string();
                }
            }
            writeln!(
                self.stdout,
                "{}{}{}",
                clear::All,
                cursor::Goto(left, top),
                self.grid
            )
            .unwrap();
            writeln!(self.stdout, "{}{}", cursor::Goto(left, top + 20), message).unwrap();
        }
    }
}
