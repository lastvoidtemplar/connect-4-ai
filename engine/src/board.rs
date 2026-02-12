use std::{error::Error, fmt::Display, str::FromStr};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;


// compile time function to help generate bottom_mask
const fn bottom(width: usize, height: usize) -> u64 {
    if width == 0 {
        0
    } else {
        bottom(width - 1, height) | (1u64 << ((width - 1) * (height + 1)))
    }
}

// one on the bottom of each column
const BOTTOM_MASK: u64 = bottom(WIDTH, HEIGHT);
// one everywhere except the the overflow row
const BOARD_MASK: u64 = BOTTOM_MASK * ((1 << HEIGHT) - 1);

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
        (self.mask & top_mask_colm(colm)) == 0
    }

    pub fn play(&mut self, colm: usize) {
        self.current = self.mask ^ self.current;
        self.mask = self.mask | (self.mask + bottom_mask_colm(colm));
        self.played_moves += 1;
    }

    pub fn is_winning(&self, colm: usize) -> bool {
        (self.current_winning_moves() & self.possible() & column_mask(colm)) != 0
    }

    pub fn played_moves(&self) -> usize {
        self.played_moves
    }

    pub fn key(&self) -> u64 {
        self.current + self.mask
    }

    fn possible(&self) -> u64 {
        (self.mask + BOTTOM_MASK) & BOARD_MASK
    }

    fn current_winning_moves(&self) -> u64 {
        compute_winning_position(self.current, self.mask)
    }

    fn opponent_winning_moves(&self) -> u64 {
        compute_winning_position(self.current ^ self.mask, self.mask)
    }

    pub fn can_win_next(&self) -> bool {
        self.current_winning_moves() & self.possible() != 0
    }

    pub fn possible_non_losing_moves(&self) -> u64{
        let mut possible = self.possible();
        let opponent_winning = self.opponent_winning_moves();
        // block or the opponent will win with next move
        let forced = possible & opponent_winning;
        if forced != 0 {
            // bit trick to see if there is more than forced move
            if forced & (forced - 1) != 0 {
                // we lose
                return 0; 
            } else {
                possible = forced;
            }
        }
        // dont play under a winning move
        possible & (!(opponent_winning >> 1))
    }

    pub fn play_move(&mut self, mov: u64) {
        self.current = self.mask ^ self.current;
        self.mask = self.mask | mov;
        self.played_moves+=1;
    }

    pub fn score(&self, mov: u64) -> i32 {
        popcount(compute_winning_position(self.current | mov, self.mask))
    }
}

fn top_mask_colm(colm: usize) -> u64 {
    (1u64 << (HEIGHT - 1)) << colm * (HEIGHT + 1)
}

fn bottom_mask_colm(colm: usize) -> u64 {
    1u64 << colm * (HEIGHT + 1)
}

pub fn column_mask(colm: usize) -> u64 {
    ((1u64 << HEIGHT) - 1) << colm * (HEIGHT + 1)
}

fn compute_winning_position(position: u64, mask: u64) -> u64 {
    // vertical - 3 under
    let mut winning = (position << 1) & (position << 2) & (position << 3);

    let horizontal_shift = HEIGHT + 1;
    // xx_
    let mut horizontal_pair = (position << horizontal_shift) & (position << (2 * horizontal_shift));
    // xxx_
    winning |= horizontal_pair & (position << (3 * horizontal_shift));
    // xx_x
    winning |= horizontal_pair & (position >> horizontal_shift);
    // _xx
    horizontal_pair = horizontal_pair >> (3 * horizontal_shift);
    // _xxx
    winning |= horizontal_pair & (position << horizontal_shift);
    // x_xx
    winning |= horizontal_pair & (position >> (3 * horizontal_shift));

    let main_dialonal_shift = HEIGHT;
    let mut main_dialonal_pair =
        (position << main_dialonal_shift) & (position << (2 * main_dialonal_shift));
    winning |= main_dialonal_pair & (position << (3 * main_dialonal_shift));
    winning |= main_dialonal_pair & (position >> main_dialonal_shift);
    main_dialonal_pair = main_dialonal_pair >> (3 * main_dialonal_shift);
    winning |= main_dialonal_pair & (position << main_dialonal_shift);
    winning |= main_dialonal_pair & (position >> (3 * main_dialonal_shift));

    let off_dialonal_shift = HEIGHT + 2;
    let mut off_dialonal_pair =
        (position << off_dialonal_shift) & (position << (2 * off_dialonal_shift));
    winning |= off_dialonal_pair & (position << (3 * off_dialonal_shift));
    winning |= off_dialonal_pair & (position >> off_dialonal_shift);
    off_dialonal_pair = off_dialonal_pair >> (3 * off_dialonal_shift);
    winning |= off_dialonal_pair & (position << off_dialonal_shift);
    winning |= off_dialonal_pair & (position >> (3 * off_dialonal_shift));

    return winning & (BOARD_MASK ^ mask);
}

fn popcount(mut mask: u64) -> i32 {
    let mut count = 0;
    while mask != 0{
        mask = mask & (mask - 1);
        count+=1;
    }
    count
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
