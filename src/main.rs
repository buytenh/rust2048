mod board;
mod cell;
mod game_state;
mod print_board;

use std::{fs, io, str, thread::sleep, time::Duration};

use board::{Board, BoardMove};
use game_state::{GameState, GameStateAction};
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
            Some(Ok(Key::Char('r'))) => {
                game_state.do_action(GameStateAction::Reroll);
            }
            Some(Ok(Key::Char('u'))) => {
                game_state.do_action(GameStateAction::Undo);
            }
            Some(Ok(Key::Char('s'))) => {
                let _ = fs::write("game.json", game_state.board().serialize());
            }
            Some(Ok(Key::Char('l'))) => match fs::read("game.json") {
                Ok(data) => match str::from_utf8(&data) {
                    Ok(board) => match Board::deserialize(board) {
                        Ok(board) => {
                            game_state = GameState {
                                board,
                                prev_boards: Vec::new(),
                            }
                        }
                        Err(err) => {
                            eprintln!("deserialization error: {}", err);
                            sleep(Duration::from_secs(2));
                        }
                    },
                    Err(err) => {
                        eprintln!("UTF-8 decoding error: {}", err);
                        sleep(Duration::from_secs(2));
                    }
                },
                Err(err) => {
                    eprintln!("save game reading error: {}", err);
                    sleep(Duration::from_secs(2));
                }
            },
            Some(Ok(Key::Char('n'))) => {
                game_state.do_action(GameStateAction::Restart);
            }
            Some(Ok(Key::Left)) => {
                game_state.do_action(GameStateAction::Move(BoardMove::Left));
            }
            Some(Ok(Key::Right)) => {
                game_state.do_action(GameStateAction::Move(BoardMove::Right));
            }
            Some(Ok(Key::Up)) => {
                game_state.do_action(GameStateAction::Move(BoardMove::Up));
            }
            Some(Ok(Key::Down)) => {
                game_state.do_action(GameStateAction::Move(BoardMove::Down));
            }
            _ => {}
        }
    }
}
