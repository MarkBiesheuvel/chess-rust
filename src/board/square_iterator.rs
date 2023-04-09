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
    /// use chess::board::{Offset, SquareIterator, Square};
    ///
    /// // Start at b8 and move like a knight
    /// let origin = "b8".parse()?;
    /// let offset = Offset::new(1, -2);
    /// let mut iter = SquareIterator::from_single_offset(origin, offset);
    ///
    /// // Line should contain only one square
    /// assert_eq!(iter.next().unwrap().to_string(), "c6");
    /// assert_eq!(iter.next(), None);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_single_offset(origin: Square, offset: Offset) -> SquareIterator {
        // Calculate destination
        let destination = &origin + offset;

        // Create list of length 1
        let list = vec![destination];

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
    /// let direction = Direction::DiagonalRightUp;
    /// let mut iter = SquareIterator::from_direction(Rc::new(origin), direction);
    ///
    /// // Line should contain these squares
    /// assert_eq!(iter.next().unwrap().to_string(), "d2");
    /// assert_eq!(iter.next().unwrap().to_string(), "e3");
    /// assert_eq!(iter.next().unwrap().to_string(), "f4");
    /// assert_eq!(iter.next().unwrap().to_string(), "g5");
    /// assert_eq!(iter.next().unwrap().to_string(), "h6");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_direction(origin: Rc<Square>, direction: Direction) -> SquareIterator {
        // Create reference counted variable
        let direction = Rc::new(direction);

        let iter = (1..).map(move |i| {
            // Calculate offset for i
            let offset = Offset::from(direction.as_ref()) * i;

            // Return desitnation
            origin.as_ref() + offset
        });

        SquareIterator::new(iter)
    }
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        self.0.next()
    }
}
