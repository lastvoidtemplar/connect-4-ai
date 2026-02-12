use std::{collections::HashMap, thread};

use engine::{
    board::{Board, ParsingBoardErr},
    engine::Engine,
};

const SCORE_SHIFT: u8 = 127;
const BOOK_PATH: &'static str = "opening-book-6";

fn generate_subtree(
    engine: &mut Engine,
    encoded_board: &mut String,
    depth: usize,
    map: &mut HashMap<u64, i32>,
) {
    let board: Result<Board, ParsingBoardErr> = encoded_board.parse();
    if let Ok(board) = board {
        let key = board.key();

        if map.contains_key(&key) {
            return;
        }

        let score = engine.score(board);
        map.insert(key, score);

        if depth == 0 {
            return;
        }

        for col in 1..=7 {
            encoded_board.push(char::from_digit(col as u32, 10).unwrap());
            generate_subtree(engine, encoded_board, depth - 1, map);
            encoded_board.pop();
        }
    }
}

fn generate_book_parallel(depth: usize) -> HashMap<u64, i32> {
    let mut handlers = Vec::new();

    for col in 1..=7 {
        let handle = thread::spawn(move || {
            let mut engine = Engine::new();
            let mut map = HashMap::new();
            let mut encoded_board = col.to_string();

            generate_subtree(&mut engine, &mut encoded_board, depth - 1, &mut map);

            map
        });

        handlers.push(handle);
    }

    let mut book = HashMap::new();

    for handler in handlers {
        let map = handler.join().unwrap();
        for (k, v) in map {
            book.entry(k).or_insert(v);
        }
    }

    book
}

pub fn save_book(book: &HashMap<u64, i32>, book_path: &str) {
    use std::fs::File;
    use std::io::Write;

    let file = File::create(book_path).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    for (&key, &score) in book {
        let encoded_value = (score + SCORE_SHIFT as i32) as u8;

        let key_bytes = key.to_le_bytes();
        writer.write_all(&key_bytes[0..7]).unwrap();
        writer.write_all(&[encoded_value]).unwrap();
    }

    writer.flush().unwrap();
}

fn main() {
    let book = generate_book_parallel(6);
    save_book(&book, BOOK_PATH);
}
