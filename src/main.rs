#[derive(Debug)]
enum Piece {
    Pawn { has_moved: bool },
    Knight,
    Bishop,
    Rook { has_moved: bool },
    Queen,
    King { has_moved: bool },
}

impl Piece {
    fn make_move(&mut self) {
        // Update the has_moved property on pieces with special moves, ignore for anything else
        match self {
            Piece::King { has_moved } | Piece::Pawn { has_moved } | Piece::Rook { has_moved } => {
                *has_moved = true;
            }
            _ => {}
        }
    }

    // TODO: create tuple struct for move
    fn legal_moves(&self) -> Vec<(i8, i8)> {
        match self {
            Piece::Knight => {
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

fn main() {
    let mut rook = Piece::Rook { has_moved: false };
    let mut bishop = Piece::Bishop;
    let mut knight = Piece::Knight;
    let _pawn = Piece::Pawn { has_moved: false };
    let _king = Piece::King { has_moved: false };
    let _queen = Piece::Queen;

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
