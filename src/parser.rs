use super::board::{Board, CastlingAvailability, Square, Squares};
use super::piece::{Piece, PieceColor, PieceKind};
use std::str::SplitWhitespace;

#[derive(Debug)]
pub enum FenParserError {
    InvalidCastling(char),
    InvalidColor(char),
    InvalidFile(char),
    InvalidPiece(char),
    UnexpectedEnd,
    IncompletePiecePlacement,
}

struct FieldIterator<'a> {
    iter: SplitWhitespace<'a>,
}
impl<'a> FieldIterator<'a> {
    fn new(specification: &'a str) -> FieldIterator<'a> {
        FieldIterator {
            iter: specification.split_whitespace(),
        }
    }

    fn next(&mut self) -> Result<&str, FenParserError> {
        match self.iter.next() {
            Some(field) => Ok(field),
            None => Err(FenParserError::UnexpectedEnd),
        }
    }
}

pub fn parse_forsyth_edwards_notation(record: &str) -> Result<Board, FenParserError> {
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

    Ok(Board::new(squares, active_color, castling_availability))
}

fn parse_piece_placement(piece_placement_field: &str) -> Result<Squares, FenParserError> {
    // Start with empty squares
    let mut squares = [[Square::Empty; 8]; 8];

    // Go from highest rank to lowest, and from lowest file to highest
    let mut rank: usize = 7;
    let mut file: usize = 0;

    // Loop over characters until a space is found
    for character in piece_placement_field.chars() {
        match character {
            // Slash indicates end of current rank,
            '/' => {
                // Verify whether we didn't miss any file
                if file != 8 {
                    return Err(FenParserError::IncompletePiecePlacement);
                }
                // Go down one rank and reset file
                file = 0;
                rank -= 1;
            }
            // A number indicates the amount of empty squares, increase file by that number
            '1'..='8' => {
                // Parse the character as a digit from 1 to 8
                let offset = character
                    .to_digit(10)
                    .expect("'1'..='8' should always parse to digit successfully")
                    as usize;
                // Increase file count by offset
                file += offset;
            }
            // Any character implies a piece on the current square, so  and increase file
            _ => {
                // Create a new piece
                let piece = parse_piece(character)?;
                // Store piece on a square, which is therefore now taken
                squares[rank][file] = Square::Taken(piece);
                // Increase file count
                file += 1;
            }
        }
    }

    // Verify whether we have gone through all the squares
    if rank == 0 && file == 8 {
        Ok(squares)
    } else {
        Err(FenParserError::IncompletePiecePlacement)
    }
}

fn parse_piece(character: char) -> Result<Piece, FenParserError> {
    match character {
        'B' => Ok(Piece::new(PieceColor::White, PieceKind::Bishop)),
        'b' => Ok(Piece::new(PieceColor::Black, PieceKind::Bishop)),
        'K' => Ok(Piece::new(PieceColor::White, PieceKind::King)),
        'k' => Ok(Piece::new(PieceColor::Black, PieceKind::King)),
        'N' => Ok(Piece::new(PieceColor::White, PieceKind::Knight)),
        'n' => Ok(Piece::new(PieceColor::Black, PieceKind::Knight)),
        'P' => Ok(Piece::new(PieceColor::White, PieceKind::Pawn)),
        'p' => Ok(Piece::new(PieceColor::Black, PieceKind::Pawn)),
        'Q' => Ok(Piece::new(PieceColor::White, PieceKind::Queen)),
        'q' => Ok(Piece::new(PieceColor::Black, PieceKind::Queen)),
        'R' => Ok(Piece::new(PieceColor::White, PieceKind::Rook)),
        'r' => Ok(Piece::new(PieceColor::Black, PieceKind::Rook)),
        _ => Err(FenParserError::InvalidPiece(character)),
    }
}

fn parse_active_color(active_color_field: &str) -> Result<PieceColor, FenParserError> {
    // Detect whether it is the turn of black or white
    match active_color_field.chars().nth(0) {
        Some(character) => match character {
            // Blacks turn to move
            'b' => Ok(PieceColor::Black),
            // Whites turn to move
            'w' => Ok(PieceColor::White),
            // Invalid character
            _ => Err(FenParserError::InvalidColor(character)),
        },
        // End of specification reached too early
        None => Err(FenParserError::UnexpectedEnd),
    }
}

fn parse_castling_availability(castling_availability_field: &str) -> Result<CastlingAvailability, FenParserError> {
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
                return Err(FenParserError::InvalidCastling(character));
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

fn parse_en_passant_target_square(en_passant_target_square_field: &str) -> Result<Option<&str>, FenParserError> {
    // Detect whether it is the turn of black or white
    match en_passant_target_square_field.chars().nth(0) {
        Some(character) => match character {
            // First character should be a file
            // TODO: read second character and convert it to a custom Target type
            'a'..='h' => Ok(Some(en_passant_target_square_field)),
            // No en passant target square, so Ok (instead of Err) and None (instead of Some)
            '-' => Ok(None),
            // Any other character is error
            _ => Err(FenParserError::InvalidFile(character)),
        },
        // End of specification reached too early
        None => Err(FenParserError::UnexpectedEnd),
    }
}
