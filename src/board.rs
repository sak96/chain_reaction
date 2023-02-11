use crate::cell::Cell;
use crate::errors::MoveError;

enum BoardState {
    Wait,
    Explosion(Vec<(usize, usize)>),
    CheckWinCondition,
    GameOver(u8),
}

type BoxBoxCell = Box<[Box<[Cell]>]>;
/// Board structure of game.
pub struct Board{
    cells: BoxBoxCell,
    rows: usize,
    cols: usize,
    cur_player: u8,
    players: u8,
    pub state: BoardState,
}

impl Board {
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
            players,
        }
    }

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
                self.cur_player = (self.cur_player + 1) % self.players;
                self.state = BoardState::Wait
            }
            Ok(())
        }
    }

    pub fn next_iteration(&mut self) {
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
                } else {
                    self.state = BoardState::CheckWinCondition
                }
            }
            BoardState::CheckWinCondition => {
                for rows in self.cells.iter() {
                    for cell in rows.iter() {
                        if let Some(owner) = cell.owner {
                            if owner != self.cur_player {
                                self.cur_player = (self.cur_player + 1) % self.players;
                                self.state = BoardState::Wait;
                                return;
                            }
                        }
                    }
                }
                self.state = BoardState::GameOver(self.cur_player);
            }
            _ => {}
        }
    }
}

