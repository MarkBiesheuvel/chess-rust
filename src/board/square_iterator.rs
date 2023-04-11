// External imports
use std::rc::Rc;

// Imports from super
use super::{Direction, Offset, Square};

/// An iterator of squares
pub struct SquareIterator(Box<dyn Iterator<Item = Square>>);

impl SquareIterator {
    fn new(iter: impl IntoIterator<Item = Square> + 'static) -> SquareIterator {
        SquareIterator(Box::new(iter.into_iter()))
    }

    /// Create a SquareIterator which has a single destination (like a knights jump)
    ///
    /// ## Examples
    /// ```
    /// use std::rc::Rc;
    /// use chess::board::{Offset, SquareIterator, Square};
    ///
    /// // Start at b8 and move like a knight
    /// let origin = "b8".parse()?;
    /// let origin = Rc::new(origin);
    /// let offset = Offset::new(1, -2);
    /// let mut iter = SquareIterator::from_single_offset(origin, offset);
    ///
    /// // Line should contain only one square
    /// assert_eq!(iter.next(), "c6".parse().ok());
    /// assert_eq!(iter.next(), None);
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_single_offset(origin: Rc<Square>, offset: Offset) -> SquareIterator {
        // Calculate destination
        let destination = origin.as_ref() + offset;

        // Create list of length 1
        let list = Vec::from([destination]);

        // Turn into iterator
        SquareIterator::new(list)
    }

    /// Create a SquareIterator which originates from the origin square and moves into the direction
    ///
    /// ## Examples
    /// ```
    /// use std::rc::Rc;
    /// use chess::board::{Direction, SquareIterator, Square};
    ///
    /// // Start at c1 and move like a bishop
    /// let origin = "c1".parse()?;
    /// let origin = Rc::new(origin);
    /// let direction = Direction::DiagonalRightUp;
    /// let mut iter = SquareIterator::from_direction(origin, direction);
    ///
    /// // Line should contain these squares
    /// assert_eq!(iter.next(), "d2".parse().ok());
    /// assert_eq!(iter.next(), "e3".parse().ok());
    /// assert_eq!(iter.next(), "f4".parse().ok());
    /// assert_eq!(iter.next(), "g5".parse().ok());
    /// assert_eq!(iter.next(), "h6".parse().ok());
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_direction(origin: Rc<Square>, direction: Direction) -> SquareIterator {
        // Create reference counted variable
        let direction = Rc::new(direction);

        let iter = (1..).map(move |i| {
            // Calculate offset for i
            let offset = Offset::from(direction.as_ref()) * i;

            // Calculate destination
            origin.as_ref() + offset
        });

        SquareIterator::new(iter)
    }

    /// Return a new SquareIterator with a hard limit
    ///
    /// This is useful for pawn moves (1 or 2 forward), or king moves (1 in any direction)
    ///
    /// ## Examples
    /// ```
    /// use std::rc::Rc;
    /// use chess::board::{Direction, SquareIterator, Square};
    ///
    /// // Start at e2 and move like a pawn on the first turn
    /// let origin = "e2".parse()?;
    /// let origin = Rc::new(origin);
    /// let direction = Direction::VerticalUp;
    /// let mut iter = SquareIterator::from_direction(origin, direction).limit(2);
    ///
    /// // Line should contain these squares
    /// assert_eq!(iter.next(), "e3".parse().ok());
    /// assert_eq!(iter.next(), "e4".parse().ok());
    /// assert_eq!(iter.next(), None);
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn limit(self, limit: usize) -> SquareIterator {
        SquareIterator::new(self.0.take(limit))
    }
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        self.0.next()
    }
}
