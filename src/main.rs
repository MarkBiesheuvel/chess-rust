use chess::Board;
use chess::Piece;

mod chess;

fn main() {
    let _board = Board {};

    let mut pieces = [
        Piece::new_bishop(),
        Piece::new_king(),
        Piece::new_knight(),
        Piece::new_pawn(),
        Piece::new_queen(),
        Piece::new_rook(),
    ];

    // Print all pieces
    for piece in &pieces {
        println!("{:?}", piece);
    }

    // Move all pieces
    for piece in &mut pieces {
        piece.make_move();
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
