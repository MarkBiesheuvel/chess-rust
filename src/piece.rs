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
        write!(f, "{}", self.symbol())
    }
}
impl Piece {
    pub fn new(color: PieceColor, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }

    fn symbol(&self) -> &str {
        match (self.color, self.kind) {
            (PieceColor::White, PieceKind::Bishop) => "♗",
            (PieceColor::Black, PieceKind::Bishop) => "♝",
            (PieceColor::White, PieceKind::King) => "♔",
            (PieceColor::Black, PieceKind::King) => "♚",
            (PieceColor::White, PieceKind::Knight) => "♘",
            (PieceColor::Black, PieceKind::Knight) => "♞",
            (PieceColor::White, PieceKind::Pawn) => "♙",
            (PieceColor::Black, PieceKind::Pawn) => "♟",
            (PieceColor::White, PieceKind::Queen) => "♕",
            (PieceColor::Black, PieceKind::Queen) => "♛",
            (PieceColor::White, PieceKind::Rook) => "♖",
            (PieceColor::Black, PieceKind::Rook) => "♜",
        }
    }

    pub fn legal_moves(&self) -> Vec<(i8, i8)> {
        match self.kind {
            PieceKind::Knight => {
                vec![(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, 2)]
            }
            _ => {
                // TODO: implement for other pieces
                vec![]
            }
        }
    }
}
