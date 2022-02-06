use crate::cell::Cell;
use crate::errors::MoveError;
use std::iter::repeat_with;

enum BoardState {
    Wait,
    Explosion(Vec<(usize, usize)>),
    CheckWinCondition,
}

struct Board {
    // TODO: use const generics,
    cells: Vec<Vec<Cell>>,
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
        let cells = repeat_with(|| repeat_with(|| Default::default()).take(cols).collect())
            .take(rows)
            .collect();
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
        if !self.next_iteration_exists() || self.cur_player != player {
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

    pub fn next_iteration_exists(&self) -> bool {
        !matches!(self.state, BoardState::Wait)
    }

    pub fn next_iteration(&self) {}
}
