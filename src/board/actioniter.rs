use crate::{Action, Board};

/// Iterator for possible actions
pub struct ActionIter {
    valid: Vec<Action>,
}

/// Implements ActionIter
impl ActionIter {
    pub fn new(board: &Board) -> Self {
        let mut valid = Vec::new();

        for action in &board.valid {
            valid.push(*action);
        }

        ActionIter { valid }
    }
}

/// Implements iterator
impl Iterator for ActionIter {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.valid.pop()
    }
}
