use std::{io, io::Write};

use termion::{clear, color, color::AnsiValue, cursor, raw::RawTerminal, style};

use crate::{board::Board, cell::Cell};

fn colors(cell: &Cell) -> (AnsiValue, AnsiValue) {
    match cell.order() {
        None => (AnsiValue::grayscale(1), AnsiValue::grayscale(23)),
        Some(1) => (AnsiValue::grayscale(2), AnsiValue::grayscale(23)),
        Some(2) => (AnsiValue::grayscale(4), AnsiValue::grayscale(23)),
        Some(3) => (AnsiValue::grayscale(6), AnsiValue::grayscale(23)),
        Some(4) => (AnsiValue::grayscale(8), AnsiValue::grayscale(23)),
        Some(5) => (AnsiValue::grayscale(10), AnsiValue::grayscale(23)),
        Some(6) => (AnsiValue::grayscale(12), AnsiValue::grayscale(23)),
        Some(7) => (AnsiValue::grayscale(14), AnsiValue(0)),
        Some(8) => (AnsiValue::grayscale(16), AnsiValue(0)),
        Some(9) => (AnsiValue::grayscale(17), AnsiValue(0)),
        Some(10) => (AnsiValue::grayscale(18), AnsiValue(0)),
        Some(11) => (AnsiValue::grayscale(19), AnsiValue(0)),
        Some(12) => (AnsiValue::grayscale(20), AnsiValue(0)),
        Some(13) => (AnsiValue::grayscale(21), AnsiValue(0)),
        Some(14) => (AnsiValue::grayscale(22), AnsiValue(0)),
        Some(_) => (AnsiValue::grayscale(23), AnsiValue(0)),
    }
}

pub fn print_board<const N: usize>(stdout: &mut RawTerminal<io::Stdout>, board: &Board<N>) {
    write!(
        stdout,
        "{}{}{}score: {:5}    moves: {:5}",
        clear::All,
        style::Reset,
        cursor::Goto(1, 1),
        board.score,
        board.moves,
    )
    .unwrap();

    for i in 0..N {
        write!(
            stdout,
            "{}",
            cursor::Goto(1, (3 + 3 * i).try_into().unwrap())
        )
        .unwrap();

        for j in 0..N {
            let (bg, fg) = colors(&board.cell[i][j]);
            write!(stdout, "{}{}       ", color::Bg(bg), color::Fg(fg)).unwrap();
        }

        write!(
            stdout,
            "{}",
            cursor::Goto(1, (3 + 3 * i + 1).try_into().unwrap())
        )
        .unwrap();

        for j in 0..N {
            let cell = board.cell[i][j];

            let (bg, fg) = colors(&cell);
            write!(stdout, "{}{}", color::Bg(bg), color::Fg(fg)).unwrap();

            match cell.order() {
                None => write!(stdout, "   .   ").unwrap(),
                Some(order) => write!(stdout, "{:^7}", 1 << order).unwrap(),
            }
        }

        write!(
            stdout,
            "{}",
            cursor::Goto(1, (3 + 3 * i + 2).try_into().unwrap())
        )
        .unwrap();

        for j in 0..N {
            let (bg, fg) = colors(&board.cell[i][j]);
            write!(stdout, "{}{}       ", color::Bg(bg), color::Fg(fg)).unwrap();
        }
    }

    write!(stdout, "{}", style::Reset).unwrap();

    let mid: u16 = ((7 * N) / 2).try_into().unwrap();

    write!(
        stdout,
        "{}[←]  [↓]  [↑]  [→]",
        cursor::Goto(mid - 8, (3 + 3 * N + 1).try_into().unwrap())
    )
    .unwrap();

    write!(
        stdout,
        "{}[u] undo    [s] save",
        cursor::Goto(mid - 9, (3 + 3 * N + 3).try_into().unwrap())
    )
    .unwrap();

    write!(
        stdout,
        "{}[q] quit    [l] load",
        cursor::Goto(mid - 9, (3 + 3 * N + 4).try_into().unwrap())
    )
    .unwrap();

    write!(
        stdout,
        "{}[r] restart",
        cursor::Goto(mid - 4, (3 + 3 * N + 5).try_into().unwrap())
    )
    .unwrap();

    if board.game_over() {
        write!(
            stdout,
            "{}GAME  OVER",
            cursor::Goto(mid - 4, (3 + 3 * N + 7).try_into().unwrap()),
        )
        .unwrap();
    }

    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

    stdout.lock().flush().unwrap();
}
