//! Provides board for playing chain reaction.

use crate::cell::Cell;
use crate::errors::MoveError;
use std::fmt::Display;
/// Board can be in any one of the states
#[derive(PartialEq)]
pub enum BoardState {
    /// When board is waiting for player input
    Wait,
    /// When board has explosion. The exploded location are stored.
    Explosion(Vec<(usize, usize)>),
    /// When board is checking if there is winner
    CheckWinCondition,
    /// When board has winner. The Player id is stored.
    GameOver(u8),
}

/// Cell State
#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    /// Cell gained a new atom.
    Explosion(u8),
    /// Cell is non empty. Stores owner id and atoms in it.
    NonEmpty(u8, u8),
    /// Cell is Empty.
    Empty,
}

type BoxBoxCell = Box<[Box<[Cell]>]>;
/// Board structure of game.
#[derive(PartialEq)]
pub struct Board{
    cells: BoxBoxCell,
    rows: usize,
    cols: usize,
    cur_player: u8,
    players: Box<[bool]>,
    state: BoardState,
}

impl Board {
    /// Provides 2d vector of [`CellState`].
    /// [`CellState`]: self::CellState
    ///
    /// # Examples
    /// When game starts the board is empty.
    ///
    /// ```
    /// use chain_reaction::board::Board;
    /// use chain_reaction::board::CellState;
    ///
    /// let board = Board::new(4, 4, 2);
    /// assert_eq!(board.cells(), vec![vec![CellState::Empty;4];4]);
    /// ```
    pub fn cells(&self) -> Vec<Vec<CellState>> {
        let exploision: Vec<(usize, usize)> = if let BoardState::Explosion(ref explosion) = self.state {
            explosion.to_vec()
        } else {
            vec![]
        };
        self.cells.iter().enumerate().map(
            |(r,row)| row.iter().enumerate().map(
                |(c, cell)|
                if exploision.contains(&(r, c)) {
                    CellState::Explosion(cell.atoms)
                } else if let Some(owner_id) = cell.owner {
                    CellState::NonEmpty(owner_id, cell.atoms)
                } else {
                    CellState::Empty
                }
            ).collect()
        ).collect()
    }

    /// Provides current player id
    ///
    /// # Examples
    /// The first move is by player 0.
    ///
    /// ```
    /// use chain_reaction::board::Board;
    /// let board = Board::new(4, 4, 2);
    /// assert_eq!(board.current_player_id(), 0);
    /// ```
    pub fn current_player_id(&self) -> u8 {
        self.cur_player
    }

    /// Provides current Board state
    pub fn state(&self) -> &BoardState {
        &self.state
    }

    /// Create new Board
    ///
    /// # Arguments
    ///
    /// * `rows` - Rows required in board.
    /// * `cols` - Columns in board.
    /// * `players` - Number of players.
    ///
    /// # Examples
    /// Creating a Board of 4x4 for 2 player can be done as follows.
    /// ```
    /// use chain_reaction::board::Board;
    /// let _board = Board::new(4, 4, 2);
    /// ```
    ///
    /// Board should have minimum of 3 rows and columns.
    /// ```should_panic
    /// use chain_reaction::board::Board;
    /// let _ = Board::new(2, 4, 2);
    /// ```
    ///
    /// Board should have minimum of 2 player.
    /// ```should_panic
    /// use chain_reaction::board::Board;
    /// let _ = Board::new(2, 4, 1);
    /// ```
    ///
    pub fn new(rows: usize, cols: usize, players: u8) -> Self {
        if rows < 3 || cols < 3 {
            panic!("rows and columns should be greater than 3");
        }
        if players < 2 {
            panic!("there should be minimum of 2 players ");
        }
        let cells = vec![vec![Default::default(); cols].into_boxed_slice(); rows].into_boxed_slice();
        Self {
            state: BoardState::Wait,
            cur_player: 0,
            rows,
            cols,
            cells,
            players: vec![true; players.into()].into_boxed_slice(),
        }
    }

