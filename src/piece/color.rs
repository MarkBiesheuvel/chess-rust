// Absolute imports within crate
use crate::board::Rank;

/// The two colors of pieces available: black and white
#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    /// The black pieces, or the player playing with the black pieces
    Black,
    /// The white pieces, or the player playing with the white pieces
    White,
}

impl Color {
    /// The starting rank of the minor and major pieces
    ///
    /// ## Examples
    /// ```
    /// use chess::piece::Color;
    ///
    /// // Black's king starts on the 8th rank
    /// let color = Color::Black;
    /// assert_eq!(color.first_rank().to_string(), "8");
    /// ```
    pub fn first_rank(&self) -> Rank {
        match self {
            Color::White => Rank::from(1),
            Color::Black => Rank::from(8),
        }
    }

    /// The starting rank of the pawns
    ///
    /// ## Examples
    /// ```
    /// use chess::piece::Color;
    ///
    /// // White's pawns starts on the 2nd rank
    /// let color = Color::White;
    /// assert_eq!(color.second_rank().to_string(), "2");
    /// ```
    pub fn second_rank(&self) -> Rank {
        match self {
            Color::White => Rank::from(2),
            Color::Black => Rank::from(7),
        }
    }

    /// The rank at which pawns promote
    ///
    /// ## Examples
    /// ```
    /// use chess::piece::Color;
    ///
    /// // Black's pawns promote on the first rank
    /// let color = Color::Black;
    /// assert_eq!(color.promotion_rank().to_string(), "1");
    /// ```
    pub fn promotion_rank(&self) -> Rank {
        match self {
            Color::White => Rank::from(8),
            Color::Black => Rank::from(1),
        }
    }
}
