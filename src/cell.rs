//! Provides cell of chain reaction board.

/// Cell structure of Board.
#[derive(Default, Clone, PartialEq)]
pub(crate) struct Cell {
    pub(crate) owner: Option<u8>,
    pub(crate) atoms: u8,
}

impl Cell {

    pub(crate) fn add_atom(
        &mut self,
        new: u8,
        player: u8,
        row: usize,
        col: usize,
        row_max: usize,
        col_max: usize,
    ) -> bool {
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
            false
        } else {
            // explode
            self.atoms -= critical_mass;
            if self.atoms == 0 {
                self.owner = None
            }
            true
        }
    }

    pub(crate) fn get_neighbors(
        row: usize,
        col: usize,
        row_max: usize,
        col_max: usize,
    ) -> Vec<(usize, usize)> {
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
