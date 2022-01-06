use utils::{Board, BoardIndex, Player};

pub fn generate_move(board: &mut Board, _player: Player) -> BoardIndex {
    for row in 0..3 {
        for col in 0..3 {
            if board.cell_is_empty((row, col)) {
                return (row, col);
            }
        }
    }
    unreachable!()

}
