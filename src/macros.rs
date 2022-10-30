// Hashmap of squares to pieces
#[macro_export]
macro_rules! piece_placement {
    { $( ($f: expr, $r: expr) => ($c: expr, $k: expr)),* $(,)?} => {
        {
            // By default a chess board contains 32 pieces
            let mut map = crate::board::PiecePlacement::with_capacity(32);

            // Add each expression to the map
            $(
                let square = crate::board::Square::new($f, $r);
                let piece = crate::piece::Piece::new($c, $k);
                map.insert(square, piece);
            )*

            map
        }
    };
}

#[macro_export]
macro_rules! add_move {
    {$list: ident <- ($piece: ident, $origin: ident, $action: expr, $destination: expr)} => {
        let chess_move = crate::board::ChessMove::new($piece.clone(), $origin.clone(), $action, $destination);
        $list.push(chess_move);
    };
}
