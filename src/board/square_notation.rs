use super::Square;

/// Easier way of creating a Square
///
/// ## Examples
/// ```
/// use chess::board::{File, Rank, Square, SquareNotation::*};
///
/// // Create the C4 square
/// let square = Square::from(C4);
///
/// // File should be 3 and rank should be 4
/// assert_eq!(square.file(), File::from(3));
/// assert_eq!(square.rank(), Rank::from(4));
/// ```
#[allow(missing_docs)]
pub enum SquareNotation {
    // A file
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    // B file
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    // C file
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    // D file
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    // E file
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    // F file
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    // G file
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    // H file
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

impl From<SquareNotation> for Square {
    fn from(value: SquareNotation) -> Square {
        // Avoid having to use fully qualified names
        use SquareNotation::*;

        let tuple = match value {
            A1 => (1, 1),
            A2 => (1, 2),
            A3 => (1, 3),
            A4 => (1, 4),
            A5 => (1, 5),
            A6 => (1, 6),
            A7 => (1, 7),
            A8 => (1, 8),
            B1 => (2, 1),
            B2 => (2, 2),
            B3 => (2, 3),
            B4 => (2, 4),
            B5 => (2, 5),
            B6 => (2, 6),
            B7 => (2, 7),
            B8 => (2, 8),
            C1 => (3, 1),
            C2 => (3, 2),
            C3 => (3, 3),
            C4 => (3, 4),
            C5 => (3, 5),
            C6 => (3, 6),
            C7 => (3, 7),
            C8 => (3, 8),
            D1 => (4, 1),
            D2 => (4, 2),
            D3 => (4, 3),
            D4 => (4, 4),
            D5 => (4, 5),
            D6 => (4, 6),
            D7 => (4, 7),
            D8 => (4, 8),
            E1 => (5, 1),
            E2 => (5, 2),
            E3 => (5, 3),
            E4 => (5, 4),
            E5 => (5, 5),
            E6 => (5, 6),
            E7 => (5, 7),
            E8 => (5, 8),
            F1 => (6, 1),
            F2 => (6, 2),
            F3 => (6, 3),
            F4 => (6, 4),
            F5 => (6, 5),
            F6 => (6, 6),
            F7 => (6, 7),
            F8 => (6, 8),
            G1 => (7, 1),
            G2 => (7, 2),
            G3 => (7, 3),
            G4 => (7, 4),
            G5 => (7, 5),
            G6 => (7, 6),
            G7 => (7, 7),
            G8 => (7, 8),
            H1 => (8, 1),
            H2 => (8, 2),
            H3 => (8, 3),
            H4 => (8, 4),
            H5 => (8, 5),
            H6 => (8, 6),
            H7 => (8, 7),
            H8 => (8, 8),
        };

        Square::from(tuple)
    }
}
