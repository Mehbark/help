#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Turn {
    X,
    O,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Space {
    X,
    O,
    Empty,
}

use std::fmt;
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => 'X',
                Self::O => 'O',
                Self::Empty => '.',
            }
        )
    }
}
impl Space {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_filled(self) -> bool {
        matches!(self, Self::X | Self::O)
    }
}

impl From<Turn> for Space {
    fn from(t: Turn) -> Self {
        match t {
            Turn::X => Self::X,
            Turn::O => Self::O,
            Turn::GameOver => panic!("can't turn a game over into a player"),
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
                Self::GameOver => "end",
            }
        )
    }
}

impl Turn {
    pub fn opposite(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
            Self::GameOver => panic!("Turn::GameOver has no opposite"),
        }
    }
}

impl From<Space> for Turn {
    fn from(s: Space) -> Self {
        match s {
            Space::X => Self::X,
            Space::O => Self::O,
            Space::Empty => panic!("can't turn an empty space into a player"),
        }
    }
}
