/// The status of a move
#[derive(Debug)]
pub enum Status {
    /// Opponent is now checkmated
    Checkmate,
    /// Opponent is now in check
    Check,
    /// Nothing special happened
    Nothing,
}
