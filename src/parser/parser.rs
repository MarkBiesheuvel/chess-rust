// Absolute imports within crate
use crate::board::{Board, CastlingAvailability, PiecePlacement, Square};
use crate::piece::{Color, Kind, Piece};

// Relative imports of sub modules
pub use parse_error::ParseError;
mod parse_error;

// TODO: implement a function on Iterator which returns a parse error

pub fn parse_forsyth_edwards_notation(record: &str) -> Result<Board, ParseError> {
    // Deconstruct specification into the different fields
    let mut fields = record.split_whitespace();

    // Collect all the pieces
    let field = fields.next().ok_or_else(|| ParseError::UnexpectedEnd)?;
    let piece_placement = parse_piece_placement(field)?;

    // Detect active color
    let field = fields.next().ok_or_else(|| ParseError::UnexpectedEnd)?;
    let active_color = parse_active_color(field)?;

    // Collect all castling options
    let field = fields.next().ok_or_else(|| ParseError::UnexpectedEnd)?;
    let castling_availability = parse_castling_availability(field)?;

    // Detect en passant target square
    let field = fields.next().ok_or_else(|| ParseError::UnexpectedEnd)?;
    let en_passant_target = parse_en_passant_target_square(field)?;

    // Detect halfmove clock
    let field = fields.next().ok_or_else(|| ParseError::UnexpectedEnd)?;
    let halfmove_clock = parse_number(field)?;

    // Detect fullmove number
    let field = fields.next().ok_or_else(|| ParseError::UnexpectedEnd)?;
    let fullmove_number = parse_number(field)?;

    Ok(Board::new(
        piece_placement,
        active_color,
        castling_availability,
        en_passant_target,
        halfmove_clock,
        fullmove_number,
    ))
}

fn parse_piece_placement(piece_placement_field: &str) -> Result<PiecePlacement, ParseError> {
    // Start with empty squares
    let mut piece_placement = PiecePlacement::new();

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
            '1' => {
                file += 1;
            }
            '2' => {
                file += 2;
            }
            '3' => {
                file += 3;
            }
            '4' => {
                file += 4;
            }
            '5' => {
                file += 5;
            }
            '6' => {
                file += 6;
            }
            '7' => {
                file += 7;
            }
            '8' => {
                file += 8;
            }
            // Any character implies a piece on the current square, so  and increase file
            _ => {
                // Create a new square and piece
                let square = Square::new(file, rank);
                let piece = parse_piece(character)?;

                // Place piece on a square
                piece_placement.insert(square, piece);

                // Increase file count
                file += 1;
            }
        }
    }

    // Verify whether we have gone through all the squares
    if rank == 1 && file == 9 {
        Ok(piece_placement)
    } else {
        Err(ParseError::IncompletePiecePlacement)
    }
}

fn parse_piece(character: char) -> Result<Piece, ParseError> {
    // Return new piece
    let piece = match character {
        'B' => Piece::new(Color::White, Kind::Bishop),
        'b' => Piece::new(Color::Black, Kind::Bishop),
        'K' => Piece::new(Color::White, Kind::King),
        'k' => Piece::new(Color::Black, Kind::King),
        'N' => Piece::new(Color::White, Kind::Knight),
        'n' => Piece::new(Color::Black, Kind::Knight),
        'P' => Piece::new(Color::White, Kind::Pawn),
        'p' => Piece::new(Color::Black, Kind::Pawn),
        'Q' => Piece::new(Color::White, Kind::Queen),
        'q' => Piece::new(Color::Black, Kind::Queen),
        'R' => Piece::new(Color::White, Kind::Rook),
        'r' => Piece::new(Color::Black, Kind::Rook),
        _ => {
            return Err(ParseError::InvalidPiece(character));
        }
    };

    Ok(piece)
}

fn parse_active_color(active_color_field: &str) -> Result<Color, ParseError> {
    // Detect whether it is the turn of black or white
    let character = active_color_field
        .chars()
        .nth(0)
        .ok_or_else(|| ParseError::UnexpectedEnd)?;

    let color = match character {
        // Blacks turn to move
        'b' => Color::Black,
        // Whites turn to move
        'w' => Color::White,
        // Invalid character
        _ => {
            return Err(ParseError::InvalidColor(character));
        }
    };

    Ok(color)
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

    Ok(CastlingAvailability::new(white_kingside, white_queenside, black_kingside, black_queenside))
}

fn parse_en_passant_target_square(en_passant_target_square_field: &str) -> Result<Option<Square>, ParseError> {
    // Detect the target square for en passant
    let mut characters = en_passant_target_square_field.chars();

    let first_character = characters.next().ok_or_else(|| ParseError::UnexpectedEnd)?;

    // Handle special case, no en passant
    if first_character == '-' {
        return Ok(None);
    }

    let second_character = characters.next().ok_or_else(|| ParseError::UnexpectedEnd)?;

    // Expecting valid file and rank now
    let file = parse_file(first_character)?;
    let rank = parse_rank(second_character)?;

    Ok(Some(Square::new(file, rank)))
}

fn parse_file(character: char) -> Result<i8, ParseError> {
    let file = match character {
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
            return Err(ParseError::InvalidFile(character));
        }
    };

    Ok(file)
}

fn parse_rank(character: char) -> Result<i8, ParseError> {
    let rank = match character {
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
            return Err(ParseError::InvalidRank(character));
        }
    };

    Ok(rank)
}

fn parse_number(field: &str) -> Result<u16, ParseError> {
    field.parse().map_err(|_| ParseError::InvalidNumber)
}
