mod board;
mod cell;
mod game_state;
mod print_board;

use std::io;

use game_state::GameState;
use print_board::print_board;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin().keys();

    let mut game_state = GameState::<4>::new();

    loop {
        print_board(&mut stdout, game_state.board());

        match stdin.next() {
            Some(Ok(Key::Char('q'))) => break,
            Some(Ok(Key::Char('r'))) => game_state = GameState::<4>::new(),
            Some(Ok(Key::Char('u'))) => {
                game_state.undo();
            }
            Some(Ok(Key::Left)) => {
                game_state.left();
            }
            Some(Ok(Key::Right)) => {
                game_state.right();
            }
            Some(Ok(Key::Up)) => {
                game_state.up();
            }
            Some(Ok(Key::Down)) => {
                game_state.down();
            }
            _ => {}
        }
    }
}
