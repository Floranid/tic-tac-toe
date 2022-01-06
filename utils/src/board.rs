use super::{Player, Winner};

#[derive(Copy, Clone, Debug, Default)]
pub struct Board {
    cells: [[Option<Player>; 3]; 3],
    moves: u8,
    rows: [isize; 3],
    cols: [isize; 3],
    tlbr: isize,
    trbl: isize,
    winner: Option<Winner>,
}

pub type BoardIndex = (usize, usize);

impl Board {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn cell_is_empty(&self, i: BoardIndex) -> bool {
        self.cells[i.1][i.0].is_none()
    }

    pub fn get_cell(&self, i: BoardIndex) -> &Option<Player> {
        &self.cells[i.1][i.0]
    }

    pub fn set_cell(&mut self, i: BoardIndex, player: Player) -> &Option<Winner> {
        self.moves += 1;

        self.cells[i.1][i.0] = Some(player);

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
        self.winner = if self.rows[i.1] == winner_weight
            || self.cols[i.0] == winner_weight
            || self.tlbr == winner_weight
            || self.trbl == winner_weight
        {
            Some(Winner::Player(player))
        } else if self.moves >= 9 {
            Some(Winner::Draw)
        } else {
            None
        };

        &self.winner
    }

    pub fn winner(&self) -> &Option<Winner> {
        &self.winner
    }
}
