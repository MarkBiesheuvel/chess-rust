use chess::board::Board;
use chess::parser::FenParserError;

#[test]
fn invalid_piece_error() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBXKBNR w KQkq - 0 1");

    // The record contains an X in the piece placement field, which is invalid
    assert_eq!(result, Err(FenParserError::InvalidPiece('X')));
}

#[test]
fn missing_white_pawn() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing a white pawn
    assert_eq!(result, Err(FenParserError::IncompletePiecePlacement));
}

#[test]
fn missing_empty_square() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing an empty square
    assert_eq!(result, Err(FenParserError::IncompletePiecePlacement));
}

#[test]
fn missing_rank() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing an entire rank
    assert_eq!(result, Err(FenParserError::IncompletePiecePlacement));
}

#[test]
fn missing_white_knight() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RBQKBNR w KQkq - 0 1");

    // The record contains is missing a white knight
    assert_eq!(result, Err(FenParserError::IncompletePiecePlacement));
}

#[test]
fn invalid_active_color() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1");

    // The record contains an x in the active color field, which is invalid
    assert_eq!(result, Err(FenParserError::InvalidColor('x')));
}

#[test]
fn invalid_castling_availability() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1");

    // The record contains an x in the castling availability field, which is invalid
    assert_eq!(result, Err(FenParserError::InvalidCastling('x')));
}

#[test]
fn unexpected_end() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");

    // The record is missing the last fields
    assert_eq!(result, Err(FenParserError::UnexpectedEnd));
}

#[test]
fn valid_record() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // This is a valid record so should return an Ok Result
    assert!(matches!(result, Ok(_)));
}
