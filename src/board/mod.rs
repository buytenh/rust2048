use rand::{seq::SliceRandom, Rng};

use crate::cell::Cell;

mod serde;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board<const N: usize> {
    // TODO: remove 'pub'
    pub cell: [[Cell; N]; N],
    pub moves: usize,
    pub score: usize,
}

impl<const N: usize> Board<N> {
    pub fn new() -> Board<N> {
        Board {
            cell: [[Cell::new_empty(); N]; N],
            moves: 0,
            score: 0,
        }
    }

    pub fn game_over(&self) -> bool {
        self.cell == self.left().cell
            && self.cell == self.right().cell
            && self.cell == self.up().cell
            && self.cell == self.down().cell
    }

    pub fn pick_random_insertion(&self) -> Result<(usize, usize, Cell), ()> {
        let mut empty_cells = Vec::with_capacity(N * N);

        for i in 0..N {
            for j in 0..N {
                if self.cell[i][j].is_empty() {
                    empty_cells.push((i, j));
                }
            }
        }

        let mut thread_rng = rand::thread_rng();

        empty_cells
            .choose(&mut thread_rng)
            .map(|(i, j)| {
                (
                    *i,
                    *j,
                    if thread_rng.gen_ratio(1, 10) {
                        Cell::new(2)
                    } else {
                        Cell::new(1)
                    },
                )
            })
            .ok_or(())
    }

    pub fn insert_random(&mut self) -> Result<(), ()> {
        let (i, j, cell) = self.pick_random_insertion()?;

        self.cell[i][j] = cell;

        Ok(())
    }

    pub fn left(&self) -> Board<N> {
        let mut newboard = Self::new();

        newboard.moves = self.moves + 1;

        newboard.score = self.score;

        for i in 0..N {
            let (row, score) =
                Cell::collapse_array::<N>(&core::array::from_fn(|j| self.cell[i][j]));

            for (j, val) in row.into_iter().enumerate() {
                newboard.cell[i][j] = val;
            }

            newboard.score += score;
        }

        newboard
    }

    pub fn right(&self) -> Board<N> {
        let mut newboard = Self::new();

        newboard.moves = self.moves + 1;

        newboard.score = self.score;

        for i in 0..N {
            let (row, score) =
                Cell::collapse_array::<N>(&core::array::from_fn(|j| self.cell[i][N - 1 - j]));

            for (j, val) in row.into_iter().enumerate() {
                newboard.cell[i][N - 1 - j] = val;
            }

            newboard.score += score;
        }

        newboard
    }

    pub fn up(&self) -> Board<N> {
        let mut newboard = Self::new();

        newboard.moves = self.moves + 1;

        newboard.score = self.score;

        for j in 0..N {
            let (row, score) =
                Cell::collapse_array::<N>(&core::array::from_fn(|i| self.cell[i][j]));

            for (i, val) in row.into_iter().enumerate() {
                newboard.cell[i][j] = val;
            }

            newboard.score += score;
        }

        newboard
    }

    pub fn down(&self) -> Board<N> {
        let mut newboard = Self::new();

        newboard.moves = self.moves + 1;

        newboard.score = self.score;

        for j in 0..N {
            let (row, score) =
                Cell::collapse_array::<N>(&core::array::from_fn(|i| self.cell[N - 1 - i][j]));

            for (i, val) in row.into_iter().enumerate() {
                newboard.cell[N - 1 - i][j] = val;
            }

            newboard.score += score;
        }

        newboard
    }
}
