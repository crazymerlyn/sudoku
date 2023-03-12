#![allow(dead_code)]

extern crate rand;
extern crate termion;

use std::io;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

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
