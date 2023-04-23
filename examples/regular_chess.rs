use std::error::Error;
use chess::piece::{Piece, Color::*, behavior::*};

fn main() -> Result<(), Box<dyn Error>> {
    let white_knight = Piece::new((2, 1), White, Knight);
    let black_queen = Piece::new((3, 3), Black, Queen);
    
    let collection = Vec::from([
        white_knight,
        black_queen,
    ]);

    dbg!(collection);

    Ok(())
}
