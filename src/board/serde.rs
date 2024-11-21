use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

use super::{Board, Cell};

#[derive(Deserialize, Serialize)]
struct FlatBoard {
    moves: usize,
    score: usize,
    dimension: usize,
    cells: Vec<i8>,
}

impl FlatBoard {
    pub fn new<const N: usize>(board: &Board<N>) -> FlatBoard {
        let mut cells = Vec::with_capacity(N * N);

        for i in 0..N {
            for j in 0..N {
                cells.push(match board.cell[i][j].order() {
                    None => -1,
                    Some(order) => i8::try_from(order).unwrap(),
                });
            }
        }

        FlatBoard {
            moves: board.moves,
            score: board.score,
            dimension: N,
            cells,
        }
    }
}

impl<const N: usize> Board<N> {
    pub fn serialize(&self) -> String {
        let flat_board = FlatBoard::new(self);

        serde_json::to_string(&flat_board).unwrap()
    }

    pub fn deserialize(board: &str) -> Result<Board<N>, Error> {
        let flat_board: FlatBoard = serde_json::from_str(board)?;

        if flat_board.dimension != N {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "runtime dimension {} but save dimension {}",
                    N, flat_board.dimension
                ),
            ));
        }

        let mut board = Board::<N>::new();

        let mut cells = flat_board.cells.iter();

        for i in 0..N {
            for j in 0..N {
                board.cell[i][j] = match cells.next().unwrap() {
                    -1 => Cell::new_empty(),
                    order => Cell::new(*order),
                };
            }
        }

        board.moves = flat_board.moves;
        board.score = flat_board.score;

        Ok(board)
    }
}
