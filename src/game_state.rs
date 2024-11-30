use crate::board::{Board, BoardMove};

pub struct GameState<const N: usize> {
    pub board: Board<N>,
    pub prev_boards: Vec<Board<N>>,
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

    pub fn do_move(&mut self, board_move: BoardMove) -> bool {
        match self.board.do_move(board_move) {
            None => false,
            Some(newboard) => {
                self.prev_boards.push(self.board);
                self.board = newboard;

                true
            }
        }
    }

    pub fn undo(&mut self) -> bool {
        match self.prev_boards.pop() {
            None => false,
            Some(board) => {
                self.board = board;
                true
            }
        }
    }
}
