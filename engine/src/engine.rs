use std::cmp::max;

use crate::board::{Board, HEIGHT, WIDTH};

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    fn negamax(&mut self, board: Board) -> i32 {
        if board.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH {
            if board.can_play(colm) && board.is_winning(colm) {
                return (WIDTH * HEIGHT - board.played_moves() + 1) as i32 / 2;
            }
        }

        let mut best_score = i32::MIN;
        for colm in 0..WIDTH {
            if board.can_play(colm) {
                let mut board = board.clone();
                board.play(colm);
                best_score = max(best_score, -self.negamax(board));
            }
        }

        return best_score;
    }

    pub fn score(&mut self, board: Board) -> i32 {
        self.negamax(board)
    }

    pub fn solve(&mut self, board: Board) -> [Option<i32>; WIDTH] {
        let mut result = [None; WIDTH];
        for (colm, colm_result) in result.iter_mut().enumerate() {
            if board.can_play(colm) {
                let mut board = board.clone();
                board.play(colm);
                *colm_result = Some(-self.negamax(board));
            }
        }
        result
    }
}
