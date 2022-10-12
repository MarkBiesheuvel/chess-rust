use chess::Board;
use chess::Piece;

mod chess;

fn main() {
    let mut rook = Piece::new_rook();
    let mut bishop = Piece::new_bishop();
    let mut knight = Piece::new_knight();
    let mut king = Piece::new_king();
    let _pawn = Piece::new_pawn();
    let _queen = Piece::new_queen();
    let _board = Board {};

    // Move all pieces
    rook.make_move();
    bishop.make_move();
    knight.make_move();
    king.make_move();

    // Print all pieces
    println!("{:?}", rook);
    println!("{:?}", bishop);
    println!("{:?}", knight);
    println!("{:?}", king);

    // Show legal moves of all pieces
    println!("{:?}", rook.legal_moves());
    println!("{:?}", bishop.legal_moves());
    println!("{:?}", knight.legal_moves());
}
