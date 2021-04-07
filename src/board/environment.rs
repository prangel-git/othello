use crate::{Action, Environment};

use super::*;

/// Implements environment for Othello
impl Environment<Action, AgentId> for Board {
    type ActionIter = ActionIter;

    fn initial_state() -> Self {
        todo!()
    }

    fn update(&mut self, a: &Action) -> bool {
        self.place_tile(a)
    }

    fn what_if(&self, a: &Action) -> Self {
        let mut board = self.clone();
        board.update(a);
        return board;
    }

    fn valid_actions(&self) -> Self::ActionIter {
        ActionIter::new(self)
    }

    fn is_valid(&self, action: &Action) -> bool {
        self.valid.contains(action)
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    fn turn(&self) -> AgentId {
        self.turn
    }

    fn winner(&self) -> Option<AgentId> {
        todo!()
    }
}
