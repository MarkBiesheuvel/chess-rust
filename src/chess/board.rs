use super::piece::{Piece, PieceColor, PieceKind};
use std::fmt;

#[derive(Copy, Clone)]
enum Square {
    Taken(Piece),
    Empty,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Taken(piece) => {
                write!(f, "{}", piece)?;
            }
            Square::Empty => {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

//
pub struct Board {
    squares: [[Square; 8]; 8],
}
impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[Square::Empty; 8]; 8],
        };

        // White pieces
        board.add_new_piece(0, 0, PieceColor::White, PieceKind::Rook);
        board.add_new_piece(1, 0, PieceColor::White, PieceKind::Knight);
        board.add_new_piece(2, 0, PieceColor::White, PieceKind::Bishop);
        board.add_new_piece(3, 0, PieceColor::White, PieceKind::Queen);
        board.add_new_piece(4, 0, PieceColor::White, PieceKind::King);
        board.add_new_piece(5, 0, PieceColor::White, PieceKind::Bishop);
        board.add_new_piece(6, 0, PieceColor::White, PieceKind::Knight);
        board.add_new_piece(7, 0, PieceColor::White, PieceKind::Rook);

        // White pawns
        board.add_new_piece(0, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(1, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(2, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(3, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(4, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(5, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(6, 1, PieceColor::White, PieceKind::Pawn);
        board.add_new_piece(7, 1, PieceColor::White, PieceKind::Pawn);

        // Black pawns
        board.add_new_piece(0, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(1, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(2, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(3, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(4, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(5, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(6, 6, PieceColor::Black, PieceKind::Pawn);
        board.add_new_piece(7, 6, PieceColor::Black, PieceKind::Pawn);

        // Black pieces
        board.add_new_piece(0, 7, PieceColor::Black, PieceKind::Rook);
        board.add_new_piece(1, 7, PieceColor::Black, PieceKind::Knight);
        board.add_new_piece(2, 7, PieceColor::Black, PieceKind::Bishop);
        board.add_new_piece(3, 7, PieceColor::Black, PieceKind::Queen);
        board.add_new_piece(4, 7, PieceColor::Black, PieceKind::King);
        board.add_new_piece(5, 7, PieceColor::Black, PieceKind::Bishop);
        board.add_new_piece(6, 7, PieceColor::Black, PieceKind::Knight);
        board.add_new_piece(7, 7, PieceColor::Black, PieceKind::Rook);

        board
    }

    fn add_new_piece(&mut self, file: usize, rank: usize, color: PieceColor, kind: PieceKind) {
        self.squares[rank][file] = Square::Taken(Piece::new(color, kind));
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓\n")?;

        for rank in (0..8).rev() {
            let row = self.squares[rank];

            write!(
                f,
                "┃ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} ┃\n",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7],
            )?;

            if rank > 0 {
                write!(f, "┠───┼───┼───┼───┼───┼───┼───┼───┨\n")?;
            }
        }

        write!(f, "┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n")?;

        Ok(())
    }
}
