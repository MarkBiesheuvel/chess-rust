/// Enum to indicate whether a square is empty, or take by the same color or by the opposite color
pub enum SquareStatus {
    /// The square is empty
    Empty,
    /// The square is taken by a piece of the same color
    TakenBySame,
    /// The square is taken by a piece of the opposite color
    TakenByOpposite,
}
