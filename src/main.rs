use chess::Board;
use chess::Piece;

mod chess;

fn main() {
    let mut rook = Piece::Rook { has_moved: false };
    let mut bishop = Piece::Bishop;
    let mut knight = Piece::Knight;
    let _pawn = Piece::Pawn { has_moved: false };
    let _king = Piece::King { has_moved: false };
    let _queen = Piece::Queen;
    let _board = Board {};

    // Move all pieces
    rook.make_move();
    bishop.make_move();
    knight.make_move();

    // Print all pieces
    println!("{:?}", rook);
    println!("{:?}", bishop);
    println!("{:?}", knight);

    // Show legal moves of all pieces
    println!("{:?}", rook.legal_moves());
    println!("{:?}", bishop.legal_moves());
    println!("{:?}", knight.legal_moves());
}
