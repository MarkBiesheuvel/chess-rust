#[derive(Debug)]
pub enum Piece {
    Pawn { has_moved: bool },
    Knight,
    Bishop,
    Rook { has_moved: bool },
    Queen,
    King { has_moved: bool },
}

impl Piece {
    pub fn make_move(&mut self) {
        // Update the has_moved property on pieces with special moves, ignore for anything else
        match self {
            Piece::King { has_moved } | Piece::Pawn { has_moved } | Piece::Rook { has_moved } => {
                *has_moved = true;
            }
            _ => {}
        }
    }

    // TODO: create tuple struct for move
    pub fn legal_moves(&self) -> Vec<(i8, i8)> {
        match self {
            Piece::Knight => {
                vec![
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, 2),
                ]
            }
            _ => {
                // TODO: implement for other pieces
                vec![]
            }
        }
    }
}
