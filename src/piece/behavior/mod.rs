//! Behavior of chess pieces, such as king, bishop, knight, etc.

pub use bishop::Bishop;
pub use king::King;
pub use piece_behavior::PieceBehavior;
pub use queen::Queen;
pub use rook::Rook;

mod bishop;
mod king;
mod piece_behavior;
mod queen;
mod rook;
