//! Provides errors that can occurring while playing chain reaction.
#[derive(Debug, PartialEq, Eq)]
/// Move Errors.
pub enum MoveError {
    /// Move made on other player's cell.
    OtherPlayersCell,
    /// Move made when it is not player's turn.
    NotCurrentPlayerMove,
    /// Move made outside board.
    MoveOutsideBoard,
}
