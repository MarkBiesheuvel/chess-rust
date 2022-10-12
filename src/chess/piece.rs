use bishop::Bishop as B;
use king::King as K;
use knight::Knight as N;
use pawn::Pawn as P;
use queen::Queen as Q;
use rook::Rook as R;

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

#[derive(Debug)]
pub enum Piece {
    Bishop(B),
    King(K),
    Knight(N),
    Pawn(P),
    Queen(Q),
    Rook(R),
}

impl Piece {
    pub fn new_bishop() -> Self {
        Piece::Bishop(B::new())
    }

    pub fn new_king() -> Self {
        Piece::King(K::new())
    }

    pub fn new_knight() -> Self {
        Piece::Knight(N::new())
    }

    pub fn new_pawn() -> Self {
        Piece::Pawn(P::new())
    }

    pub fn new_queen() -> Self {
        Piece::Queen(Q::new())
    }

    pub fn new_rook() -> Self {
        Piece::Rook(R::new())
    }

    pub fn make_move(&mut self) {
        // Update the has_moved property on pieces with special moves, ignore for anything else
        match self {
            Piece::Pawn(pawn) => match pawn.has_moved() {
                false => {
                    pawn.set_move_true();
                }
                true => {}
            },
            Piece::Rook(rook) => match rook.has_moved() {
                false => {
                    rook.set_move_true();
                }
                true => {}
            },
            Piece::King(king) => match king.has_moved() {
                false => {
                    king.set_move_true();
                }
                true => {}
            },
            _ => {}
        }
    }

    // TODO: create tuple struct for move
    pub fn legal_moves(&self) -> Vec<(i8, i8)> {
        match self {
            Piece::Knight(_) => {
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
