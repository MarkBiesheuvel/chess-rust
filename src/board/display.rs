// External imports
use std::fmt;
// Imports from parent
use super::{Board, Square};

const ROW_TOP_BORDER___: &str = "  ┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓\n";
const ROW_SEPARATOR____: &str = "  ┠───┼───┼───┼───┼───┼───┼───┼───┨\n";
const ROW_BOTTOM_BORDER: &str = "  ┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n";
const ROW_ANNOTATIONS__: &str = "    A   B   C   D   E   F   G   H\n";
const COLUMN_LEFT_BORDER_: &str = " ┃ ";
const COLUMN_SEPARATOR___: &str = " │ ";
const COLUMN_RIGHT_BORDER: &str = " ┃\n";
const SQUARE_EMPTY: &str = " ";

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write top border of the board
        write!(f, "{}", ROW_TOP_BORDER___)?;
        // Iterate over rows (ranks)
        for rank in (1..=8).rev() {
            // Write left border of a row
            write!(f, "{}{}", rank, COLUMN_LEFT_BORDER_)?;
            // Iterate over columns (files) in a row (rank)
            for file in 1..=8 {
                // Write piece if it exists
                let square = Square::new(file, rank);
                let square = match self.piece_placement.get(&square) {
                    Some(piece) => piece.symbol(),
                    None => SQUARE_EMPTY,
                };
                write!(f, "{}", square)?;
                // Write columns separator
                if file < 8 {
                    write!(f, "{}", COLUMN_SEPARATOR___)?;
                }
            }
            // Write right border of a row
            write!(f, "{}", COLUMN_RIGHT_BORDER)?;
            // Write row separator
            if rank > 1 {
                write!(f, "{}", ROW_SEPARATOR____)?;
            }
        }
        // Write bottom border of the board
        write!(f, "{}", ROW_BOTTOM_BORDER)?;

        // Write annotations of the files at the bottom
        write!(f, "{}", ROW_ANNOTATIONS__)?;

        // If we reached this point, none of the writes failed
        Ok(())
    }
}
