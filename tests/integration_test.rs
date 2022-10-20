use chess::board::Board;
use chess::parser::ParseError;
use chess::piece::Kind;

#[test]
fn invalid_piece_error() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBXKBNR w KQkq - 0 1");

    // The record contains an X in the piece placement field, which is invalid
    match result {
        Err(error) => assert_eq!(error, ParseError::InvalidPiece('X')),
        _ => assert!(false),
    };
}

#[test]
fn missing_white_pawn() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing a white pawn
    match result {
        Err(error) => assert_eq!(error, ParseError::IncompletePiecePlacement),
        _ => assert!(false),
    };
}

#[test]
fn missing_empty_square() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing an empty square
    match result {
        Err(error) => assert_eq!(error, ParseError::IncompletePiecePlacement),
        _ => assert!(false),
    };
}

#[test]
fn missing_rank() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // The record contains is missing an entire rank
    match result {
        Err(error) => assert_eq!(error, ParseError::IncompletePiecePlacement),
        _ => assert!(false),
    };
}

#[test]
fn missing_white_knight() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RBQKBNR w KQkq - 0 1");

    // The record contains is missing a white knight
    match result {
        Err(error) => assert_eq!(error, ParseError::IncompletePiecePlacement),
        _ => assert!(false),
    };
}

#[test]
fn invalid_active_color() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1");

    // The record contains an x in the active color field, which is invalid
    match result {
        Err(error) => assert_eq!(error, ParseError::InvalidColor('x')),
        _ => assert!(false),
    };
}

#[test]
fn invalid_castling_availability() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1");

    // The record contains an x in the castling availability field, which is invalid
    match result {
        Err(error) => assert_eq!(error, ParseError::InvalidCastling('x')),
        _ => assert!(false),
    };
}

#[test]
fn unexpected_end() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");

    // The record is missing the last fields
    match result {
        Err(error) => assert_eq!(error, ParseError::UnexpectedEnd),
        _ => assert!(false),
    };
}

#[test]
fn valid_record_no_captures() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("rnbqkb1r/ppp1pppp/8/3pP3/6n1/8/PPPPKPPP/RNBQ1BNR w kq d6 0 4");

    // This is a valid record
    match result {
        Ok(board) => {
            // Should still have 32 pieces on the board
            assert_eq!(board.pieces().len(), 32);
            // Should still have 16 white pieces
            assert_eq!(board.white_pieces().len(), 16);
            // Should still have 16 black pieces
            assert_eq!(board.black_pieces().len(), 16);
        }
        _ => assert!(false),
    };
}

#[test]
fn valid_record_end_game() {
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let result = Board::forsyth_edwards_notation("3r4/1p3pk1/p4b2/P3p2p/1PP1P1b1/4K1p1/1R6/2r5 b - - 0 37");

    // This is a valid record
    match result {
        Ok(board) => {
            // Should have only 17 pieces left
            assert_eq!(board.pieces().len(), 17);
            // Should have only 6 white pieces left
            assert_eq!(board.white_pieces().len(), 6);
            // Should have only 11 black pieces left
            assert_eq!(board.black_pieces().len(), 11);
        }
        _ => assert!(false),
    };
}

#[test]
fn starting_position() {
    // Starting position returns a Board without it being wrapped in a Result
    let board = Board::starting_position();

    // Should start with 32 pieces on the board
    assert_eq!(board.pieces().len(), 32);
    // Should start with 16 white pieces
    assert_eq!(board.white_pieces().len(), 16);
    // Should start with 16 black pieces
    assert_eq!(board.black_pieces().len(), 16);

    // (8 pawns × 2 moves) + (2 knights × 2 moves) = 20 moves
    assert_eq!(board.legal_moves().len(), 20);

    // Calculate the relative value of all the pieces (except the king)
    let total_value: f32 = board
        .white_pieces()
        .into_iter()
        .filter(|piece| *piece.kind() != Kind::King)
        .map(|piece| (*piece).kind().relative_value())
        .sum();

    // Take floating point erros into account
    assert!(41.5 < total_value && total_value < 41.53);
}