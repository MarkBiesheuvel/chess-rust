// Absolute imports within crate
use crate::board::{Board, CastlingAvailability, Square, Squares};
use crate::piece::{Color, Kind, Piece};
// Relative imports of sub modules
use field_iterator::FieldIterator;
pub use parse_error::ParseError;
mod field_iterator;
mod parse_error;

pub fn parse_forsyth_edwards_notation(record: &str) -> Result<Board, ParseError> {
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
    let en_passant_target = parse_en_passant_target_square(field)?;

    // TODO: create struct for target square
    // TODO: implement parse function for halfmove clock and fullmove number

    Ok(Board::new(
        squares,
        active_color,
        castling_availability,
        en_passant_target,
    ))
}

fn parse_piece_placement(piece_placement_field: &str) -> Result<Squares, ParseError> {
    // Start with empty squares
    let mut squares = Squares::new();

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
                let square = Square::new(file, rank);
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

fn parse_piece(character: char) -> Result<Piece, ParseError> {
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

fn parse_active_color(active_color_field: &str) -> Result<Color, ParseError> {
    // Detect whether it is the turn of black or white
    match active_color_field.chars().nth(0) {
        Some(character) => match character {
            // Blacks turn to move
            'b' => Ok(Color::Black),
            // Whites turn to move
            'w' => Ok(Color::White),
            // Invalid character
            _ => Err(ParseError::InvalidColor(character)),
        },
        // End of specification reached too early
        None => Err(ParseError::UnexpectedEnd),
    }
}

fn parse_castling_availability(castling_availability_field: &str) -> Result<CastlingAvailability, ParseError> {
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

    Ok(CastlingAvailability::new(
        white_kingside,
        white_queenside,
        black_kingside,
        black_queenside,
    ))
}

fn parse_en_passant_target_square(en_passant_target_square_field: &str) -> Result<Option<Square>, ParseError> {
    // Detect the target square for en passant
    let mut characters = en_passant_target_square_field.chars();

    let first_character = characters.next();
    let second_character = characters.next();

    // Handle special case, no en passant
    if first_character == Some('-') {
        return Ok(None);
    }

    // Expecting valid file and rank now
    let file = parse_en_passant_target_file(first_character)?;
    let rank = parse_en_passant_target_rank(second_character)?;

    Ok(Some(Square::new(file, rank)))
}

fn parse_en_passant_target_file(character: Option<char>) -> Result<i8, ParseError> {
    match character {
        Some(file) => match file {
            'a' => Ok(1),
            'b' => Ok(2),
            'c' => Ok(3),
            'd' => Ok(4),
            'e' => Ok(5),
            'f' => Ok(6),
            'g' => Ok(7),
            'h' => Ok(8),
            // Any other character is error
            _ => Err(ParseError::InvalidFile(file)),
        },
        // End of specification reached too early
        None => Err(ParseError::UnexpectedEnd),
    }
}

fn parse_en_passant_target_rank(character: Option<char>) -> Result<i8, ParseError> {
    match character {
        Some(rank) => match rank {
            '1'..='8' => Ok(rank
                .to_digit(10)
                .expect("'1'..='8' should always parse to digit successfully") as i8),
            // Any other character is error
            _ => Err(ParseError::InvalidRank(rank)),
        },
        // End of specification reached too early
        None => Err(ParseError::UnexpectedEnd),
    }
}
