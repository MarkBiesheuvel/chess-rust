use chess::Board;
use chess::Piece;
use chess::PieceKind::{Bishop, King, Knight, Pawn, Queen, Rook};

mod chess;

fn main() {
    let _board = Board::new();

    let mut pieces = [
        Piece::new(Bishop),
        Piece::new(King),
        Piece::new(Knight),
        Piece::new(Pawn),
        Piece::new(Queen),
        Piece::new(Rook),
    ];

    // Print all pieces
    for piece in &pieces {
        println!("{:?}", piece);
    }

    // Move all pieces
    for piece in &mut pieces {
        piece.update();
    }

    // Print all pieces
    for piece in &pieces {
        println!("{:?}", piece);
    }

    // Show legal moves of all pieces
    for piece in &pieces {
        println!("{:?}", piece.legal_moves());
    }
}
