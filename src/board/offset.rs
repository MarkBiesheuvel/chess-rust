// Imports from parent
use super::{File, Rank};

// Offset on the chess board
#[derive(Debug)]
pub struct Offset {
    file: File,
    rank: Rank,
}

impl Offset {
    pub const fn new(file: File, rank: Rank) -> Offset {
        Offset { file, rank }
    }

    pub fn file(&self) -> File {
        self.file
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}
