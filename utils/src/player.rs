#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X = 1,
    O = -1,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}
