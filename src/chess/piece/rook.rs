#[derive(Debug)]
pub struct Rook {
    has_moved: bool,
}

impl Rook {
    pub fn new() -> Self {
        Rook { has_moved: false }
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_move_true(&mut self) {
        self.has_moved = true;
    }
}
