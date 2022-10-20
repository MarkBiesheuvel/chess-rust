// Absolute imports within crate
use crate::piece;

// Type to indicate whether castling is available for the either player on either king- or queenside
#[derive(Debug)]
pub struct CastlingAvailability {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}
impl CastlingAvailability {
    pub fn new(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> CastlingAvailability {
        CastlingAvailability {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }

    pub fn is_short_castle_available(&self, color: &piece::Color) -> bool {
        match color {
            piece::Color::White => self.white_kingside,
            piece::Color::Black => self.black_kingside,
        }
    }

    pub fn is_long_castle_available(&self, color: &piece::Color) -> bool {
        match color {
            piece::Color::White => self.white_queenside,
            piece::Color::Black => self.black_queenside,
        }
    }
}
