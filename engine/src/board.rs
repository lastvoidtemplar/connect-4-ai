use std::{error::Error, fmt::Display, str::FromStr};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;


/*
    the board is encoded in this direction
    6 13 ...
    5 12 ...
    4 11 ...
    3 10 ...
    2  9 ...
    1  8 ...
    0  7 ...
col 0  1  2
*/

/*
    mask is a bitboard with 1 where there is a non-empty cell
    current is a bitboard where the cells of the current player are marked as 1
*/
#[derive(Clone, Copy)]
pub struct Board {
    current: u64,
    mask: u64,
    played_moves: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            current: 0,
            mask: 0,
            played_moves: 0,
        }
    }

    pub fn can_play(&self, colm: usize) -> bool {
        (self.mask & top_mask(colm)) == 0
    }

    pub fn play(&mut self, colm: usize) {
        self.current = self.mask ^ self.current;
        self.mask = self.mask | (self.mask + bottom_mask(colm));
        self.played_moves += 1;
    }

    pub fn is_winning(&self, colm: usize) -> bool {
        // filling top of the colm
        let position = self.current | ((self.mask + bottom_mask(colm)) & column_mask(colm));

        let vertical_shift = 1;
        let vertical_pair = position & (position >> vertical_shift);
        let vertical_line = vertical_pair & (vertical_pair >> (2 * vertical_shift));

        let horizontal_shift = HEIGHT + 1;
        let horizontal_pair = position & (position >> horizontal_shift);
        let horizontal_line = horizontal_pair & (horizontal_pair >> (2 * horizontal_shift));

        let main_diagonal_shift = HEIGHT;
        let main_diagonal_pair = position & (position >> main_diagonal_shift);
        let main_diagonal_line =
            main_diagonal_pair & (main_diagonal_pair >> (2 * main_diagonal_shift));

        let off_diagonal_shift = HEIGHT + 2;
        let off_diagonal_pair = position & (position >> off_diagonal_shift);
        let off_diagonal_line = off_diagonal_pair & (off_diagonal_pair >> (2 * off_diagonal_shift));

        return vertical_line != 0
            || horizontal_line != 0
            || main_diagonal_line != 0
            || off_diagonal_line != 0;
    }

    pub fn played_moves(&self) -> usize {
        self.played_moves
    }

    pub fn key(&self) -> u64 {
        self.current + self.mask
    }
}

fn top_mask(colm: usize) -> u64 {
    (1u64 << (HEIGHT - 1)) << colm * (HEIGHT + 1)
}

fn bottom_mask(colm: usize) -> u64 {
    1u64 << colm * (HEIGHT + 1)
}

fn column_mask(colm: usize) -> u64 {
    ((1u64 << HEIGHT) - 1) << colm * (HEIGHT + 1)
}

impl FromStr for Board {
    type Err = ParsingBoardErr;

    fn from_str(encoded_board: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        for ch in encoded_board.as_bytes() {
            let colm = (ch - '1' as u8) as usize;
            if colm >= WIDTH || !board.can_play(colm) || board.is_winning(colm) {
                return Err(ParsingBoardErr { msg: encoded_board.to_string() });
            }
            board.play(colm);
        }
        Ok(board)
    }
}

#[derive(Debug)]
pub struct ParsingBoardErr {
    msg: String,
}

impl Display for ParsingBoardErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Invalid board: {}", self.msg)
    }
}

impl Error for ParsingBoardErr {}
