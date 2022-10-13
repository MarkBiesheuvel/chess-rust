#[derive(Debug)]
pub enum PieceKind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(Debug)]
pub struct Piece {
    kind: PieceKind,
    has_moved: bool,
}

impl Piece {
    pub fn new(kind: PieceKind) -> Self {
        Piece {
            kind,
            has_moved: false,
        }
    }

    pub fn update(&mut self) {
        if !self.has_moved {
            self.has_moved = true;
        }
    }

    pub fn legal_moves(&self) -> Vec<(i8, i8)> {
        match self.kind {
            PieceKind::Knight => {
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
