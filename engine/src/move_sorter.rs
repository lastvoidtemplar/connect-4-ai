use crate::board::WIDTH;

pub struct MoveSorter {
    entries: [Entry; WIDTH],
    size: usize,
}

#[derive(Clone, Copy, Default)]
struct Entry {
    mov: u64,
    score: i32,
}

impl MoveSorter {
    pub fn new() -> Self {
        Self {
            entries: [Entry::default(); WIDTH],
            size: 0,
        }
    }

    pub fn add(&mut self, mov: u64, score: i32) {
        let mut pos = self.size;
        while pos != 0 && self.entries[pos - 1].score > score {
            self.entries[pos] = self.entries[pos-1];
            pos -= 1;
        }
        self.entries[pos].mov = mov;
        self.entries[pos].score = score;
        self.size += 1;
    }
}

impl Iterator for MoveSorter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.entries[self.size].mov)
        } else {
            None
        }
    }
}