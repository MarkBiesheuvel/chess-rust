// Hashmap of squares to pieces
#[macro_export]
macro_rules! squares {
    { $( ($f: expr, $r: expr) => ($c: expr, $k: expr)),* $(,)?} => {
        {
            // By default a chess board contains 32 pieces
            let mut map = crate::board::Squares::with_capacity(32);

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

// Iterator that returns a line of squares in a particular direction (x, y)
#[macro_export]
macro_rules! line {
    ( $x:expr, $y:expr ) => {
        (1..).map(|i| crate::board::Offset::new(i * $x, i * $y))
    };
}
