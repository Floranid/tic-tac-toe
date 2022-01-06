use super::Player;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Winner {
    Player(Player),
    Draw,
}
