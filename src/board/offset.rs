// Offset on the chess board
#[derive(Debug)]
pub struct Offset {
    file: i8,
    rank: i8,
}

impl Offset {
    pub const fn new(file: i8, rank: i8) -> Offset {
        Offset { file, rank }
    }

    pub fn file(&self) -> i8 {
        self.file
    }

    pub fn rank(&self) -> i8 {
        self.rank
    }
}
