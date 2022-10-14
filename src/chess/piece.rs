#![allow(dead_code)]
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum PieceKind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    color: PieceColor,
    kind: PieceKind,
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
    pub fn fen(character: char) -> Piece {
        match character {
            'B' => Piece {
                color: PieceColor::White,
                kind: PieceKind::Bishop,
            },
            'b' => Piece {
                color: PieceColor::Black,
                kind: PieceKind::Bishop,
            },
            'K' => Piece {
                color: PieceColor::White,
                kind: PieceKind::King,
            },
            'k' => Piece {
                color: PieceColor::Black,
                kind: PieceKind::King,
            },
            'N' => Piece {
                color: PieceColor::White,
                kind: PieceKind::Knight,
            },
            'n' => Piece {
                color: PieceColor::Black,
                kind: PieceKind::Knight,
            },
            'P' => Piece {
                color: PieceColor::White,
                kind: PieceKind::Pawn,
            },
            'p' => Piece {
                color: PieceColor::Black,
                kind: PieceKind::Pawn,
            },
            'Q' => Piece {
                color: PieceColor::White,
                kind: PieceKind::Queen,
            },
            'q' => Piece {
                color: PieceColor::Black,
                kind: PieceKind::Queen,
            },
            'R' => Piece {
                color: PieceColor::White,
                kind: PieceKind::Rook,
            },
            'r' => Piece {
                color: PieceColor::Black,
                kind: PieceKind::Rook,
            },
            _ => {
                panic!("Invalid piece");
            }
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
