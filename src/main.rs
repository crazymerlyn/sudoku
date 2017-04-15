#![allow(dead_code)]

extern crate termion;
extern crate rand;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use std::io;

mod grid;

mod game;
use game::Game;

fn main() {
    let stdin = io::stdin();
    let screen = AlternateScreen::from(io::stdout().into_raw_mode().unwrap());
    let stdout = MouseTerminal::from(screen);

    let mut game = Game::new(stdin, stdout);

    game.run();
}
