#[derive(Debug)]
pub struct King {
    has_moved: bool,
}

impl King {
    pub fn new() -> Self {
        King { has_moved: false }
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_move_true(&mut self) {
        self.has_moved = true;
    }
}
