mod board;
mod player;
mod winner;

pub use board::{Board, Point2D};
pub use player::Player;
pub use winner::Winner;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
