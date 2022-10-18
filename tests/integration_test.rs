use chess::board::Board;
use chess::parser::ParseError;

#[test]
fn invalid_piece_error() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBXKBNR w KQkq - 0 1");

    // The record contains an X in the piece placement field, which is invalid
    assert_eq!(result, Err(ParseError::InvalidPiece('X')));
}

#[test]
fn missing_white_pawn() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing a white pawn
    assert_eq!(result, Err(ParseError::IncompletePiecePlacement));
}

#[test]
fn missing_empty_square() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing an empty square
    assert_eq!(result, Err(ParseError::IncompletePiecePlacement));
}

#[test]
fn missing_rank() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing an entire rank
    assert_eq!(result, Err(ParseError::IncompletePiecePlacement));
}

#[test]
fn missing_white_knight() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RBQKBNR w KQkq - 0 1");

    // The record contains is missing a white knight
    assert_eq!(result, Err(ParseError::IncompletePiecePlacement));
}

#[test]
fn invalid_active_color() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1");

    // The record contains an x in the active color field, which is invalid
    assert_eq!(result, Err(ParseError::InvalidColor('x')));
}

#[test]
fn invalid_castling_availability() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1");

    // The record contains an x in the castling availability field, which is invalid
    assert_eq!(result, Err(ParseError::InvalidCastling('x')));
}

#[test]
fn unexpected_end() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");

    // The record is missing the last fields
    assert_eq!(result, Err(ParseError::UnexpectedEnd));
}

#[test]
fn valid_record() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkb1r/ppp1pppp/8/3pP3/6n1/8/PPPPKPPP/RNBQ1BNR w kq d6 0 4");

    // This is a valid record so should return an Ok Result
    assert!(matches!(result, Ok(_)));
}

#[test]
fn starting_position() {
    // Starting position returns a Board without it being wrapped in a Result
    let board = Board::starting_position();

    // Should start with 32 pieces on the board
    assert_eq!(board.pieces().count(), 32);
    // Should start with 16 white pieces
    assert_eq!(board.white_pieces().count(), 16);
    // Should start with 16 black pieces
    assert_eq!(board.black_pieces().count(), 16);
}