    /// Allow player to make a move on board.
    ///
    /// # Arguments
    ///
    /// * `player` - The player making move.
    /// * `row` - The row where move is made.
    /// * `col` - The column where move is made.
    ///
    /// # Errors
    ///
    /// Will return [`MoveError`] if move is invalid.
    ///
    /// [`MoveError`]: crate::error::MoveError
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_reaction::board::Board;
    /// use chain_reaction::errors::MoveError;
    ///
    /// let mut board = Board::new(4, 4, 2);
    ///
    /// // First move should be by player 0.
    /// // But player 1 is making move, Hence the error.
    ///
    /// // Player 0 make valid move.
    /// assert_eq!(board.player_move(0, 0, 0), Ok(()));
    ///
    /// // It is player 1 turn. But player 0 makes move, hence the error.
    /// assert_eq!(board.player_move(0, 0, 0), Err(MoveError::NotCurrentPlayerMove));
    ///
    /// // Player 1 make move on player 0's cell, hence the error.
    /// assert_eq!(board.player_move(1, 0, 0), Err(MoveError::OtherPlayersCell));
    ///
    /// // Player 1 make move outside board, hence the error.
    /// assert_eq!(board.player_move(1, 0, 0), Err(MoveError::OtherPlayersCell));
    /// ```
    pub fn player_move(&mut self, player: u8, row: usize, col: usize) -> Result<(), MoveError> {
        if !matches!(self.state, BoardState::Wait) || self.cur_player != player {
            Err(MoveError::NotCurrentPlayerMove)
        } else if row >= self.rows || col >= self.cols {
            Err(MoveError::MoveOutsideBoard)
        } else {
            let explosion = self
                .cells
                .get_mut(row)
                .ok_or(MoveError::MoveOutsideBoard)?
                .get_mut(col)
                .ok_or(MoveError::MoveOutsideBoard)?
                .add_checked(self.cur_player, row, col, self.rows, self.cols)?;
            if !explosion.is_empty() {
                self.state = BoardState::Explosion(explosion)
            } else {
                self.next_player()
            }
            Ok(())
        }
    }

    fn next_player(&mut self) {
        let mut i = self.cur_player as usize;
        let player_count = self.players.len();
        loop  {
            i = (i + 1) % player_count;
            if self.players[i] {
                self.cur_player = i as u8;
                break
            }
        }
        self.state = BoardState::Wait
    }

    /// Runs next iterations of explosion.
    ///
    /// Returns `true` if there is next iteration.
    pub fn next_iteration(&mut self) -> bool {
        match self.state {
            BoardState::Explosion(ref mut explosion) => {
                let mut exploded_cells = vec![];
                for (row, col) in explosion.drain(..) {
                    exploded_cells.append(
                        &mut self
                            .cells
                            .get_mut(row)
                            .unwrap()
                            .get_mut(col)
                            .unwrap()
                            .add_unchecked(1, self.cur_player, row, col, self.rows, self.cols),
                    )
                }
                if !exploded_cells.is_empty() {
                    self.state = BoardState::Explosion(exploded_cells);
                    true
                } else {
                    self.state = BoardState::CheckWinCondition;
                    true
                }
            }
            BoardState::CheckWinCondition => {
                self.players.iter_mut().for_each(|i| *i = false);
                for rows in self.cells.iter() {
                    for cell in rows.iter() {
                        if let Some(owner) = cell.owner {
                            self.players[owner as usize] = true
                        }
                    }
                }
                if self.players.iter().filter(|x| **x).count() == 1 {
                    self.state = BoardState::GameOver(self.cur_player);
                } else {
                    self.next_player();
                }
                false
            }
            _ => {
                false
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rows in self.cells() {
            for cell in rows {
                match cell {
                    CellState::Explosion(atoms) => write!(f, "|+{}|", atoms)?,
                    CellState::NonEmpty(owner_id,atoms) =>  write!(f, "|{}{}|", owner_id, atoms)?,
                    CellState::Empty => write!(f, "|  |")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_cannot_play_if_all_cells_are_lost() {
        let moves = [
            (0, 0, 0),
            (1, 0, 1),
            (2, 0, 2),
            (0, 0, 0),
        ];
        let mut b = Board::new(4, 4, 3);
        for (player, r, c) in moves {
            assert!(b.player_move(player, r, c).is_ok());
        }

        // player 1 lost all cells.
        let player_lost_all_cells = 1;
        for rows in b.cells() {
            for cell in rows {
                if let CellState::NonEmpty(owner, _) = cell {
                    assert_ne!(owner, player_lost_all_cells);
                }
            }
        }

        // the player cannot play.
        assert_ne!(b.current_player_id(), player_lost_all_cells);

    }
}

