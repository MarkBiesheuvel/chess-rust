#[derive(Debug)]
struct Piece {
    piece_type: char,
    has_moved: bool,
}

impl Piece {
    fn new(piece_type: char) -> Self {
        Self {
            piece_type,
            has_moved: false,
        }
    }

    fn make_move(&mut self) {
        self.has_moved = true;
    }

    fn legal_moves(&self) -> Vec<(i8, i8)> {
        // TODO: create tuple struct for move
        // TODO: implement for other pieces
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
}

fn main() {
    let mut rook = Piece::new('R');
    let bishop = Piece::new('B');
    let knight = Piece::new('N');

    // Move rook
    rook.make_move();

    // Print all pieces
    println!("{:?}", rook);
    println!("{:?}", bishop);
    println!("{:?}", knight);

    // Show legal moves of knight
    println!("{:?}", knight.legal_moves());

    // Avoid dead code warning
    bishop.piece_type;
    bishop.has_moved;
}
