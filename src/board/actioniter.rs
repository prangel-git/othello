use crate::{Action, Board};

/// Iterator for possible actions
pub struct ActionIter {}

/// Implements ActionIter
impl ActionIter {
    pub fn new(board: &Board) -> Self {
        ActionIter {}
    }
}

/// Implements iterator
impl Iterator for ActionIter {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
