use crate::errors::MoveError;

#[derive(Default)]
pub struct Cell {
    // TODO: make this smaller than u8 ??
    owner: Option<u8>,
    atoms: u8,
}

impl Cell {
    /// Add checked is used when player add atom to a cell.
    ///
    /// It only allows addition in blank or cell owned by player.
    pub fn add_checked(
        &mut self,
        player: u8,
        row: usize,
        col: usize,
        row_max: usize,
        col_max: usize,
    ) -> Result<Vec<(usize, usize)>, MoveError> {
        if let Some(cell_player) = self.owner {
            if cell_player != player {
                return Err(MoveError::OtherPlayersCell);
            }
        }
        Ok(self.add_unchecked(1, player, row, col, row_max, col_max))
    }

    /// Add unchecked handles explosions.
    /// Explosion can let multiple
    pub fn add_unchecked(
        &mut self,
        new: u8,
        player: u8,
        row: usize,
        col: usize,
        row_max: usize,
        col_max: usize,
    ) -> Vec<(usize, usize)> {
        let mut critical_mass = 4;
        self.owner = Some(player);

        // top down edge
        if row == 0 || row + 1 == row_max {
            critical_mass -= 1;
        }

        // right left edge
        if col == 0 || col + 1 == col_max {
            critical_mass -= 1;
        }
        self.atoms += new;

        if self.atoms < critical_mass {
            vec![]
        } else {
            // explode
            self.atoms -= critical_mass;
            if self.atoms == 0 {
                self.owner = None
            }
            let mut explosion = vec![];
            if row != 0 {
                explosion.push((row - 1, col))
            }
            if row + 1 != row_max {
                explosion.push((row + 1, col))
            }
            if col != 0 {
                explosion.push((row, col - 1))
            }
            if col + 1 != col_max {
                explosion.push((row, col + 1))
            }
            explosion
        }
    }
}
