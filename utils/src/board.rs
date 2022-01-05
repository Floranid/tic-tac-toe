use super::{Player, Winner};

#[derive(Copy, Clone, Debug, Default)]
pub struct Board {
    cells: [[Option<Player>; 3]; 3],
    moves: u8,
    rows: [isize; 3],
    cols: [isize; 3],
    tlbr: isize,
    trbl: isize,
}

pub type Point2D = (usize, usize);

impl Board {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn cell_is_empty(&self, i: Point2D) -> bool {
        self.cells[i.1][i.0].is_none()
    }

    pub fn get_cell(&self, i: Point2D) -> &Option<Player> {
        &self.cells[i.1][i.0]
    }

    pub fn set_cell(&mut self, i: Point2D, player: Player) -> Option<Winner> {
        if self.moves == 9 {
            return Some(Winner::Draw);
        }

        self.cells[i.1][i.0] = Some(player);
        self.moves += 1;

        let weight = player as isize;

        self.rows[i.1] += weight;
        self.cols[i.0] += weight;

        if i.0 == i.1 {
            self.tlbr += weight;
        }

        if i.1 + i.0 == 2 {
            self.trbl += weight;
        }

        let winner_weight = 3 * weight;

        if self.rows[i.1] == winner_weight
            || self.cols[i.0] == winner_weight
            || self.tlbr == winner_weight
            || self.trbl == winner_weight
        {
            Some(Winner::Player(player))
        } else {
            None
        }
    }
}
