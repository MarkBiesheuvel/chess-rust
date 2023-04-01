// use chess::board::Board;
// use chess::parser::ParseError;

// #[test]
// fn invalid_piece_error() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBXKBNR w KQkq - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains an X in the piece placement field, which is invalid
//     assert_eq!(error, ParseError::InvalidPiece('X'));
// }

// #[test]
// fn missing_white_pawn() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP/RNBQKBNR w KQkq - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains is missing a white pawn
//     assert_eq!(error, ParseError::IncompletePiecePlacement);
// }

// #[test]
// fn missing_empty_square() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains is missing an empty square
//     assert_eq!(error, ParseError::IncompletePiecePlacement);
// }

// #[test]
// fn missing_rank() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains is missing an entire rank
//     assert_eq!(error, ParseError::IncompletePiecePlacement);
// }

// #[test]
// fn missing_white_knight() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RBQKBNR w KQkq - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains is missing a white knight
//     assert_eq!(error, ParseError::IncompletePiecePlacement);
// }

// #[test]
// fn invalid_active_color() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains an x in the active color field, which is invalid
//     assert_eq!(error, ParseError::InvalidColor('x'));
// }

// #[test]
// fn invalid_castling_availability() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains an x in the castling availability field, which is invalid
//     assert_eq!(error, ParseError::InvalidCastling('x'));
// }

// #[test]
// fn invalid_en_passant_target_file() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq x3 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains an x in the en passant target square field, which is invalid
//     assert_eq!(error, ParseError::InvalidFile('x'));
// }

// #[test]
// fn invalid_en_passant_target_rank() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq b9 0 1")
//         .expect_err("This should be an invalid record");

//     // The record contains an x in the en passant target square field, which is invalid
//     assert_eq!(error, ParseError::InvalidRank('9'));
// }

// #[test]
// fn unexpected_end() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let error = Board::forsyth_edwards_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq")
//         .expect_err("This should be an invalid record");

//     // The record is missing the last fields
//     assert_eq!(error, ParseError::UnexpectedEnd);
// }

// #[test]
// fn valid_record_no_captures() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let board = Board::forsyth_edwards_notation("rnbqkb1r/ppp1pppp/8/3pP3/6n1/8/PPPPKPPP/RNBQ1BNR w kq d6 0 4")
//         .expect("This should be a valid record");

//     // Should still have 32 pieces on the board
//     assert_eq!(board.pieces().len(), 32);

//     // Should still have 16 white pieces
//     assert_eq!(board.white_pieces().len(), 16);

//     // Should still have 16 black pieces
//     assert_eq!(board.black_pieces().len(), 16);
// }
