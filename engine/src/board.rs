use std::{error::Error, fmt::Display, str::FromStr};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

#[derive(Clone)]
pub struct Board {
    board: [[i32; HEIGHT]; WIDTH],
    columns_heights: [usize; WIDTH],
    played_moves: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[0; HEIGHT]; WIDTH],
            columns_heights: [0; WIDTH],
            played_moves: 0,
        }
    }

    fn current_player(&self) -> i32 {
        (1 + (self.played_moves & 1)) as i32
    }

    pub fn can_play(&self, colm: usize) -> bool {
        self.columns_heights[colm] < HEIGHT
    }

    pub fn play(&mut self, colm: usize) {
        self.board[colm][self.columns_heights[colm]] += self.current_player();
        self.columns_heights[colm] += 1;
        self.played_moves += 1;
    }

    pub fn is_winning(&self, colm: usize) -> bool {
        let current_player = self.current_player();

        let column_height = self.columns_heights[colm];
        let board_column = self.board[colm];
        if column_height >= 3
            && board_column[column_height - 1] == current_player
            && board_column[column_height - 2] == current_player
            && board_column[column_height - 3] == current_player
        {
            return true;
        }

        // -1 - / diagonal, 0 - _ horizotal, 1 - \
        for direction_y in -1..=1 {
            let mut encounter_cells_current_player = 0;
            for direction_x in (-1..=1).step_by(2) {
                let mut x = colm as i32 + direction_x;
                let mut y = column_height as i32 + direction_y * direction_x;
                while x >= 0
                    && x < WIDTH as i32
                    && y >= 0
                    && y < HEIGHT as i32
                    && self.board[x as usize][y as usize] == current_player
                {
                    encounter_cells_current_player += 1;
                    x += direction_x;
                    y += direction_y * direction_x
                }
            }
            if encounter_cells_current_player >= 3 {
                return true;
            }
        }
        return false;
    }

    pub fn played_moves(&self) -> usize {
        self.played_moves
    }
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
