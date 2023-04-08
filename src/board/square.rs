// External imports
use std::{fmt, ops, str};

// Imports from super
use super::{File, Offset, Rank};
use crate::parser::ParseError;

/// A single square on the chess board
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Square {
    file: File,
    rank: Rank,
}

impl Square {
    /// Create new Square
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{File, Rank, Square};
    ///
    /// // Eighth file, seventh rank is h7
    /// let file = File::from(8);
    /// let rank = Rank::from(7);
    /// let square = Square::new(file, rank);
    /// assert_eq!(square.to_string(), "h7");
    /// ```
    pub fn new(file: File, rank: Rank) -> Square {
        Square { file, rank }
    }

    /// Get the file of this Square
    pub fn file(&self) -> &File {
        &self.file
    }

    /// Get the rank of this Square
    pub fn rank(&self) -> &Rank {
        &self.rank
    }
}

impl<F, R> From<(F, R)> for Square
where
    F: Into<File>,
    R: Into<Rank>,
{
    /// Create new Square from a tuple of items which can be converted into File and Rank
    ///
    /// ## Examples
    /// ```
    /// use chess::board::Square;
    ///
    /// // Second file, fifth rank is b5
    /// let square = Square::from((2, 5));
    /// assert_eq!(square.to_string(), "b5");
    /// ```
    fn from((file, rank): (F, R)) -> Self {
        Square::new(file.into(), rank.into())
    }
}

impl str::FromStr for Square {
    type Err = ParseError;

    /// Parse string as Square
    ///
    /// ## Examples
    /// ```
    /// use chess::board::Square;
    ///
    /// // The e2 square
    /// let text = "e2";
    /// let square = text.parse::<Square>()?;
    ///
    /// assert_eq!(square.to_string(), text);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // Split up the string into individual characters
        let mut characters = string.chars();

        let first_character = characters.next().ok_or(ParseError::UnexpectedEnd)?;
        let file = match first_character {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => {
                // Any other character is error
                return Err(ParseError::InvalidFile(first_character));
            }
        };

        let second_character = characters.next().ok_or(ParseError::UnexpectedEnd)?;
        let rank = match second_character {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => {
                // Any other character is error
                return Err(ParseError::InvalidRank(second_character));
            }
        };

        let square = Square::from((file, rank));
        Ok(square)
    }
}

impl ops::Add<Offset> for &Square {
    type Output = Square;

    /// Add an offset to a square
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Square, Offset};
    ///
    /// // Start at g8
    /// let starting_square = Square::from((7, 8));
    /// assert_eq!(starting_square.to_string(), "g8");
    ///
    /// // Make a knight move to f6
    /// let offset = Offset::new(-1 , -2);
    /// let destination_square = &starting_square + offset;
    /// assert_eq!(destination_square.to_string(), "f6");
    /// ```
    fn add(self, offset: Offset) -> Self::Output {
        Square::new(&self.file + offset.file(), &self.rank + offset.rank())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}
