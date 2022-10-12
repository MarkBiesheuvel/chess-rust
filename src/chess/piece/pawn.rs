#[derive(Debug)]
pub struct Pawn {
    has_moved: bool,
}

impl Pawn {
    pub fn new() -> Self {
        Pawn { has_moved: false }
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_move_true(&mut self) {
        self.has_moved = true;
    }
}
