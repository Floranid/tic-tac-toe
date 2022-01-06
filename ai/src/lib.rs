mod tree;

use utils::{
    Board,
    BoardIndex,
    Player
};
use tree::Tree;

pub fn generate_move(board: Board, player: Player) -> BoardIndex {
    let mut tree = Tree::new(board);
    tree.min_max(player)
}
