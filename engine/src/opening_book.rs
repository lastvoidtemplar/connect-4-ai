use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error, Read},
};

use crate::board::Board;

const SCORE_SHIFT: u8 = 127;

#[derive(Clone)]
pub struct OpeningBook {
    book: HashMap<u64, i32>,
}

impl OpeningBook {
    pub fn new() -> Self {
        Self {
            book: HashMap::new(),
        }
    }

    pub fn open(book_path: &str) -> Result<Self, Error> {
        let file = File::open(book_path)?;
        let mut reader = BufReader::new(file);

        let mut book = HashMap::new();
        let mut buf = [0u8; 8];

        while let Ok(_) = reader.read_exact(&mut buf) {
            let mut key_bytes = [0u8; 8];
            key_bytes[0..7].copy_from_slice(&buf[0..7]);
            let key = u64::from_le_bytes(key_bytes);

            let score = buf[7] as i32 - SCORE_SHIFT as i32;
            book.insert(key, score);
        }

        Ok(Self { book })
    }

    pub fn score(&self, board: &Board) -> Option<i32> {
        self.book.get(&board.key()).cloned()
    }
}
