use crate::board::Board;

pub struct GameState<const N: usize> {
    board: Board<N>,
    prev_boards: Vec<Board<N>>,
}

impl<const N: usize> GameState<N> {
    pub fn new() -> Self {
        let mut board = Board::<N>::new();

        board.insert_random().unwrap();
        board.insert_random().unwrap();

        GameState {
            board,
            prev_boards: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board<N> {
        &self.board
    }

    fn try_move(&mut self, mut newboard: Board<N>) -> bool {
        if newboard.cell != self.board.cell {
            newboard.insert_random().unwrap();
            self.prev_boards.push(self.board);
            self.board = newboard;

            true
        } else {
            false
        }
    }

    pub fn left(&mut self) -> bool {
        self.try_move(self.board.left())
    }

    pub fn right(&mut self) -> bool {
        self.try_move(self.board.right())
    }

    pub fn up(&mut self) -> bool {
        self.try_move(self.board.up())
    }

    pub fn down(&mut self) -> bool {
        self.try_move(self.board.down())
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
