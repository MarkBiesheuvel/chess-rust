use crate::piece::behavior::PieceBehavior;

/// The type of action of a chess move. For example a capture, promotion or castle
#[derive(Debug)]
pub enum Action {
    /// Normal move
    Normal,
    /// Capture
    Capture,
    /// En-passant capture
    EnPassant,
    /// Normal promotion
    MovePromotion(Box<dyn PieceBehavior>),
    /// Capture promotion
    CapturePromotion(Box<dyn PieceBehavior>),
    /// Short castle
    ShortCastle,
    /// Long castle
    LongCastle,
}
