use std::fmt;

#[derive(Copy, Clone)]
pub enum PieceKind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Copy, Clone)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
    has_moved: bool,
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.color, self.kind) {
            (PieceColor::White, PieceKind::Bishop) => {
                write!(f, "♗")?;
            }
            (PieceColor::Black, PieceKind::Bishop) => {
                write!(f, "♝")?;
            }
            (PieceColor::White, PieceKind::King) => {
                write!(f, "♔")?;
            }
            (PieceColor::Black, PieceKind::King) => {
                write!(f, "♚")?;
            }
            (PieceColor::White, PieceKind::Knight) => {
                write!(f, "♘")?;
            }
            (PieceColor::Black, PieceKind::Knight) => {
                write!(f, "♞")?;
            }
            (PieceColor::White, PieceKind::Pawn) => {
                write!(f, "♙")?;
            }
            (PieceColor::Black, PieceKind::Pawn) => {
                write!(f, "♟")?;
            }
            (PieceColor::White, PieceKind::Queen) => {
                write!(f, "♕")?;
            }
            (PieceColor::Black, PieceKind::Queen) => {
                write!(f, "♛")?;
            }
            (PieceColor::White, PieceKind::Rook) => {
                write!(f, "♖")?;
            }
            (PieceColor::Black, PieceKind::Rook) => {
                write!(f, "♜")?;
            }
        }

        Ok(())
    }
}
impl Piece {
    pub fn new(color: PieceColor, kind: PieceKind) -> Self {
        Piece {
            kind,
            color,
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
