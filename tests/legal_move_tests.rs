// use chess::board::{Action, Board};

// #[test]
// fn valid_record_end_game() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let result = Board::forsyth_edwards_notation("3r4/1p3pk1/p4b2/P3p2p/1PP1P1b1/4K1p1/1R6/2r5 b - - 0 37");

//     // This is a valid record
//     match result {
//         Ok(board) => {
//             // Should have only 17 pieces left
//             assert_eq!(board.pieces().len(), 17);
//             // Should have only 6 white pieces left
//             assert_eq!(board.white_pieces().len(), 6);
//             // Should have only 11 black pieces left
//             assert_eq!(board.black_pieces().len(), 11);

//             // (1+1+2) =  4 pawn moves
//             //   (3+8) = 11 bishop moves
//             // (10+14) = 24 rook moves
//             //     (6) =  6 king moves
//             //     total 45 moves
//             assert_eq!(board.legal_moves().len(), 45);
//         }
//         Err(_) => {
//             assert!(false);
//         }
//     };
// }

// #[test]
// fn special_moves_allowed() {
//     // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
//     let board = Board::forsyth_edwards_notation("rnbqkbnr/pp2p1pp/8/1p1pPp2/8/5N2/PPPP1PPP/RNBQK2R w KQkq f6 0 5")
//         .expect("This should be a valid record");

//     // (2+2+2+2+2+0+2+2) =  14 pawn moves (one of which is en passant)
//     //               (0) =  0 bishop moves
//     //             (0+2) =   2 rook moves
//     //               (3) =   3 king moves (one of which is short castle)
//     //               (1) =   1 queen move
//     //             (2+4) =   6 knight moves
//     //                total 26 moves
//     assert_eq!(board.legal_moves().len(), 26);

//     // One of the moves is en passant
//     let any_en_passant = board
//         .legal_moves()
//         .into_iter()
//         .any(|chess_move| chess_move.action() == &Action::EnPassant);
//     assert!(any_en_passant);

//     // One of the moves is short castle
//     let any_short_castle = board
//         .legal_moves()
//         .into_iter()
//         .any(|chess_move| chess_move.action() == &Action::ShortCastle);
//     assert!(any_short_castle);
// }

// #[test]
// fn starting_position() {
//     // Starting position returns a Board without it being wrapped in a Result
//     let board = Board::starting_position();

//     // Should start with 32 pieces on the board
//     assert_eq!(board.pieces().len(), 32);

//     // Should start with 16 white pieces
//     assert_eq!(board.white_pieces().len(), 16);

//     // Should start with 16 black pieces
//     assert_eq!(board.black_pieces().len(), 16);

//     // (8 pawns × 2 moves) + (2 knights × 2 moves) = 20 moves
//     assert_eq!(board.legal_moves().len(), 20);
// }
