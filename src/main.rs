#![allow(dead_code)]

extern crate fastrand;
extern crate termion;

use std::io;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

mod grid;

mod game;
use game::Game;

fn main() {
    let stdin = io::stdin();
    let screen = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let stdout = MouseTerminal::from(screen);

    let mut game = Game::new(stdin, stdout);

    game.run();
}
