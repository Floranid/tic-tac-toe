use utils::{Board, BoardIndex, Player, Winner};
use std::cmp::Ordering;

struct Node {
    path: BoardIndex,
    state: Board,
    score: Option<isize>,
    children: Vec<Node>,
}

impl Node {
    fn new(path: BoardIndex, state: Board) -> Self {
        Self {
            path,
            state,
            score: Some(0),
            children: Vec::new(),
        }
    }

    fn compare_nodes(lhs: &&Self, rhs: &&Self) -> Ordering {
        lhs.score.unwrap().cmp(&rhs.score.unwrap())
    }
    
    fn get_children(&mut self, player: Player) {
        if let Some(winner) = self.state.winner() {
            self.score = Some(match winner {
                Winner::Draw => 0,
                Winner::Player(player) => *player as isize,
            })
        } else {
            for row in 0..3 {
                for col in 0..3 {
                    if self.state.cell_is_empty((col, row)) {
                        let mut child_state = self.state;
                        child_state.set_cell((col, row), player);
                        let mut child = Self::new((col, row), child_state);
                        child.get_children(player.other());
                        self.children.push(child);
                    }
                }
            }

            self.score = {
                let chosen_child = match player {
                        Player::X => self.children.iter().max_by(Self::compare_nodes),
                        Player::O => self.children.iter().min_by(Self::compare_nodes),
                    }.unwrap();
                    chosen_child.score
            }
        }
    }
}

pub struct Tree {
    root: Box<Node>,
}

impl Tree {
    pub fn new(state: Board) -> Self {
        Self {
            root: Box::new(Node::new((0x00, 0x00), state)),
        }
    }

    pub fn min_max(&mut self, player: Player) -> BoardIndex {
        self.root.get_children(player);
        match player {
            Player::X => self.root.children.iter().max_by(Node::compare_nodes),
            Player::O => self.root.children.iter().min_by(Node::compare_nodes),
        }.unwrap().path
    }
}


