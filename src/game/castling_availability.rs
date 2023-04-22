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

impl Default for CastlingAvailability {
    fn default() -> Self {
        CastlingAvailability::new(true, true, true, true)
    }
}

impl CastlingAvailability {
    pub fn new(
        white_kingside: bool, white_queenside: bool, black_kingside: bool, black_queenside: bool,
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

    pub fn disable_both(&mut self, color: &piece::Color) {
        self.disable_kingside(color);
        self.disable_queenside(color);
    }

    pub fn disable_kingside(&mut self, color: &piece::Color) {
        match color {
            piece::Color::White => {
                self.white_kingside = false;
            }
            piece::Color::Black => {
                self.black_kingside = false;
            }
        }
    }

    pub fn disable_queenside(&mut self, color: &piece::Color) {
        match color {
            piece::Color::White => {
                self.white_queenside = false;
            }
            piece::Color::Black => {
                self.black_queenside = false;
            }
        }
    }
}
