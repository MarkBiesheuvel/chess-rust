// Absolute imports within crate
use crate::piece::Color;

/// Type to indicate whether castling is available for the either player on either king- or queenside
///
/// ## Examples
/// ```
/// use chess::piece::Color::*;
/// use chess::game::CastlingAvailability;
///
/// // Start with all castling available
/// let mut availablity = CastlingAvailability::default();
///
/// // Assume white's king moved
/// availablity.disable_both(&White);
///
/// // Asume black's rook moved
/// availablity.disable_kingside(&Black);
///
/// // Assertions
/// assert_eq!(availablity.is_long_castle_available(&White), false);
/// assert_eq!(availablity.is_short_castle_available(&White), false);
/// assert_eq!(availablity.is_long_castle_available(&Black), true);
/// assert_eq!(availablity.is_short_castle_available(&Black), false);
/// ```
#[derive(Debug)]
pub struct CastlingAvailability {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl Default for CastlingAvailability {
    /// Overwrite default implementation of Default to set all booleans to true
    fn default() -> Self {
        CastlingAvailability::new(true, true, true, true)
    }
}

impl CastlingAvailability {
    /// Create a new CastlingAvailability using the raw boolean values
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

    /// Whether short castle is available for specific color
    pub fn is_short_castle_available(&self, color: &Color) -> bool {
        match color {
            Color::White => self.white_kingside,
            Color::Black => self.black_kingside,
        }
    }

    /// Whether long castle is available for specific color
    pub fn is_long_castle_available(&self, color: &Color) -> bool {
        match color {
            Color::White => self.white_queenside,
            Color::Black => self.black_queenside,
        }
    }

    /// Disable all castling for color
    pub fn disable_both(&mut self, color: &Color) {
        self.disable_kingside(color);
        self.disable_queenside(color);
    }

    /// Disable short castling for color
    pub fn disable_kingside(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.white_kingside = false;
            }
            Color::Black => {
                self.black_kingside = false;
            }
        }
    }

    /// Disable long castling for color
    pub fn disable_queenside(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.white_queenside = false;
            }
            Color::Black => {
                self.black_queenside = false;
            }
        }
    }
}
