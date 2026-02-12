use crate::board::{Board, HEIGHT, WIDTH};
use std::cmp::max;

const MIN_SCORE: i32 = -(WIDTH as i32 * HEIGHT as i32) / 2 + 3;
const MAX_SCORE: i32 = (WIDTH as i32 * HEIGHT as i32 + 1) / 2 - 3;

pub struct Engine{
    column_order: [usize; WIDTH]
}

impl Engine {
    pub fn new() -> Self {
        let mut column_order = [0; WIDTH];

        // [3, 2, 4, 1, 5, 0, 6]
        for ind in 0..(WIDTH as i32) {
            let colm = WIDTH as i32 / 2 + (1 - 2 * (ind & 1)) * (ind + 1) / 2;
            column_order[ind as usize] = colm as usize;
        }

        Self {
            column_order,
        }
    }

    fn negamax(&mut self, board: Board, mut alpha: i32, mut beta: i32) -> i32 {
        if board.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH {
            if board.can_play(colm) && board.is_winning(colm) {
                return (WIDTH * HEIGHT - board.played_moves() + 1) as i32 / 2;
            }
        }

        let upper_bound = (WIDTH * HEIGHT - board.played_moves() - 1) as i32 / 2;
        if upper_bound < beta {
            beta = upper_bound;
            if alpha >= beta {
                return beta;
            }
        }

        for ind in 0..WIDTH {
            let colm = self.column_order[ind];
            if board.can_play(colm) {
                let mut board = board.clone();
                board.play(colm);
                alpha = max(alpha, -self.negamax(board, -beta, -alpha));
                if alpha >= beta {
                    return alpha;
                }
            }
        }
        return alpha;
    }

    pub fn score(&mut self, board: Board) -> i32 {
        self.negamax(board, MIN_SCORE, MAX_SCORE)
    }

    pub fn solve(&mut self, board: Board) -> [Option<i32>; WIDTH] {
        let mut result = [None; WIDTH];
        for (colm, colm_result) in result.iter_mut().enumerate() {
            if board.can_play(colm) {
                let mut board = board.clone();
                board.play(colm);
                *colm_result = Some(-self.negamax(board, MIN_SCORE, MAX_SCORE));
            }
        }
        result
    }
}
