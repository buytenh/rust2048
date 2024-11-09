use std::{
    fmt::{Debug, Display, Error, Formatter},
    num::NonZeroU8,
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Cell(Option<NonZeroU8>);

impl Cell {
    pub fn new_empty() -> Cell {
        Cell(None)
    }

    pub fn new<T>(order: T) -> Cell
    where
        T: TryInto<u8>,
        <T as TryInto<u8>>::Error: Debug,
    {
        let order = order.try_into().unwrap();

        if order == 0 {
            Cell(None)
        } else {
            Cell(Some(NonZeroU8::new(order).unwrap()))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_nonempty(&self) -> bool {
        self.0.is_some()
    }

    pub fn order(&self) -> Option<u8> {
        self.0.map(|order| order.get())
    }

    pub fn double(&self) -> Cell {
        Self::new(self.0.unwrap().get() + 1)
    }

    pub fn collapse(val: &[Cell]) -> (Vec<Cell>, usize) {
        let mut newval = Vec::new();
        let mut score = 0;

        let mut acc = Cell::new_empty();

        for el in val.iter().filter(|el| el.is_nonempty()) {
            if acc.is_empty() {
                acc = *el;
            } else if acc == *el {
                let acc_double = acc.double();

                score += 1 << acc_double.order().unwrap();

                newval.push(acc_double);
                acc = Cell::new_empty();
            } else {
                newval.push(acc);
                acc = *el;
            }
        }

        if acc.is_nonempty() {
            newval.push(acc);
        }

        (newval, score)
    }

    pub fn collapse_array<const N: usize>(val: &[Cell; N]) -> ([Cell; N], usize) {
        let (mut val, score) = Self::collapse(&val[..]);

        val.resize_with(N, Cell::new_empty);

        (val[0..N].try_into().unwrap(), score)
    }

    pub fn print(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self.0 {
            None => write!(fmt, "-"),
            Some(order) => write!(fmt, "{}", 1 << order.get()),
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        self.print(fmt)
    }
}

impl Display for Cell {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        self.print(fmt)
    }
}

#[test]
fn test() {
    struct Test {
        input: [u8; 4],
        output: [u8; 4],
        score: usize,
    }

    // Taken from https://github.com/mevdschee/2048.c/blob/c4f03846a47df967b967ca5f7d11fb4ef82196f4/2048.c#L353-L366
    const TESTS: [Test; 13] = [
        Test {
            input: [0, 0, 0, 1],
            output: [1, 0, 0, 0],
            score: 0,
        },
        Test {
            input: [0, 0, 1, 1],
            output: [2, 0, 0, 0],
            score: 4,
        },
        Test {
            input: [0, 1, 0, 1],
            output: [2, 0, 0, 0],
            score: 4,
        },
        Test {
            input: [1, 0, 0, 1],
            output: [2, 0, 0, 0],
            score: 4,
        },
        Test {
            input: [1, 0, 1, 0],
            output: [2, 0, 0, 0],
            score: 4,
        },
        Test {
            input: [1, 1, 1, 0],
            output: [2, 1, 0, 0],
            score: 4,
        },
        Test {
            input: [1, 0, 1, 1],
            output: [2, 1, 0, 0],
            score: 4,
        },
        Test {
            input: [1, 1, 0, 1],
            output: [2, 1, 0, 0],
            score: 4,
        },
        Test {
            input: [1, 1, 1, 1],
            output: [2, 2, 0, 0],
            score: 8,
        },
        Test {
            input: [2, 2, 1, 1],
            output: [3, 2, 0, 0],
            score: 12,
        },
        Test {
            input: [1, 1, 2, 2],
            output: [2, 3, 0, 0],
            score: 12,
        },
        Test {
            input: [3, 0, 1, 1],
            output: [3, 2, 0, 0],
            score: 4,
        },
        Test {
            input: [2, 0, 1, 1],
            output: [2, 2, 0, 0],
            score: 4,
        },
    ];

    for test in &TESTS {
        let test_input = test.input.map(|val| Cell::new(val));
        let test_output = test.output.map(|val| Cell::new(val));
        let test_score = test.score;

        let (output, score) = Cell::collapse_array(&test_input);

        assert_eq!(test_output, output);
        assert_eq!(test_score, score);
    }
}
