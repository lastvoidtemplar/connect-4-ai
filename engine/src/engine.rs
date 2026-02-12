use crate::{
    board::{Board, HEIGHT, WIDTH},
    transposition_table::TranspositionTable,
};
use std::cmp::max;

const MIN_SCORE: i32 = -(WIDTH as i32 * HEIGHT as i32) / 2 + 3;
const MAX_SCORE: i32 = (WIDTH as i32 * HEIGHT as i32 + 1) / 2 - 3;

// nearest prime to 8 * 1024 * 1024
const TRANSPOSITION_TABLE_SIZE: usize = 8388593;

pub struct Engine {
    column_order: [usize; WIDTH],
    table: TranspositionTable,
    explored_nodes: usize,
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
            table: TranspositionTable::new(TRANSPOSITION_TABLE_SIZE),
            explored_nodes: 0,
        }
    }

    fn negamax(&mut self, board: Board, mut alpha: i32, mut beta: i32) -> i32 {
        self.explored_nodes += 1;
        if board.played_moves() == WIDTH * HEIGHT {
            return 0;
        }

        for colm in 0..WIDTH {
            if board.can_play(colm) && board.is_winning(colm) {
                return (WIDTH * HEIGHT - board.played_moves() + 1) as i32 / 2;
            }
        }

        let mut upper_bound = (WIDTH * HEIGHT - board.played_moves() - 1) as i32 / 2;
        if let Some(value) = self.table.get(board.key()) {
            upper_bound = value as i32 + MIN_SCORE as i32 - 1;
        }
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
        self.table.put(board.key(), (alpha - MIN_SCORE + 1) as u8);
        return alpha;
    }

    pub fn score(&mut self, board: Board) -> i32 {
        let mut left = -((WIDTH * HEIGHT - board.played_moves()) as i32) / 2;
        let mut right = (WIDTH * HEIGHT - board.played_moves() + 1) as i32 / 2;

        while left < right {
            let mut median = left + (right - left) / 2;
            if median <= 0 && left / 2 < median {
                median = left / 2;
            } else if median >= 0 && median < right / 2 {
                median = right / 2;
            }
            let score = self.negamax(board, median, median + 1);
            if score <= median {
                right = score;
            } else {
                left = score;
            }
        }

        left
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

    pub fn explored_nodes(&self) -> usize {
        self.explored_nodes
    }

    pub fn reset(&mut self) {
        self.explored_nodes = 0;
        self.table = TranspositionTable::new(TRANSPOSITION_TABLE_SIZE);
    }
}
