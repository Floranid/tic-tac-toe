use super::Player;

#[derive(Copy, Clone, PartialEq)]
pub enum Winner {
    Player(Player),
    Draw,
}
