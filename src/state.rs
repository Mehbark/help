use crate::space::{Space, Turn};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    board: [Space; 9],
    to_go: Turn,
}

impl State {
    pub fn new(board: [Space; 9], to_go: Turn) -> Self {
        Self { board, to_go }
    }
    pub fn line_wins(&self, line: (usize, usize, usize)) -> bool {
        let (a, b, c) = (self.board[line.0], self.board[line.1], self.board[line.2]);
        a.is_filled() && b.is_filled() && c.is_filled() && a == b && b == c
    }

    // Grid:
    // 0 1 2
    // 3 4 5
    // 6 7 8
    pub fn next_turn(&self) -> Turn {
        let lines = [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];
        if self.board.iter().all(|s| s.is_filled())
            || lines.iter().any(|line| self.line_wins(*line))
        {
            Turn::GameOver
        } else {
            self.to_go.opposite()
        }
    }

    pub fn empty_spaces(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, s)| if s.is_empty() { Some(i) } else { None })
            .collect()
    }

    pub fn successors(&self) -> Vec<(usize, Self)> {
        if self.is_over() {
            Vec::new()
        } else {
            self.empty_spaces()
                .iter()
                .map(|i| {
                    let mut new = *self;
                    new.board[*i] = self.to_go.into();
                    new.to_go = new.next_turn();
                    (*i, new)
                })
                .collect()
        }
    }

    pub fn is_over(&self) -> bool {
        self.to_go == Turn::GameOver
    }
}

use std::fmt;
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars = &self
            .board
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()[..9];

        // "{}\n {} │ {} │ {} \n───┼───┼───\n {} │ {} │ {} \n───┼───┼───\n {} │ {} │ {} ",
        write!(
            f,
            "{}\n{}{}{}\n{}{}{}\n{}{}{}",
            self.to_go,
            chars[0],
            chars[1],
            chars[2],
            chars[3],
            chars[4],
            chars[5],
            chars[6],
            chars[7],
            chars[8]
        )
    }
}
