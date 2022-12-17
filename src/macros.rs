// Hashmap of squares to pieces
#[macro_export]
macro_rules! piece_placement {
    { $( ($f: expr, $r: expr) => ($c: expr, $k: expr)),* $(,)?} => {
        {
            // A chess board contains a maximum of 32 pieces
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
