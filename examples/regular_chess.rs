use std::error::Error;
use chess::piece::Piece;
use chess::piece::Color::{Black, White};
use chess::piece::behavior::{Knight, Queen};

fn main() -> Result<(), Box<dyn Error>> {
    let white_knight = Piece::new(White, Knight);
    let black_queen = Piece::new(Black, Queen);
    
    let collection = Vec::from([
        white_knight,
        black_queen,
    ]);

    dbg!(collection);

    Ok(())
}
