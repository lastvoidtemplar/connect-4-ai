#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<Entry>
}

#[derive(Clone, Copy)]
struct Entry(u64);

impl Entry {
    #[inline]
    fn key(self) -> u64 {
        self.0 >> 8
    }

    #[inline]
    fn value(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    #[inline]
    fn new(key: u64, value: u8) -> Self {
        Entry((key << 8) | value as u64)
    }
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self { table: vec![Entry(0); size] }
    }

    fn index(&self, key: u64) -> usize {
        key as usize % self.table.len()
    }

    pub fn get(&self, key: u64) -> Option<u8> {
        let ind = self.index(key);
        let entry = self.table[ind];
        if entry.key() == key {
            Some(entry.value())
        } else {
            None
        }
    }

    pub fn put(&mut self, key: u64, value: u8) {
        let ind = self.index(key);
        let entry = Entry::new(key, value);
        self.table[ind] = entry;
    }
}