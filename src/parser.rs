// Absolute imports within crate
use crate::board;
use crate::piece;
// Relative imports of sub modules
use field_iterator::FieldIterator;
pub use parse_error::ParseError;
mod field_iterator;
mod parse_error;

pub fn parse_forsyth_edwards_notation(record: &str) -> Result<board::Board, ParseError> {
    // Deconstruct specification into the different fields
    let mut field_iterator = FieldIterator::new(record);

    // Collect all the pieces
    let field = field_iterator.next()?;
    let squares = parse_piece_placement(field)?;

    // Detect active color
    let field = field_iterator.next()?;
    let active_color = parse_active_color(field)?;

    // Collect all castling options
    let field = field_iterator.next()?;
    let castling_availability = parse_castling_availability(field)?;

    // Detect en passant target square
    let field = field_iterator.next()?;
    let _en_passant_target_square = parse_en_passant_target_square(field)?;

    // TODO: create struct for target square
    // TODO: implement parse function for halfmove clock and fullmove number

    Ok(board::Board::new(squares, active_color, castling_availability))
}

fn parse_piece_placement(piece_placement_field: &str) -> Result<board::Squares, ParseError> {
    // Start with empty squares
    let mut squares = board::Squares::new();

    // Go from highest rank to lowest, and from lowest file to highest
    let mut rank: i8 = 8;
    let mut file: i8 = 1;

    // Loop over characters until a space is found
    for character in piece_placement_field.chars() {
        match character {
            // Slash indicates end of current rank,
            '/' => {
                // Verify whether we didn't miss any file
                if file != 9 {
                    return Err(ParseError::IncompletePiecePlacement);
                }
                // Go down one rank and reset file
                file = 1;
                rank -= 1;
            }
            // A number indicates the amount of empty squares, increase file by that number
            '1'..='8' => {
                // Parse the character as a digit from 1 to 8
                let offset = character
                    .to_digit(10)
                    .expect("'1'..='8' should always parse to digit successfully") as i8;
                // Increase file count by offset
                file += offset;
            }
            // Any character implies a piece on the current square, so  and increase file
            _ => {
                // Create a new square and piece
                let square = board::Square::new(file, rank);
                let piece = parse_piece(character)?;
                // Place piece on a square
                squares.insert(square, piece);
                // Increase file count
                file += 1;
            }
        }
    }

    // Verify whether we have gone through all the squares
    if rank == 1 && file == 9 {
        Ok(squares)
    } else {
        Err(ParseError::IncompletePiecePlacement)
    }
}

fn parse_piece(character: char) -> Result<piece::Piece, ParseError> {
    // Pull Piece, Color, and Kind into scope for this function only
    use piece::{Color, Kind, Piece};

    // Return new piece
    match character {
        'B' => Ok(Piece::new(Color::White, Kind::Bishop)),
        'b' => Ok(Piece::new(Color::Black, Kind::Bishop)),
        'K' => Ok(Piece::new(Color::White, Kind::King)),
        'k' => Ok(Piece::new(Color::Black, Kind::King)),
        'N' => Ok(Piece::new(Color::White, Kind::Knight)),
        'n' => Ok(Piece::new(Color::Black, Kind::Knight)),
        'P' => Ok(Piece::new(Color::White, Kind::Pawn)),
        'p' => Ok(Piece::new(Color::Black, Kind::Pawn)),
        'Q' => Ok(Piece::new(Color::White, Kind::Queen)),
        'q' => Ok(Piece::new(Color::Black, Kind::Queen)),
        'R' => Ok(Piece::new(Color::White, Kind::Rook)),
        'r' => Ok(Piece::new(Color::Black, Kind::Rook)),
        _ => Err(ParseError::InvalidPiece(character)),
    }
}

fn parse_active_color(active_color_field: &str) -> Result<piece::Color, ParseError> {
    // Detect whether it is the turn of black or white
    match active_color_field.chars().nth(0) {
        Some(character) => match character {
            // Blacks turn to move
            'b' => Ok(piece::Color::Black),
            // Whites turn to move
            'w' => Ok(piece::Color::White),
            // Invalid character
            _ => Err(ParseError::InvalidColor(character)),
        },
        // End of specification reached too early
        None => Err(ParseError::UnexpectedEnd),
    }
}

fn parse_castling_availability(castling_availability_field: &str) -> Result<board::CastlingAvailability, ParseError> {
    // By default, no castling is allowed
    let mut white_kingside = false;
    let mut white_queenside = false;
    let mut black_kingside = false;
    let mut black_queenside = false;

    for character in castling_availability_field.chars() {
        match character {
            // A dash indicates that no player can castle anymore
            '-' => {
                break;
            }
            // Letter K indicates white can castle kingside
            'K' => {
                white_kingside = true;
            }
            // Letter Q indicates white can castle queenside
            'Q' => {
                white_queenside = true;
            }
            // Letter k indicates black can castle kingside
            'k' => {
                black_kingside = true;
            }
            // Letter q indicates black can castle queenside
            'q' => {
                black_queenside = true;
            }
            // Invalid character
            _ => {
                return Err(ParseError::InvalidCastling(character));
            }
        }
    }

    Ok(board::CastlingAvailability::new(
        white_kingside,
        white_queenside,
        black_kingside,
        black_queenside,
    ))
}

fn parse_en_passant_target_square(en_passant_target_square_field: &str) -> Result<Option<&str>, ParseError> {
    // Detect whether it is the turn of black or white
    match en_passant_target_square_field.chars().nth(0) {
        Some(character) => match character {
            // First character should be a file
            // TODO: read second character and convert it to a custom Target type
            'a'..='h' => Ok(Some(en_passant_target_square_field)),
            // No en passant target square, so Ok (instead of Err) and None (instead of Some)
            '-' => Ok(None),
            // Any other character is error
            _ => Err(ParseError::InvalidFile(character)),
        },
        // End of specification reached too early
        None => Err(ParseError::UnexpectedEnd),
    }
}
