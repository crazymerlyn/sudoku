use std::convert::Into;
use std::fmt;

use rand::{random, thread_rng, Rng};

use super::Grid;

const VERY_EASY: &'static str = include_str!("./seeds/veasy.csv");
const EASY: &'static str = include_str!("./seeds/easy.csv");
const MEDIUM: &'static str = include_str!("./seeds/medium.csv");
const HARD: &'static str = include_str!("./seeds/hard.csv");
const FIENDISH: &'static str = include_str!("./seeds/fiendish.csv");

pub struct Generator {}

impl Generator {
    pub fn generate<T: Into<Difficulty>>(diff: T) -> Grid {
        let puzzles_str = diff.into().puzzles();
        let puzzles = read_puzzles(puzzles_str);

        let mut puzzle = puzzles[random::<usize>() % puzzles.len()].clone();

        let mut permutation: Vec<_> = (1..10).collect();
        thread_rng().shuffle(&mut permutation);
        puzzle.permute(permutation);

        if random() {
            puzzle.flip_horizontally();
        }

        if random() {
            puzzle.flip_vertically();
        }

        puzzle
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    Fiendish,
}

impl Difficulty {
    pub fn puzzles(&self) -> &'static str {
        match *self {
            Difficulty::VeryEasy => VERY_EASY,
            Difficulty::Easy => EASY,
            Difficulty::Medium => MEDIUM,
            Difficulty::Hard => HARD,
            Difficulty::Fiendish => FIENDISH,
        }
    }
}

impl<'a> From<&'a str> for Difficulty {
    fn from(s: &'a str) -> Difficulty {
        match s.to_lowercase().as_str() {
            "very easy" => Difficulty::VeryEasy,
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            "fiendish" => Difficulty::Fiendish,
            _ => panic!("Unknown Difficulty: {}", s),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_rep = match *self {
            Difficulty::VeryEasy => "Very Easy",
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Fiendish => "Fiendish",
        };
        write!(f, "{}", str_rep)
    }
}

fn read_puzzles(puzzles_str: &'static str) -> Vec<Grid> {
    let mut puzzles = vec![];
    let lines: Vec<_> = puzzles_str.lines().collect();

    for i in 0..lines.len() / 9 {
        puzzles.push(Grid::from_csv(lines[i * 9..i * 9 + 9].join("\n").as_str()));
    }

    puzzles
}
