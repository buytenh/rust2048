use crate::board::{Board, BoardMove};

pub struct GameState<const N: usize> {
    pub board: Board<N>,
    pub prev_boards: Vec<Board<N>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameStateAction {
    Restart,
    Move(BoardMove),
    Undo,
}

impl<const N: usize> GameState<N> {
    pub fn new() -> Self {
        let board = Board::<N>::new();

        GameState {
            board,
            prev_boards: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board<N> {
        &self.board
    }

    pub fn do_action(&mut self, game_action: GameStateAction) -> bool {
        match game_action {
            GameStateAction::Restart => {
                self.board = Board::<N>::new();
                self.prev_boards = Vec::new();
                true
            }
            GameStateAction::Move(board_move) => match self.board.do_move(board_move) {
                None => false,
                Some(newboard) => {
                    self.prev_boards.push(self.board);
                    self.board = newboard;

                    true
                }
            },
            GameStateAction::Undo => match self.prev_boards.pop() {
                None => false,
                Some(board) => {
                    self.board = board;
                    true
                }
            },
        }
    }
}
