// External imports
use std::fmt;
// Absolute imports within crate
use crate::board;

impl fmt::Display for board::Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write top border of the board
        write!(f, "┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓\n")?;
        // Iterate over rows (ranks)
        for rank in (0..8).rev() {
            // Write left border of a row
            write!(f, "┃ ")?;
            // Iterate over columns (files) in a row (rank)
            for file in 0..8 {
                // Write piece if it exists
                let square = board::Square::new(file, rank);
                match self.squares.get(&square) {
                    Some(piece) => {
                        write!(f, "{}", piece.symbol())?;
                    }
                    None => {
                        write!(f, " ")?;
                    }
                };
                // Write columns separator
                if file < 7 {
                    write!(f, " │ ")?;
                }
            }
            // Write right border of a row
            write!(f, " ┃\n")?;
            // Write row separator
            if rank > 0 {
                write!(f, "┠───┼───┼───┼───┼───┼───┼───┼───┨\n")?;
            }
        }
        // Write bottom border of the board
        write!(f, "┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n")?;

        // If we reached this point, none of the writes failed
        Ok(())
    }
}
